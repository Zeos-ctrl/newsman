pub mod tests;

use clap::Parser;

// Add email to database
// Remove email from database
// Start mailing job
// Execute mailing job
// Start daemon
// Stop daemon


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Adds an email to the mailing list, -a [email]
    #[arg(short, value_name = "EMAIL")]
    add_email: Option<String>,
   
    /// Remove email from mailing list, -r [email]
    #[arg(short, value_name = "EMAIL")]
    remove_email: Option<String>,

    /// Starts a mailing job, -j [newsletter name]
    #[arg(short, value_name = "NEWSLETTER NAME")]
    job: Option<String>,

    /// Executes all mailing jobs, -e
    #[arg(short)]
    execute: Option<bool>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    /*
     * -d : run as daemon 
     * -n : add email 
     * -r : remove email 
     * -j : start mailing job
     * -e : execute all mailing jobs
     */

    let cli = Args::parse();

    if let Some(add_email) = cli.add_email.as_deref() {
        println!("{}", add_email);
    }

    if let Some(remove_email) = cli.remove_email.as_deref() {
        println!("{}", remove_email);
    }

    if let Some(_) = cli.execute {
        println!("Executing jobs");
    }

    if let Some(job) = cli.job.as_deref() {
        println!("{}", job);
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    Ok(())
}
