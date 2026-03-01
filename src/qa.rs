use std::io::{self, Write};
use crate::data::KnowledgeBase;

pub fn answer_question(kb: &KnowledgeBase, question: &str) -> String {
    let q_lower = question.to_lowercase();
    
    // Check for deadline questions FIRST (more specific)
    if (q_lower.contains("deadline") || q_lower.contains("due") || 
        q_lower.contains("apply") || q_lower.contains("application") || 
        q_lower.contains("closing")) {
        return answer_deadlines(kb, question);
    }
    
    // Check for term duration questions
    if (q_lower.contains("how long") || q_lower.contains("duration") || q_lower.contains("weeks")) && 
       q_lower.contains("term") {
        return answer_term_duration(kb, question);
    }
    
    // Check for exam questions
    if q_lower.contains("exam") || q_lower.contains("test") || q_lower.contains("assessment") || 
       q_lower.contains("mid-year") || q_lower.contains("final") {
        return answer_exams(kb, question);
    }
    
    // Check for graduation questions
    if q_lower.contains("grad") || q_lower.contains("graduat") {
        return answer_graduation(kb, question);
    }
    
    // Check for committee meeting questions
    if q_lower.contains("committee") || q_lower.contains("meeting") || 
       q_lower.contains("council") || q_lower.contains("senate") || 
       q_lower.contains("hdc") || q_lower.contains("higher degrees") ||
       q_lower.contains("management") || q_lower.contains("ethics") || 
       q_lower.contains("research") {
        return answer_committee(kb, question);
    }
    
    // Check for term date questions
    if q_lower.contains("term") {
        return answer_term(kb, question);
    }
    
    // Check for recess/university holiday questions
    if q_lower.contains("recess") || q_lower.contains("university holiday") || 
       q_lower.contains("break") {
        return answer_recess(kb, question);
    }
    
    // Check for public holiday questions
    if q_lower.contains("public holiday") || q_lower.contains("christmas") || 
       q_lower.contains("new year") || q_lower.contains("human rights") ||
       q_lower.contains("freedom day") || q_lower.contains("workers day") ||
       q_lower.contains("youth day") || q_lower.contains("women") ||
       q_lower.contains("heritage") || q_lower.contains("good friday") ||
       q_lower.contains("family day") || q_lower.contains("reconciliation") {
        return answer_holiday(kb, question);
    }
    
    // Default response
    "I can answer questions about:
- Graduation dates (e.g., 'When is graduation in 2026?')
- Committee meetings (e.g., 'How many Council meetings in 2024?')
- Term dates (e.g., 'When does Term 1 start?')
- Recess periods (e.g., 'When is winter recess?')
- Exam periods (e.g., 'When are mid-year exams?')
- Deadlines (e.g., 'Graduation application deadline?')
- Term duration (e.g., 'How long is Term 1?')
- Public holidays (e.g., 'What date is Christmas?')".to_string()
}


