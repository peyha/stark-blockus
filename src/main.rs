use clap::Parser;
use anyhow::Result;

mod block_utils;
use block_utils::{get_block, get_block_number};

mod utils;
use utils::{display_pretty_block, DisplayType};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args{
    #[arg(short, long)]
    rpc: String,
}




#[tokio::main]
async fn main() {
    
    let args = Args::parse();

    let mut cur_block_number = 0;
    
    loop {
        
        let block_number = get_block_number(args.rpc.clone()).await.unwrap();
        if cur_block_number != block_number {
            
            match get_block(args.rpc.clone(), block_number).await {
                Ok(lines) => {
                    if let Result::Err(e) = display_pretty_block(lines, DisplayType::SingleLine) {
                        println!("Block display error on block {}: {:?}", block_number, e)
                    };
                    cur_block_number = block_number;
                },
                Err(e) => {
                    println!("Block retrieval error on block {}: {:?}", block_number, e);
                }
            };

        }
    }
}
