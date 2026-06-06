

mod blockchain;
use blockchain::block::Block;
use blockchain::transaction::Transaction;
use blockchain::status::{TransactionStatus}; 

use std::collections::HashMap;

use crate::blockchain::block;


fn main() {
    let mut block1 = Block::new(
        1,
    HashMap::from([
        (
        1,
         Transaction::new(
            1,
            100,
            String::from("Aman Khan"),
            String::from("John Doe"),
            1634567890,
            TransactionStatus::Pending,
        ))])
    );
    block1.print();
    let is_added = block1.add_transaction(Transaction::new(
        2,
        50,
        String::from("Jane Doe"),
        String::from("Aman Khan"),
        1634567891,
        TransactionStatus::Pending,
    ) );
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
    if let Some(transaction ) = block1.get_transaction_mut(1) {
        transaction.transaction_status(TransactionStatus::Completed);
    }
    if let Some(transaction) = block1.get_transaction_mut(2)  {
        transaction.transaction_status(TransactionStatus::Completed);
    }
    if let Some(transaction) = block1.get_transaction_mut(3)  {
        transaction.transaction_status(TransactionStatus::Completed);
    }
    block1.validate_block();
       
    block1.print();
    println!("------------------All transactions in the block:---------------------------------");
    block1.get_block_all_transactions().iter().for_each(|tx| tx.print());
    println!("------------------------Getting transaction with ID 2:--------------------------------");
    block1.get_transaction(2).map(|tx : &Transaction| tx.print());
    println!("Block ID: {}", block1.get_block_id() );
    
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