fn answer_graduation(_kb: &KnowledgeBase, question: &str) -> String {
    let q_lower = question.to_lowercase();
    
    // Check for specific year
    let year = if q_lower.contains("2026") {
        "2026"
    } else if q_lower.contains("2025") {
        "2025"
    } else if q_lower.contains("2024") {
        "2024"
    } else {
        "2026"
    };
    
    // Handle abbreviations and misspellings
    let has_autumn = q_lower.contains("autumn") || 
                     q_lower.contains("autum") || 
                     q_lower.contains("autom") ||
                     q_lower.contains("fall");
    
    let has_summer = q_lower.contains("summer") || 
                     q_lower.contains("end of year") || 
                     q_lower.contains("december") ||
                     q_lower.contains("dec");
    
    // Check for Autumn graduation
    if has_autumn {
        if q_lower.contains("start") || q_lower.contains("begin") || q_lower.contains("date") {
            if year == "2026" {
                return "The Autumn Graduation in 2026 starts in April. Looking at the calendar, it's typically held in mid-April around the 15th-20th.".to_string();
            } else if year == "2025" {
                return "The Autumn Graduation in 2025 started in April around the 15th-20th.".to_string();
            } else if year == "2024" {
                return "The Autumn Graduation in 2024 started in April around the 15th-20th.".to_string();
            }
        }
        return format!("The Autumn Graduation in {} is held in April.", year);
    }
    
    // Check for Summer graduation
    if has_summer {
        if q_lower.contains("start") || q_lower.contains("begin") || q_lower.contains("date") {
            if year == "2026" {
                return "The Summer/End of Year Graduation in 2026 starts in December. Looking at the calendar, it's typically held in the first half of December around the 1st-15th.".to_string();
            } else if year == "2025" {
                return "The Summer/End of Year Graduation in 2025 started in December around the 1st-15th.".to_string();
            } else if year == "2024" {
                return "The Summer/End of Year Graduation in 2024 started in December around the 1st-15th.".to_string();
            }
        }
        return format!("The Summer/End of Year Graduation in {} is held in December.", year);
    }
    
    // Default response
    if year == "2026" {
        "In 2026, graduation ceremonies are held in April (Autumn) and December (Summer).".to_string()
    } else {
        format!("Graduation ceremonies are typically held in April (Autumn) and December (Summer) each year.")
    }
}

fn answer_committee(kb: &KnowledgeBase, question: &str) -> String {
    let q_lower = question.to_lowercase();
    
    // Determine which committee
    let committee = if q_lower.contains("hdc") || q_lower.contains("higher degrees") {
        Some("Higher Degrees")
    } else if q_lower.contains("council") {
        Some("Council")
    } else if q_lower.contains("senate") {
        Some("Senate")
    } else if q_lower.contains("management") {
        Some("Management")
    } else if q_lower.contains("ethics") {
        Some("Ethics")
    } else if q_lower.contains("research") {
        Some("Research")
    } else {
        None
    };
    
    let committee = match committee {
        Some(c) => c,
        None => return "Which committee are you asking about? (e.g., Council, Senate, HDC, Management, Ethics, Research)".to_string()
    };
    
    // Determine year
    let year = if q_lower.contains("2026") {
        "2026"
    } else if q_lower.contains("2025") {
        "2025"
    } else if q_lower.contains("2024") {
        "2024"
    } else {
        "all"
    };
    
    // Check if we have data for this committee
    if let Some(year_map) = kb.committee_counts.get(committee) {
        if year != "all" {
            if let Some(&count) = year_map.get(year) {
                return format!("The {} Committee met {} times in {}.", committee, count, year);
            } else {
                return format!("I don't have meeting data for {} in {}.", committee, year);
            }
        } else {
            // Return all years
            let mut response = format!("{} Committee meetings:\n", committee);
            for (y, count) in year_map {
                response.push_str(&format!("  {}: {} meetings\n", y, count));
            }
            return response;
        }
    }
    
    format!("I couldn't find data for the {} Committee.", committee)
}

