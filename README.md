\# CPUT Calendar Q\&A System



A Rust-based Question-Answering system that reads academic calendar documents and answers questions about graduations, committee meetings, term dates, recess periods, and public holidays.



\## Features



\- 📅 \*\*Graduation Queries\*\*: "When is graduation in 2026?"

\- 📊 \*\*Committee Meetings\*\*: "How many times did HDC meet in 2024?"

\- 📚 \*\*Term Dates\*\*: "When does Term 1 start?"

\- 🏖️ \*\*Recess Periods\*\*: "When is winter recess in 2025?"

\- 🎉 \*\*Public Holidays\*\*: "What date is Christmas?"

\- 💬 \*\*Interactive Mode\*\*: Chat-like interface

\- ⌨️ \*\*Command Line\*\*: Single question mode



\## Installation



\### Prerequisites

\- Rust (install from \[rustup.rs](https://rustup.rs/))

\- Git



\### Steps



1\. Clone the repository:

   ```bash

   git clone https://github.com/Amanda2026-crypto/cput-calendar-qa.git

   cd cput-calendar-qa



2\. Add your calendar files to the `data/` folder:

&nbsp;  - `calendar\_2024.docx`

&nbsp;  - `calendar\_2025.docx`

&nbsp;  - `calendar\_2026.docx`



3\. Build the knowledge base:

&nbsp;  ```bash

&nbsp;  cargo run -- build

