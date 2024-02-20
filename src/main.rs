use clap::Parser;


mod block_utils;
use block_utils::{get_block, get_block_number};

mod utils;
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args{
    #[arg(short, long)]
    rpc: String,
}




#[tokio::main]
async fn main() {
    
    let args = Args::parse();

    let block_number = get_block_number(args.rpc.clone()).await.unwrap();
    let lines = get_block(args.rpc, block_number).await.unwrap();

    for line in lines {
        println!("{}", line);
    }
    // print info of block
}
