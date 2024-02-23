use clap::Parser;


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
               
            let lines = get_block(args.rpc.clone(), block_number).await.unwrap();
            display_pretty_block(lines, DisplayType::SingleLine);
            cur_block_number = block_number;
        }
    }
}
