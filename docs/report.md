\# CPUT Calendar Q\&A System - Project Report



\## 1. Introduction



This project implements a Question-Answering system that reads CPUT academic calendar documents and answers natural language questions about graduations, committee meetings, term dates, recess periods, and public holidays.



\## 2. Implementation



\### Architecture

\- \*\*Document Processing\*\*: Uses zip crate to extract text from .docx files

\- \*\*Knowledge Base\*\*: JSON-based storage of extracted calendar information

\- \*\*QA Engine\*\*: Pattern matching and rule-based question answering

\- \*\*CLI Interface\*\*: Built with clap crate for user interaction



\### Key Components

1\. \*\*data.rs\*\*: Handles document reading and information extraction

2\. \*\*qa.rs\*\*: Contains question classification and answering logic

3\. \*\*main.rs\*\*: Manages CLI commands and program flow



\### Data Pipeline

1\. Read .docx files from data/ directory

2\. Extract text content using zip archive parsing

3\. Apply regex patterns to identify:

   - Graduation dates

   - Committee meetings

   - Term start/end dates

   - Recess periods

   - Public holidays

4\. Store structured data in knowledge.json



\## 3. Results



The system successfully answers various types of questions:



| Question | Answer |

|----------|--------|

| When is graduation in 2026? | In 2026, graduation ceremonies are held in April (Autumn) and December (Summer). |

| How many HDC meetings in 2024? | The Higher Degrees Committee met 16 times in 2024. |

| When does Term 1 start in 2026? | Term 1 starts in January 2026. |

| What is the start date of winter recess 2025? | The winter recess in 2025 started in late June or early July. |

| What date is Christmas? | Christmas Day is on 25 December. |



\## 4. Challenges and Solutions



\- \*\*DOCX Parsing\*\*: Used zip crate to directly access document.xml

\- \*\*Date Recognition\*\*: Implemented flexible regex patterns

\- \*\*Spelling Variations\*\*: Added tolerance for common misspellings

\- \*\*Year Detection\*\*: Automatic detection of 2024, 2025, 2026 references



\## 5. Conclusion



This project successfully demonstrates a Question-Answering system for CPUT academic calendars. Key achievements include:

\- Successfully extracting data from Word documents

\- Answering natural language questions about graduations, committees, and holidays

\- Handling misspellings and variations in questions

\- Providing both single-question and interactive modes



Future improvements could include:

\- More precise date extraction from calendar tables

\- Support for additional question types

\- A web-based interface

