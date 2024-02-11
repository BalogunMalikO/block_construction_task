use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
//create a struct of mempool tx
struct MempoolTransaction {
    txid: String,
    fee: i32,
    weight: i32,
    parents: Vec<String>,
}

impl MempoolTransaction {
    fn new(txid: String, fee: i32, weight: i32, parents: Vec<String>) -> Self {
        MempoolTransaction { txid, fee, weight, parents }
    }
}
//read the mempool csv file, select transactions and returns a list of
//mempoolTransaction objects.
fn parse_mempool_csv(file_path: &str) -> Result<Vec<MempoolTransaction>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut transactions = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let parts: Vec<_> = line?.split(',').map(|s| s.to_string()).collect();
        let txid = parts[0].clone();
        let fee = parts[1].parse()?;
        let weight = parts[2].parse()?;
        let parents = if parts[3].is_empty() { Vec::new() }
          else { parts[3].split(',').map(|s| s.to_string()).collect() };
        
        transactions.push(MempoolTransaction::new(txid, fee, weight, parents));
    }

    Ok(transactions)
}
// this takes list of mempooltransansaction and return a set of
//selected transaction ids. itt iterate through sorted transactions by fee in descending order
//and checks if it meets the following conditions:
//not already selscted, parents are selcted and weight does not exceed 4000000
fn select_transactions(transactions: Vec<MempoolTransaction>) -> Vec<String> {
    let mut selected_txids = Vec::new();
    let mut total_weight = 0;
    let mut sorted_transactions = transactions.clone();
    sorted_transactions.sort_by_key(|t| -t.fee);

    for tx in sorted_transactions {
        if !selected_txids.contains(&tx.txid) && tx.parents.iter().all(|parent_txid| selected_txids.contains(parent_txid)) {
            selected_txids.push(tx.txid.clone());
            total_weight += tx.weight;
            if total_weight > 4000000 {
                break;
            }
        }
    }

    selected_txids
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "mempool.csv";
    let transactions = parse_mempool_csv(file_path)?;

    let selected_txids = select_transactions(transactions);

    for txid in selected_txids {
        println!("{}", txid);
    }

    Ok(())
}