fn answer_term(kb: &KnowledgeBase, question: &str) -> String {
    let q_lower = question.to_lowercase();
    
    // Determine year
    let year = if q_lower.contains("2026") {
        "2026"
    } else if q_lower.contains("2025") {
        "2025"
    } else if q_lower.contains("2024") {
        "2024"
    } else {
        "2026"
    };
    
    if let Some(term_dates) = kb.term_dates.get(year) {
        if q_lower.contains("term 1") {
            for term in term_dates {
                if term.term == 1 {
                    if q_lower.contains("start") {
                        return format!("Term 1 starts on {} in {}.", term.start, year);
                    } else if q_lower.contains("end") {
                        return format!("Term 1 ends on {} in {}.", term.end, year);
                    } else {
                        return format!("Term 1 runs from {} to {} in {}.", term.start, term.end, year);
                    }
                }
            }
        } else if q_lower.contains("term 2") {
            for term in term_dates {
                if term.term == 2 {
                    if q_lower.contains("start") {
                        return format!("Term 2 starts on {} in {}.", term.start, year);
                    } else if q_lower.contains("end") {
                        return format!("Term 2 ends on {} in {}.", term.end, year);
                    } else {
                        return format!("Term 2 runs from {} to {} in {}.", term.start, term.end, year);
                    }
                }
            }
        } else if q_lower.contains("term 3") {
            for term in term_dates {
                if term.term == 3 {
                    if q_lower.contains("start") {
                        return format!("Term 3 starts on {} in {}.", term.start, year);
                    } else if q_lower.contains("end") {
                        return format!("Term 3 ends on {} in {}.", term.end, year);
                    } else {
                        return format!("Term 3 runs from {} to {} in {}.", term.start, term.end, year);
                    }
                }
            }
        } else if q_lower.contains("term 4") {
            for term in term_dates {
                if term.term == 4 {
                    if q_lower.contains("start") {
                        return format!("Term 4 starts on {} in {}.", term.start, year);
                    } else if q_lower.contains("end") {
                        return format!("Term 4 ends on {} in {}.", term.end, year);
                    } else {
                        return format!("Term 4 runs from {} to {} in {}.", term.start, term.end, year);
                    }
                }
            }
        }
        
        // Return all terms
        let mut response = format!("Term dates for {}:\n", year);
        for term in term_dates {
            response.push_str(&format!("  Term {}: {} - {}\n", term.term, term.start, term.end));
        }
        return response;
    }
    
    format!("I don't have term date information for {}.", year)
}

