use clap::{Parser, Subcommand};
use rail_lang::{prompt::RailPrompt, rail_machine, tokens, RAIL_VERSION};

pub fn main() {
    let args = RailShell::parse();

    let state = rail_machine::state_with_libs(args.no_stdlib, args.lib_list);

    match args.mode {
        Some(Mode::Interactive) | None => RailPrompt::default().run(state),
        Some(Mode::Run { file }) => {
            let tokens = tokens::from_rail_source_file(file);
            state.run_tokens(tokens);
        }
        Some(Mode::RunStdin) => unimplemented!("I don't know how to run stdin yet"),
    }
}

#[derive(Parser)]
#[clap(name = "rail", version = RAIL_VERSION)]
/// A straightforward programming language
struct RailShell {
    #[clap(subcommand)]
    mode: Option<Mode>,

    #[clap(long)]
    /// Disable loading the Rail standard library.
    no_stdlib: bool,

    #[clap(short = 'l', long)]
    /// A file containing a line-separated list of library paths to preload.
    lib_list: Option<String>,
}

#[derive(Subcommand)]
enum Mode {
    #[clap(visible_alias = "i")]
    /// Start an interactive session. (Default when no subcommand specified)
    Interactive,

    #[clap(visible_alias = "r")]
    /// Execute a file.
    Run { file: String },

    #[clap(name = "-")]
    /// Read from standard input.
    RunStdin,
}
