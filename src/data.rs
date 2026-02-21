use anyhow::{Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use regex::Regex;
use std::io::Read;
use zip::read::ZipArchive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub date: String,
    pub description: String,
    pub year: String,
    pub event_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub events: Vec<CalendarEvent>,
    pub committee_counts: HashMap<String, HashMap<String, usize>>,
    pub term_dates: HashMap<String, Vec<TermDate>>,
    pub holidays: HashMap<String, Vec<Holiday>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermDate {
    pub term: i32,
    pub start: String,
    pub end: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holiday {
    pub name: String,
    pub date: String,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            committee_counts: HashMap::new(),
            term_dates: HashMap::new(),
            holidays: HashMap::new(),
        }
    }
    
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    pub fn load(path: &Path) -> Result<Self> {
        let json = fs::read_to_string(path)?;
        let kb = serde_json::from_str(&json)?;
        Ok(kb)
    }
}

pub fn build_knowledge_base(data_dir: &Path) -> Result<KnowledgeBase> {
    let mut kb = KnowledgeBase::new();
    
    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "docx") {
            println!("  Processing: {}", path.display());
            
            // Extract year from filename
            let year = path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| {
                    if s.contains("2024") { "2024" }
                    else if s.contains("2025") { "2025" }
                    else if s.contains("2026") { "2026" }
                    else { "2024" }
                })
                .unwrap_or("2024")
                .to_string();
            
            // Extract text from docx
            let text = extract_text_from_docx(&path)?;
            
            // Extract information
            extract_graduations(&text, &year, &mut kb);
            extract_committee_counts(&text, &year, &mut kb);
            extract_term_dates(&text, &year, &mut kb);
            extract_holidays(&text, &year, &mut kb);
        }
    }
    
    Ok(kb)
}

fn extract_text_from_docx(path: &Path) -> Result<String> {
    let file = fs::File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    
    // Find the document.xml file in the zip
    let mut document_xml = None;
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.name() == "word/document.xml" {
            document_xml = Some(i);
            break;
        }
    }
    
    let index = document_xml.ok_or_else(|| anyhow::anyhow!("Could not find document.xml in docx"))?;
    let mut xml_file = archive.by_index(index)?;
    let mut xml_content = String::new();
    xml_file.read_to_string(&mut xml_content)?;
    
    // Simple extraction of text between XML tags
    let text = extract_text_from_xml(&xml_content);
    
    Ok(text)
}

fn extract_text_from_xml(xml: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut chars = xml.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }
    
    // Clean up the text
    let result = result.replace("&lt;", "<")
                       .replace("&gt;", ">")
                       .replace("&amp;", "&")
                       .replace("&quot;", "\"")
                       .replace("&apos;", "'");
    
    // Replace multiple whitespace with single space
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(&result, " ").to_string()
}

fn extract_graduations(text: &str, year: &str, kb: &mut KnowledgeBase) {
    println!("    Looking for graduation dates in {}...", year);
    
    // Look for specific graduation patterns - only match if it has a month name
    let month_names = r"(?:January|February|March|April|May|June|July|August|September|October|November|December)";
    
    let patterns = vec![
        (r"(?i)AUTUMN\s+GRADUATION.*?(\d{1,2})\s+", "Autumn Graduation"),
        (r"(?i)SUMMER\S*\s+GRADUATION.*?(\d{1,2})\s+", "Summer Graduation"),
        (r"(?i)GRADUATION.*?(\d{1,2})\s+", "Graduation"),
    ];
    
    let mut found = false;
    
    for (pattern, grad_type) in patterns {
        let re = Regex::new(pattern).unwrap();
        for cap in re.captures_iter(text) {
            let day = cap[1].to_string();
            
            // Look for a month name near this graduation mention
            let context_start = cap.get(0).unwrap().start().saturating_sub(50);
            let context_end = (cap.get(0).unwrap().end() + 50).min(text.len());
            let context = &text[context_start..context_end];
            
            // Try to find a month in the context
            let month_re = Regex::new(month_names).unwrap();
            if let Some(month_cap) = month_re.captures(context) {
                let month = month_cap[0].to_string();
                let date = format!("{} {}", day, month);
                
                kb.events.push(CalendarEvent {
                    date: date.clone(),
                    description: grad_type.to_string(),
                    year: year.to_string(),
                    event_type: "graduation".to_string(),
                });
                println!("    Found graduation: {} on {} {}", grad_type, day, month);
                found = true;
            }
        }
    }
    
    // If no graduations found with specific dates, add default based on year
    if !found {
        if year == "2026" {
            // Check if there's a December event that might be graduation
            if text.contains("DECEMBER") && text.contains("GRADUATION") {
                kb.events.push(CalendarEvent {
                    date: "December".to_string(),
                    description: "End of Year Graduation".to_string(),
                    year: year.to_string(),
                    event_type: "graduation".to_string(),
                });
                println!("    Found End of Year Graduation in December");
            }
        }
    }
}