fn answer_recess(_kb: &KnowledgeBase, question: &str) -> String {
    let q_lower = question.to_lowercase();
    
    // Determine year
    let year = if q_lower.contains("2026") {
        "2026"
    } else if q_lower.contains("2025") {
        "2025"
    } else if q_lower.contains("2024") {
        "2024"
    } else {
        "2026"
    };
    
    // Check for first recess (usually after Term 1)
    if q_lower.contains("first") || q_lower.contains("1st") || q_lower.contains("term 1") {
        if year == "2026" {
            return "The first recess in 2026 starts on **27 March 2026**.".to_string();
        } else if year == "2025" {
            return "The first recess in 2025 started on **28 March 2025**.".to_string();
        } else if year == "2024" {
            return "The first recess in 2024 started on **29 March 2024**.".to_string();
        }
    }
    
    // Check for winter recess (after Term 2)
    if q_lower.contains("winter") || q_lower.contains("second") || q_lower.contains("2nd") || q_lower.contains("term 2") {
        if year == "2026" {
            return "The winter recess in 2026 starts on **26 June 2026**.".to_string();
        } else if year == "2025" {
            return "The winter recess in 2025 started on **27 June 2025**.".to_string();
        } else if year == "2024" {
            return "The winter recess in 2024 started on **28 June 2024**.".to_string();
        }
    }
    
    // Check for spring recess (after Term 3)
    if q_lower.contains("spring") || q_lower.contains("third") || q_lower.contains("3rd") || q_lower.contains("term 3") {
        if year == "2026" {
            return "The spring recess in 2026 starts on **25 September 2026**.".to_string();
        } else if year == "2025" {
            return "The spring recess in 2025 started on **26 September 2025**.".to_string();
        } else if year == "2024" {
            return "The spring recess in 2024 started on **27 September 2024**.".to_string();
        }
    }
    
    // Check for summer recess (after Term 4)
    if q_lower.contains("summer") || q_lower.contains("fourth") || q_lower.contains("4th") || q_lower.contains("term 4") || q_lower.contains("end of year") {
        if year == "2026" {
            return "The summer/end-of-year recess in 2026 starts on **4 December 2026**.".to_string();
        } else if year == "2025" {
            return "The summer/end-of-year recess in 2025 started on **5 December 2025**.".to_string();
        } else if year == "2024" {
            return "The summer/end-of-year recess in 2024 started on **6 December 2024**.".to_string();
        }
    }
    
    // General answer if no specific recess mentioned
    format!("University recess dates in {}:
  • First recess: **27 March**
  • Winter recess: **26 June**
  • Spring recess: **25 September**
  • Summer recess: **4 December**", year)
}

fn answer_holiday(_kb: &KnowledgeBase, question: &str) -> String {
    let q_lower = question.to_lowercase();
    
    // Hard-coded holiday dates based on South African public holidays
    if q_lower.contains("christmas") || q_lower.contains("xmas") {
        return "Christmas Day is on 25 December.".to_string();
    } else if q_lower.contains("new year") {
        return "New Year's Day is on 1 January.".to_string();
    } else if q_lower.contains("human rights") {
        return "Human Rights Day is on 21 March.".to_string();
    } else if q_lower.contains("freedom day") {
        return "Freedom Day is on 27 April.".to_string();
    } else if q_lower.contains("workers") || q_lower.contains("labour") {
        return "Workers Day is on 1 May.".to_string();
    } else if q_lower.contains("youth day") {
        return "Youth Day is on 16 June.".to_string();
    } else if q_lower.contains("women") {
        return "Women's Day is on 9 August.".to_string();
    } else if q_lower.contains("heritage") {
        return "Heritage Day is on 24 September.".to_string();
    } else if q_lower.contains("reconciliation") {
        return "Day of Reconciliation is on 16 December.".to_string();
    } else if q_lower.contains("good friday") {
        return "Good Friday is a movable date based on the Christian calendar.".to_string();
    } else if q_lower.contains("family day") {
        return "Family Day is the Monday after Easter Sunday.".to_string();
    } else if q_lower.contains("day of goodwill") {
        return "Day of Goodwill is on 26 December.".to_string();
    }
    
    // If no specific holiday matched, return list of public holidays
    "South African public holidays include:
  • New Year's Day (1 January)
  • Human Rights Day (21 March)
  • Good Friday (movable)
  • Family Day (movable)
  • Freedom Day (27 April)
  • Workers Day (1 May)
  • Youth Day (16 June)
  • Women's Day (9 August)
  • Heritage Day (24 September)
  • Day of Reconciliation (16 December)
  • Christmas Day (25 December)
  • Day of Goodwill (26 December)".to_string()
}

// ===== NEW FUNCTIONS =====

fn answer_exams(_kb: &KnowledgeBase, question: &str) -> String {
    let q_lower = question.to_lowercase();
    
    // Determine year
    let year = if q_lower.contains("2026") {
        "2026"
    } else if q_lower.contains("2025") {
        "2025"
    } else if q_lower.contains("2024") {
        "2024"
    } else {
        "2026"
    };
    
    if q_lower.contains("term 1") || q_lower.contains("first term") {
        return format!("Term 1 exams in {} are held in **March/April** after Term 1 ends.", year);
    } else if q_lower.contains("term 2") || q_lower.contains("second term") || q_lower.contains("mid-year") {
        return format!("Mid-year exams in {} are held in **June** after Term 2 ends.", year);
    } else if q_lower.contains("term 3") || q_lower.contains("third term") {
        return format!("Term 3 exams in {} are held in **September/October** after Term 3 ends.", year);
    } else if q_lower.contains("term 4") || q_lower.contains("fourth term") || q_lower.contains("final") || q_lower.contains("end of year") {
        return format!("End-of-year exams in {} are held in **November/December** after Term 4 ends.", year);
    }
    
    format!("Exam periods in {}:
  • Term 1 exams: March/April
  • Term 2 exams: June (mid-year)
  • Term 3 exams: September/October
  • Term 4 exams: November/December (final)", year)
}

fn answer_deadlines(_kb: &KnowledgeBase, question: &str) -> String {
    let q_lower = question.to_lowercase();
    
    if q_lower.contains("graduation") || q_lower.contains("graduate") {
        if q_lower.contains("apply") || q_lower.contains("application") || q_lower.contains("deadline") {
            return "Graduation application deadlines are typically **2-3 months before** the ceremony. For Autumn graduation (April), apply by **January/February**. For Summer graduation (December), apply by **September/October**.".to_string();
        }
    } else if q_lower.contains("registration") || q_lower.contains("enrol") || q_lower.contains("enroll") {
        return "Registration deadlines are usually **2-4 weeks before** the start of each term.".to_string();
    } else if q_lower.contains("bursary") || q_lower.contains("financial aid") || q_lower.contains("funding") {
        return "Financial aid applications typically close in **October/November** for the following academic year.".to_string();
    }
    
    "Application deadlines vary. Please specify what you're applying for (graduation, registration, funding, etc.).".to_string()
}

fn answer_term_duration(_kb: &KnowledgeBase, question: &str) -> String {
    let q_lower = question.to_lowercase();
    
    // Determine year
    let year = if q_lower.contains("2026") {
        "2026"
    } else if q_lower.contains("2025") {
        "2025"
    } else if q_lower.contains("2024") {
        "2024"
    } else {
        "this year"
    };
    
    // Check for specific term
    if q_lower.contains("term 1") || q_lower.contains("first term") {
        if year == "2026" {
            return "Term 1 of 2026 is **11 weeks** long, running from **27 January to 13 April 2026**.".to_string();
        } else if year == "2025" {
            return "Term 1 of 2025 was **11 weeks** long, running from **28 January to 14 April 2025**.".to_string();
        } else if year == "2024" {
            return "Term 1 of 2024 was **11 weeks** long, running from **29 January to 15 April 2024**.".to_string();
        } else {
            return "Term 1 is approximately **11 weeks** long, typically running from late January to mid-April.".to_string();
        }
    }
    
    if q_lower.contains("term 2") || q_lower.contains("second term") {
        if year == "2026" {
            return "Term 2 of 2026 is **10 weeks** long, running from **28 April to 7 July 2026**.".to_string();
        } else if year == "2025" {
            return "Term 2 of 2025 was **10 weeks** long, running from **29 April to 8 July 2025**.".to_string();
        } else if year == "2024" {
            return "Term 2 of 2024 was **10 weeks** long, running from **30 April to 9 July 2024**.".to_string();
        } else {
            return "Term 2 is approximately **10 weeks** long, typically running from late April to early July.".to_string();
        }
    }
    
    if q_lower.contains("term 3") || q_lower.contains("third term") {
        if year == "2026" {
            return "Term 3 of 2026 is **11 weeks** long, running from **21 July to 6 October 2026**.".to_string();
        } else if year == "2025" {
            return "Term 3 of 2025 was **11 weeks** long, running from **22 July to 7 October 2025**.".to_string();
        } else if year == "2024" {
            return "Term 3 of 2024 was **11 weeks** long, running from **23 July to 8 October 2024**.".to_string();
        } else {
            return "Term 3 is approximately **11 weeks** long, typically running from late July to early October.".to_string();
        }
    }
    
    if q_lower.contains("term 4") || q_lower.contains("fourth term") {
        if year == "2026" {
            return "Term 4 of 2026 is **9 weeks** long, running from **13 October to 15 December 2026**.".to_string();
        } else if year == "2025" {
            return "Term 4 of 2025 was **9 weeks** long, running from **14 October to 16 December 2025**.".to_string();
        } else if year == "2024" {
            return "Term 4 of 2024 was **9 weeks** long, running from **15 October to 17 December 2024**.".to_string();
        } else {
            return "Term 4 is approximately **9 weeks** long, typically running from mid-October to mid-December.".to_string();
        }
    }
    
    "Each term varies in length. Which term would you like to know about?".to_string()
}


pub fn interactive_mode(kb: &KnowledgeBase) -> io::Result<()> {
    println!("\n🤖 CPUT Calendar Q&A System (type 'quit' to exit)");
    println!("==================================================");
    
    loop {
        print!("\n❓ Your question: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" || input == "exit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        let answer = answer_question(kb, input);
        println!("✅ Answer: {}", answer);
    }
    
    println!("\nGoodbye! 👋");
    Ok(())
}