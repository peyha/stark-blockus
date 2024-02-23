use serde::Serialize;
use serde_json::{Value, Number};
use reqwest::Client;
use std::collections::HashMap;
use anyhow::Result;

use crate::utils::parse_hexa_value;

// Struct for GetBlock RPC call
#[derive(Serialize)]
struct GetBlockNumberRequest{
    params: (),
    jsonrpc: &'static str,
    method: &'static str,
    id: &'static str
}

#[derive(Serialize)]
struct BlockInfo{
    block_id: BlockIdentifier,
}

#[derive(Serialize)]
struct BlockIdentifier{
    block_number: u64
}

#[derive(Serialize)]
struct GetBlockRequest{
    params: BlockInfo,
    jsonrpc: &'static str,
    method: &'static str,
    id: &'static str
}

pub async fn get_block_number(url: String) -> Result<u64> {

    let request = GetBlockNumberRequest {
        params: (),
        jsonrpc:  "2.0",
        method: "starknet_blockNumber",
        id: "1",
    };
    let client = Client::new();
    let res = client.post(url)
        .json(&request)
        .send()
        .await?
        .text()
        .await?;
    let data: Value = serde_json::from_str(res.as_str())?;
    
    Ok(data["result"].as_number().unwrap().as_u64().unwrap())
}

pub async fn get_block(url: String, block_id: u64) -> Result<Vec<String>> {
    let mut lines = Vec::new();

    let request = GetBlockRequest {
        params: BlockInfo{
            block_id: BlockIdentifier{
                block_number: block_id,
            }
        },
        jsonrpc:  "2.0",
        method: "starknet_getBlockWithTxs",
        id: "1",
    };

    let client = Client::new();
    let res = client.post(url)
        .json(&request)
        .send()
        .await?
        .text()
        .await?;
    let data: Value = serde_json::from_str(res.as_str())?;
    println!("{}", data);
    let block_info = data.get("result").unwrap();

    // block info
    
    let block_number = block_info.get("block_number").unwrap().as_number().unwrap();
    lines.push(
        format!("Block number: {}", block_number)
    );

    let timestamp = block_info.get("timestamp").unwrap().as_number().unwrap();
    lines.push(
        format!("Timestamp: {}", timestamp)
    ); // TODO format date

    let block_hash = block_info.get("block_hash").unwrap().as_str().unwrap();
    lines.push(
        format!("Block hash: {}", block_hash)
    );

    let parent_hash = block_info.get("parent_hash").unwrap().as_str().unwrap();
    lines.push(
        format!("Parent hash: {}", parent_hash)
    );

    let starknet_version = block_info.get("starknet_version").unwrap().as_str().unwrap();
    lines.push(
        format!("Starknet version: {}", starknet_version)
    );

    let status = block_info.get("status").unwrap().as_str().unwrap();
    lines.push(
        format!("Block status: {}", status)
    );

    let sequencer_address = block_info.get("sequencer_address").unwrap().as_str().unwrap();
    lines.push(
        format!("Sequencer address on mainnet is {}", sequencer_address)
    );
    let new_root = block_info.get("new_root").unwrap().as_str().unwrap();
    lines.push(
        format!("New root is {}", new_root)
    );

    let l1_gas_price =  parse_hexa_value(block_info.get("l1_gas_price").unwrap().get("price_in_wei").unwrap()).unwrap() as f64 / 1e9;
    lines.push(
        format!("L1 gas price is {:.2}", l1_gas_price)
    );

    let txs = block_info.get("transactions").unwrap().as_array().unwrap();
    let nb_txs = txs.len() as u64;
    let mut tx_version_count: HashMap<String, u64> = HashMap::new();
    let (mut min_fee, mut max_seen_fee, mut avg_fee) = (i32::MAX, i32::MIN, 0 as f64);
    let mut type_version_count: HashMap<(String, u64), u64> = HashMap::new();
    for tx in txs {
        // TODO decode tx according to version
        let max_fee: i32 = parse_hexa_value(tx.get("max_fee").unwrap_or(&Value::Number(Number::from(0)))).unwrap_or(0) as i32 ;
        min_fee = i32::min(min_fee, max_fee);
        max_seen_fee = i32::max(max_seen_fee, max_fee);
        avg_fee += (max_fee as f64) / (nb_txs as f64);
        
        let tx_type = tx.get("type").unwrap().as_str();
        
        if !tx_version_count.contains_key(&tx_type.unwrap().to_string()){
            tx_version_count.insert(tx_type.unwrap().to_string(), 0);
        }
        let cur_version_count = tx_version_count.get(&tx_type.unwrap().to_string()).unwrap();
        tx_version_count.insert(tx_type.unwrap().to_string(), cur_version_count + 1);

        let version = parse_hexa_value(tx.get("version").unwrap()).unwrap();
        
        if !type_version_count.contains_key(&(tx_type.unwrap().to_string(), version)){
            type_version_count.insert((tx_type.unwrap().to_string(), version), 0);
        }
        let cur_count = type_version_count.get(&(tx_type.unwrap().to_string(), version)).unwrap();
        type_version_count.insert((tx_type.unwrap().to_string(), version), cur_count + 1);
    }

    lines.push(
        format!("Max fee: min={:.2} gwei, max={:.2} gwei, avg={:.2} gwei", min_fee as f64 / 1e9, max_seen_fee as f64 / 1e9, avg_fee / 1e9)
    );
    let mut cur_line = format!("Tx cnt: {} ", nb_txs);
    for (tx_type, count) in tx_version_count.iter(){
        cur_line.push_str(format!(", {}: {}", tx_type, count).as_str());
    }
    lines.push(cur_line);

    for ((s, n), v) in type_version_count.iter(){
        lines.push(
            format!("Seen {} type with version {} {} times", s, n, v)
        );
    }

    // interesting attribute in txs: max_fee, type (INVOKE, ...), version
    Ok(lines)
    
}