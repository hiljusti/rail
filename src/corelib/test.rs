use colored::Colorize;

use crate::rail_machine::RailDef;

// TODO: More forms, optional messages, etc. Input as stab? Output as stab or quote of failures?
pub fn builtins() -> Vec<RailDef<'static>> {
    vec![RailDef::on_quote(
        "assert-true",
        &["bool", "string"],
        &[],
        |quote| {
            let (msg, quote) = quote.pop_string("assert-true");
            let (b, quote) = quote.pop_bool("assert-true");

            if !b {
                let msg = format!("Assertion failed: {}", msg).red();
                eprintln!("{}", msg);
                std::process::exit(1);
            }

            quote
        },
    )]
}
