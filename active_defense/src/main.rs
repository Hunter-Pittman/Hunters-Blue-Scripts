use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author="Hunter Pittman", version, about="Program for hardening Windows", long_about = None)]

struct Args {
    #[clap(short, long, required = false, takes_value = false)]
    firewall: String,

    #[clap(short, long, takes_value = false, required = false)]
    ports: String
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}