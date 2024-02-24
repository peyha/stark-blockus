use anyhow::Result;
use reqwest::Client;
use serde::Serialize;
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::num::ParseIntError;

use crate::utils::parse_hexa_value;

// Struct for GetBlock RPC call
#[derive(Serialize)]
struct GetBlockNumberRequest {
    params: (),
    jsonrpc: &'static str,
    method: &'static str,
    id: &'static str,
}

#[derive(Serialize)]
struct BlockInfo {
    block_id: BlockIdentifier,
}

#[derive(Serialize)]
struct BlockIdentifier {
    block_number: u64,
}

#[derive(Serialize)]
struct GetBlockRequest {
    params: BlockInfo,
    jsonrpc: &'static str,
    method: &'static str,
    id: &'static str,
}

#[derive(Debug)]
pub enum BlockNumberFetchErr {
    RequestFail(reqwest::Error),
    ConversionFail(serde_json::Error),
    NumberConvertFail(),
}

pub async fn get_block_number(url: String) -> Result<u64, BlockNumberFetchErr> {
    let request = GetBlockNumberRequest {
        params: (),
        jsonrpc: "2.0",
        method: "starknet_blockNumber",
        id: "1",
    };
    let client = Client::new();
    let res = client
        .post(url)
        .json(&request)
        .send()
        .await
        .map_err(BlockNumberFetchErr::RequestFail)?
        .text()
        .await
        .map_err(BlockNumberFetchErr::RequestFail)?;

    let data: Value =
        serde_json::from_str(res.as_str()).map_err(BlockNumberFetchErr::ConversionFail)?;

    Ok(data["result"]
        .as_number()
        .ok_or(BlockNumberFetchErr::NumberConvertFail())?
        .as_u64()
        .ok_or(BlockNumberFetchErr::NumberConvertFail())?)
}

#[derive(Debug)]
pub enum BlockFetchErr {
    RequestFail(reqwest::Error),
    ConversionFail(serde_json::Error),
    NumberConvertFail(String),
    IntConvertFail(ParseIntError),
    IndexError(String),
}