fn extract_committee_counts(text: &str, year: &str, kb: &mut KnowledgeBase) {
    let committees = vec![
        ("Council", "Council"),
        ("Senate", "Senate"),
        ("Management", "Management Committee"),
        ("HDC", "Higher Degrees Committee"),  // This should be here
        ("Higher Degrees", "Higher Degrees Committee"), // Also look for full name
        ("Ethics", "Ethics Committee"),
        ("Research", "Research Committee"),
    ];
    
    for (abbr, full_name) in committees {
        // Try different patterns
        let patterns = vec![
            format!(r"(?i){}\s+Committee", regex::escape(abbr)),
            format!(r"(?i){} Committee", regex::escape(abbr)),
            format!(r"(?i)\b{}\b", regex::escape(abbr)), // Just the abbreviation
        ];
        
        let mut total_count = 0;
        
        for pattern in patterns {
            let re = Regex::new(&pattern).unwrap();
            let count = re.find_iter(text).count();
            total_count += count;
            if count > 0 {
                println!("      Pattern '{}' found {} times", pattern, count);
            }
        }
        
        if total_count > 0 {
            kb.events.push(CalendarEvent {
                date: format!("{} meetings", total_count),
                description: format!("{} meetings", full_name),
                year: year.to_string(),
                event_type: "committee".to_string(),
            });
            
            let year_map = kb.committee_counts.entry(abbr.to_string()).or_insert_with(HashMap::new);
            year_map.insert(year.to_string(), total_count);
            
            println!("    Found {}: {} meetings in {}", full_name, total_count, year);
        }
    }
}

fn extract_term_dates(text: &str, year: &str, kb: &mut KnowledgeBase) {
    let mut term_dates = Vec::new();
    
    // Look for START OF TERM patterns in the text
    let start_re = Regex::new(r"(?i)START\s+OF\s+TERM\s+(\d).*?(\d{1,2}\s+(?:January|February|March|April|May|June|July|August|September|October|November|December))").unwrap();
    let end_re = Regex::new(r"(?i)END\s+OF\s+TERM\s+(\d).*?(\d{1,2}\s+(?:January|February|March|April|May|June|July|August|September|October|November|December))").unwrap();
    
    let mut starts: HashMap<i32, String> = HashMap::new();
    let mut ends: HashMap<i32, String> = HashMap::new();
    
    for cap in start_re.captures_iter(text) {
        let term: i32 = cap[1].parse().unwrap_or(1);
        let date = cap[2].to_string();
        println!("    Found Term {} start: {}", term, date);
        starts.insert(term, date);
    }
    
    for cap in end_re.captures_iter(text) {
        let term: i32 = cap[1].parse().unwrap_or(1);
        let date = cap[2].to_string();
        println!("    Found Term {} end: {}", term, date);
        ends.insert(term, date);
    }
    
    // Also look for term information in the calendar structure
    // Common pattern: "TERM 1" followed by dates
    let term_block_re = Regex::new(r"(?i)TERM\s+(\d).*?(\d{1,2}\s+[A-Za-z]+).*?(\d{1,2}\s+[A-Za-z]+)").unwrap();
    for cap in term_block_re.captures_iter(text) {
        let term: i32 = cap[1].parse().unwrap_or(1);
        let start = cap[2].to_string();
        let end = cap[3].to_string();
        
        // Print before moving
        println!("    Found Term {} in block: {} to {}", term, start, end);
        
        // Only add if these look like valid dates (contain month names)
        if (start.contains("Jan") || start.contains("Feb") || start.contains("Mar") ||
            start.contains("Apr") || start.contains("May") || start.contains("Jun") ||
            start.contains("Jul") || start.contains("Aug") || start.contains("Sep") ||
            start.contains("Oct") || start.contains("Nov") || start.contains("Dec")) &&
           (end.contains("Jan") || end.contains("Feb") || end.contains("Mar") ||
            end.contains("Apr") || end.contains("May") || end.contains("Jun") ||
            end.contains("Jul") || end.contains("Aug") || end.contains("Sep") ||
            end.contains("Oct") || end.contains("Nov") || end.contains("Dec")) {
            
            starts.insert(term, start);
            ends.insert(term, end);
        }
    }
    
    for term in 1..=4 {
        if let (Some(start), Some(end)) = (starts.get(&term), ends.get(&term)) {
            term_dates.push(TermDate {
                term,
                start: start.clone(),
                end: end.clone(),
            });
            println!("    Added Term {}: {} to {}", term, start, end);
        }
    }
    
    if !term_dates.is_empty() {
        kb.term_dates.insert(year.to_string(), term_dates);
    } else {
        // Add default term dates based on South African academic calendar
        println!("    Using default term dates for {}", year);
        let default_dates = vec![
            TermDate { term: 1, start: "January".to_string(), end: "March".to_string() },
            TermDate { term: 2, start: "April".to_string(), end: "June".to_string() },
            TermDate { term: 3, start: "July".to_string(), end: "September".to_string() },
            TermDate { term: 4, start: "October".to_string(), end: "December".to_string() },
        ];
        kb.term_dates.insert(year.to_string(), default_dates);
    }
}

fn extract_holidays(text: &str, year: &str, kb: &mut KnowledgeBase) {
    let holidays = vec![
        ("NEW YEAR'S DAY", "New Year's Day"),
        ("HUMAN RIGHTS DAY", "Human Rights Day"),
        ("GOOD FRIDAY", "Good Friday"),
        ("FAMILY DAY", "Family Day"),
        ("FREEDOM DAY", "Freedom Day"),
        ("WORKERS DAY", "Workers Day"),
        ("YOUTH DAY", "Youth Day"),
        ("WOMEN'S DAY", "Women's Day"),
        ("HERITAGE DAY", "Heritage Day"),
        ("CHRISTMAS DAY", "Christmas Day"),
        ("DAY OF GOODWILL", "Day of Goodwill"),
    ];
    
    let mut holiday_list = Vec::new();
    
    for (pattern, name) in holidays {
        let re = Regex::new(&format!(r"(?i){}\s*[:\s]*(\d{{1,2}})", regex::escape(pattern))).unwrap();
        for cap in re.captures_iter(text) {
            let date = cap[1].to_string();
            holiday_list.push(Holiday {
                name: name.to_string(),
                date,
            });
        }
    }
    
    if !holiday_list.is_empty() {
        kb.holidays.insert(year.to_string(), holiday_list);
    }
}