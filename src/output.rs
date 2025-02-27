use crate::api::ExtractedContent;
use colored::*;
use tavily::SearchResponse;

/// Format and print search results
pub fn print_search_results(response: &SearchResponse) {
    // Print answer if available
    if let Some(answer) = &response.answer {
        println!("\n{}\n{}\n", "AI Answer:".green().bold(), answer);

        // Check if follow-up questions exist
        if let Some(follow_up) = &response.follow_up_questions {
            if !follow_up.is_empty() {
                println!("{}", "Follow-up questions:".yellow());
                for (i, question) in follow_up.iter().enumerate() {
                    println!("  {}. {}", i + 1, question);
                }
                println!();
            }
        }
    }

    // Print search results
    println!("{}", "Search Results:".blue().bold());

    if response.results.is_empty() {
        println!("  No results found");
        return;
    }

    for (i, result) in response.results.iter().enumerate() {
        println!(
            "{}. {} {}",
            (i + 1).to_string().cyan(),
            result.title.bold(),
            format!("({})", result.url).dimmed()
        );

        // Print content snippet if available
        let content = &result.content;
        if !content.is_empty() {
            let preview = if content.len() > 150 {
                format!("{}...", &content[..150])
            } else {
                content.to_string()
            };
            println!("   {}", preview);
        }

        println!();
    }
}

/// Format and print extraction results
pub fn print_extraction_results(extractions: &[ExtractedContent], extract_limit: usize) {
    println!("{}", "Extraction Results:".blue().bold());

    if extractions.is_empty() {
        println!("  No extractions found");
        return;
    }

    for (i, extract) in extractions.iter().enumerate() {
        // Display title if available, otherwise just the URL
        if let Some(title) = &extract.title {
            println!(
                "{}. {} {}",
                (i + 1).to_string().cyan(),
                title.bold(),
                format!("({})", extract.url).dimmed().underline()
            );
        } else {
            println!(
                "{}. {}",
                (i + 1).to_string().cyan(),
                extract.url.underline()
            );
        }

        if let Some(content) = &extract.content {
            let preview = if content.len() > extract_limit {
                format!("{}...", &content[..extract_limit])
            } else {
                content.to_string()
            };
            println!("\n{}\n", preview);
        } else {
            println!("  No content available");
        }

        println!();
    }
}