pub async fn get_block(url: String, block_id: u64) -> Result<Vec<String>, BlockFetchErr> {
    let mut lines = Vec::new();

    let request = GetBlockRequest {
        params: BlockInfo {
            block_id: BlockIdentifier {
                block_number: block_id,
            },
        },
        jsonrpc: "2.0",
        method: "starknet_getBlockWithTxs",
        id: "1",
    };

    let client = Client::new();
    let res = client
        .post(url)
        .json(&request)
        .send()
        .await
        .map_err(BlockFetchErr::RequestFail)?
        .text()
        .await
        .map_err(BlockFetchErr::RequestFail)?;

    let data: Value = serde_json::from_str(res.as_str()).map_err(BlockFetchErr::ConversionFail)?;
    let block_info = data.get("result").ok_or(BlockFetchErr::IndexError("result".to_string()))?;

    // block info

    let block_number = block_info
        .get("block_number")
        .ok_or(BlockFetchErr::IndexError("block_number".to_string()))?
        .as_number()
        .ok_or(BlockFetchErr::NumberConvertFail("block_number".to_string()))?;

    lines.push(format!("Block number: {}", block_number));

    let timestamp = block_info
        .get("timestamp")
        .ok_or(BlockFetchErr::IndexError("timestamp".to_string()))?
        .as_number()
        .ok_or(BlockFetchErr::NumberConvertFail("timestamp".to_string()))?;

    lines.push(format!("Timestamp: {}", timestamp)); // TODO format date

    let block_hash = block_info
        .get("block_hash")
        .ok_or(BlockFetchErr::IndexError("block_hash".to_string()))?
        .as_str()
        .ok_or(BlockFetchErr::IndexError("block_hash".to_string()))?;

    lines.push(format!("Block hash: {}", block_hash));

    let parent_hash = block_info
        .get("parent_hash")
        .ok_or(BlockFetchErr::IndexError("parent_hash".to_string()))?
        .as_str()
        .ok_or(BlockFetchErr::IndexError("parent_hash".to_string()))?;
    lines.push(format!("Parent hash: {}", parent_hash));

    let starknet_version = block_info
        .get("starknet_version")
        .ok_or(BlockFetchErr::IndexError("starknet_version".to_string()))?
        .as_str()
        .ok_or(BlockFetchErr::IndexError("starknet_version".to_string()))?;

    lines.push(format!("Starknet version: {}", starknet_version));

    let status = block_info
        .get("status")
        .ok_or(BlockFetchErr::IndexError("status".to_string()))?
        .as_str()
        .ok_or(BlockFetchErr::IndexError("status".to_string()))?;

    lines.push(format!("Block status: {}", status));

    let sequencer_address = block_info
        .get("sequencer_address")
        .ok_or(BlockFetchErr::IndexError("sequencer_address".to_string()))?
        .as_str()
        .ok_or(BlockFetchErr::IndexError("sequencer_address".to_string()))?;

    lines.push(format!(
        "Sequencer address on mainnet is {}",
        sequencer_address
    ));
    let new_root = block_info
        .get("new_root")
        .ok_or(BlockFetchErr::IndexError("new_root".to_string()))?
        .as_str()
        .ok_or(BlockFetchErr::IndexError("new_root".to_string()))?;
    lines.push(format!("New root is {}", new_root));

    let l1_gas_price = parse_hexa_value(
        block_info
            .get("l1_gas_price")
            .unwrap_or(&Value::Number(Number::from(0)))
            .get("price_in_wei")
            .unwrap_or(&Value::Number(Number::from(0))),
    )? as f64
        / 1e9;
    lines.push(format!("L1 gas price is {:.2}", l1_gas_price));

    let txs = block_info
        .get("transactions")
        .ok_or(BlockFetchErr::IndexError("transactions".to_string()))?
        .as_array()
        .ok_or(BlockFetchErr::IndexError("transactions".to_string()))?;

    let nb_txs = txs.len() as u64;
    let mut tx_version_count: HashMap<String, u64> = HashMap::new();
    let (mut min_fee, mut max_seen_fee, mut avg_fee) = (i32::MAX, i32::MIN, 0 as f64);
    let mut type_version_count: HashMap<(String, u64), u64> = HashMap::new();
    for tx in txs {
        // TODO decode tx according to version
        let max_fee: i32 =
            parse_hexa_value(tx.get("max_fee").unwrap_or(&Value::Number(Number::from(0))))
                .unwrap_or(0) as i32;
        min_fee = i32::min(min_fee, max_fee);
        max_seen_fee = i32::max(max_seen_fee, max_fee);
        avg_fee += (max_fee as f64) / (nb_txs as f64);

        let tx_type = tx
            .get("type")
            .ok_or(BlockFetchErr::IndexError("type".to_string()))?
            .as_str()
            .ok_or(BlockFetchErr::IndexError("type".to_string()))?;

        if !tx_version_count.contains_key(&tx_type.to_string()) {
            tx_version_count.insert(tx_type.to_string(), 0);
        }
        let cur_version_count = tx_version_count
            .get(&tx_type.to_string())
            .ok_or(BlockFetchErr::IndexError("type".to_string()))?;
        tx_version_count.insert(tx_type.to_string(), cur_version_count + 1);

        let version = parse_hexa_value(
            tx.get("version")
                .ok_or(BlockFetchErr::IndexError("version".to_string()))?,
        )? as u64;

        if !type_version_count.contains_key(&(tx_type.to_string(), version)) {
            type_version_count.insert((tx_type.to_string(), version), 0);
        }
        let cur_count = type_version_count
            .get(&(tx_type.to_string(), version))
            .ok_or(BlockFetchErr::IndexError("type".to_string()))?;
        type_version_count.insert((tx_type.to_string(), version), cur_count + 1);
    }

    lines.push(format!(
        "Max fee: min={:.2} gwei, max={:.2} gwei, avg={:.2} gwei",
        min_fee as f64 / 1e9,
        max_seen_fee as f64 / 1e9,
        avg_fee / 1e9
    ));
    let mut cur_line = format!("Tx cnt: {} ", nb_txs);
    for (tx_type, count) in tx_version_count.iter() {
        cur_line.push_str(format!(", {}: {}", tx_type, count).as_str());
    }
    lines.push(cur_line);

    for ((s, n), v) in type_version_count.iter() {
        lines.push(format!("Seen {} type with version {} {} times", s, n, v));
    }

    // interesting attribute in txs: max_fee, type (INVOKE, ...), version
    Ok(lines)
}
