
use std::collections::HashMap;

use crate::blockchain::errors::BlockError;
use crate::blockchain::status::{BlockStatus, TransactionStatus};
use crate::blockchain::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    block_id: u64,
    transactions: HashMap<u64, Transaction>,
    pub status: BlockStatus,
}

impl Block {
    pub fn new(block_id: u64, transactions: HashMap<u64, Transaction>) -> Block {
        Block {
            block_id,
            transactions,
            status: BlockStatus::InProcess,
        }
    }

    pub fn print(&self) {
        println!("{:?} ", self);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), BlockError> {
        match self.status {
            BlockStatus::InProcess => {
                self.transactions.insert(transaction.transaction_id, transaction);
                Ok(())
            }
            _ => Err(BlockError::BlockFinalized),
        }
    }

    pub fn validate_block(&mut self) -> bool {
        if self
            .transactions
            .iter()
            .all(|(_,tx)| tx.status == TransactionStatus::Completed)
        {
            self.status = BlockStatus::Mined;
            return true;
        } else {
            return false;
        }
    }
    pub fn get_transaction_mut(&mut self, transaction_id: u64) -> Option<&mut Transaction> {
        for tx in &mut self.transactions.values_mut() {
            if tx.transaction_id == transaction_id {
                return Some(tx);
            }
        }
        None
    }

    pub fn get_block_all_transactions(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    pub fn get_transaction(&self, transaction_id: u64) -> Option<&Transaction> {
       let tx =  self.transactions.get(&transaction_id);
         match tx {
          Some(transaction) => Some(&transaction),
          None => None,
         }
    }
    pub fn get_block_id(&self) -> u64 {
        self.block_id
    }   
}

