use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::collections::HashMap;

mod data;
mod qa;

// Add this function temporarily to debug
fn debug_knowledge(kb: &data::KnowledgeBase) {
    println!("=== DEBUG: Knowledge Base Contents ===");
    println!("Committees found: {:?}", kb.committee_counts.keys());
    for (committee, years) in &kb.committee_counts {
        println!("  {}: {:?}", committee, years);
    }
    println!("=====================================");
}

#[derive(Parser)]
#[command(author, version, about = "CPUT Calendar Q&A System")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build knowledge base from calendar documents
    Build {
        /// Path to directory containing .docx files
        #[arg(short, long, default_value = "./data")]
        data_dir: PathBuf,
        
        /// Path to save knowledge base
        #[arg(short, long, default_value = "./knowledge.json")]
        output: PathBuf,
    },
    
    /// Ask a question
    Ask {
        /// Path to knowledge base
        #[arg(short, long, default_value = "./knowledge.json")]
        knowledge: PathBuf,
        
        /// The question to ask
        #[arg(short, long)]
        question: String,
    },
    
    /// Interactive mode
    Interactive {
        /// Path to knowledge base
        #[arg(short, long, default_value = "./knowledge.json")]
        knowledge: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Build { data_dir, output } => {
            println!("🔨 Building knowledge base from: {}", data_dir.display());
            let knowledge = data::build_knowledge_base(&data_dir)?;
            knowledge.save(&output)?;
            println!("✅ Knowledge base saved to: {}", output.display());
        }
        
       Commands::Ask { knowledge, question } => {
    let knowledge = data::KnowledgeBase::load(&knowledge)?;
    debug_knowledge(&knowledge); // Add this line temporarily
    let answer = qa::answer_question(&knowledge, &question);
    println!("❓ Question: {}", question);
    println!("✅ Answer: {}", answer);
}

        
        Commands::Interactive { knowledge } => {
            let knowledge = data::KnowledgeBase::load(&knowledge)?;
            qa::interactive_mode(&knowledge)?;
        }
    }
    
    Ok(())
}