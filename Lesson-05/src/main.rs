use std::{collections::{HashMap, hash_map}, panic};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum BlockStatus {
    InProcess,
    InValid,
    Mined,
    Orphaned,
}

#[derive(Debug)]
struct Transaction {
    transaction_id: u64,
    amount: u64,
    sender: String,
    receiver: String,
    timestamp: u64,
    status: TransactionStatus,
}

#[derive(Debug)]
struct Block {
    block_id: u64,
    transactions: HashMap<u64, Transaction>,
    status: BlockStatus,
}

// Error
enum BlockError {
    BlockFinalized,
}

fn main() {
    let mut block1 = Block::new(
        1,
    HashMap::from([
        (
        1,
         Transaction {
            transaction_id: 1,
            amount: 100,
            sender: String::from("Aman Khan"),
            receiver: String::from("John Doe"),
            timestamp: 1634567890,
            status: TransactionStatus::Pending,
        })])
    );
    block1.print();
    let is_added = block1.add_transaction(Transaction {
        transaction_id: 2,
        amount: 50,
        sender: String::from("Jane Doe"),
        receiver: String::from("Aman Khan"),
        timestamp: 1634567891,
        status: TransactionStatus::Pending,
    });
    if let Err(_) = is_added {
        panic!("Failed to add transaction to the block.");
    }

    block1.print();
    block1.print();
    let not_mined = block1.validate_block();
    if not_mined {
        panic!("Block is not mined yet. All transactions must be completed before mining.");
    }
   let is_added = block1.add_transaction(Transaction::new(
        3,
        25,
        String::from("Alice"),
        String::from("Bob"),
        1634567892,
        TransactionStatus::Pending,
    ));
    if let Err(_) = is_added {
        panic!("Failed to add transaction to the block.");
    }
    block1.print();
    if let Some(transaction) = block1.get_transaction_mut(1) {
        transaction.transaction_status(TransactionStatus::Completed);
    }
    if let Some(transaction) = block1.get_transaction_mut(2) {
        transaction.transaction_status(TransactionStatus::Completed);
    }
    if let Some(transaction) = block1.get_transaction_mut(3) {
        transaction.transaction_status(TransactionStatus::Completed);
    }
    block1.validate_block();
       
    block1.print();
    println!("------------------All transactions in the block:---------------------------------");
    block1.get_block_all_transactions().iter().for_each(|tx| tx.print());
    println!("------------------------Getting transaction with ID 2:--------------------------------");
    block1.get_transaction(2).map(|tx| tx.print());

    let is_added = block1.add_transaction(Transaction::new(
        3,
        25,
        String::from("Alice"),
        String::from("Bob"),
        1634567892,
        TransactionStatus::Pending,
    ));
    if let Err(_) = is_added {
        panic!("Failed to add transaction to the block.");
    }
}

impl Block {
    fn new(block_id: u64, transactions: HashMap<u64, Transaction>) -> Block {
        Block {
            block_id,
            transactions,
            status: BlockStatus::InProcess,
        }
    }

    fn print(&self) {
        println!("{:?} ", self);
    }

    fn add_transaction(&mut self, transaction: Transaction) -> Result<(), BlockError> {
        match (self.status) {
            BlockStatus::InProcess => {
                self.transactions.insert(transaction.transaction_id, transaction);
                Ok(())
            }
            _ => Err(BlockError::BlockFinalized),
        }
    }

    fn validate_block(&mut self) -> bool {
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
    fn get_transaction_mut(&mut self, transaction_id: u64) -> Option<&mut Transaction> {
        for tx in &mut self.transactions.values_mut() {
            if tx.transaction_id == transaction_id {
                return Some(tx);
            }
        }
        None
    }

    fn get_block_all_transactions(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    fn get_transaction(&self, transaction_id: u64) -> Option<&Transaction> {
       let tx =  self.transactions.get(&transaction_id);
         match tx {
          Some(transaction) => Some(&transaction),
          None => None,
         }
    }
}

impl Transaction {
    fn new(
        transaction_id: u64,
        amount: u64,
        sender: String,
        receiver: String,
        timestamp: u64,
        status: TransactionStatus,
    ) -> Transaction {
        Transaction {
            transaction_id,
            amount,
            sender,
            receiver,
            timestamp,
            status,
        }
    }

    fn print(&self) {
        println!("{:?} ", self);
    }

    fn transaction_status(&mut self, new_status: TransactionStatus) {
        self.status = new_status;
    }
}
