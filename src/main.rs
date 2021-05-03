mod cli;
mod tasks;

use anyhow::anyhow;
use cli::{Action::Add, Action::Done, Action::List, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // Get the command line args
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack thr journal file.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    //Perform the action.
    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        Done { task_position } => tasks::complete_task(journal_file, task_position),
        List => tasks::list_tak(journal_file),
    }?;

    Ok(())
}
