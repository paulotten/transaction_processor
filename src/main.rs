use client::ClientsMap;
use transaction::TransactionsMap;

mod args;
mod cents;
mod client;
mod input;
mod output;
mod process;
mod transaction;

fn main() -> Result<(), &'static str> {
    let filename = args::process_args()?;

    let mut clients = ClientsMap::new();
    let mut transactions = TransactionsMap::new();

    input::process_input_file(&filename, &mut clients, &mut transactions)?;

    output::write_accounts(&clients);

    Ok(())
}
