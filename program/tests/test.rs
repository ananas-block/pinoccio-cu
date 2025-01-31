#![cfg(feature = "test-sbf")]

use rand::Rng;
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use test_program::entrypoint::{InstructionType, NOOP_PUBKEY};

// Total pinoccio: 8,213 CU
// Total solana-program: 9,846 CU
#[tokio::test]
async fn test_cpis() {
    let mut rng = rand::thread_rng();
    let program_id = Pubkey::new_unique();
    let mut context = ProgramTest::new("test_program", program_id, None);
    context.add_program("spl_noop", Pubkey::new_from_array(NOOP_PUBKEY), None);
    let (mut banks_client, payer, recent_blockhash) = context.start().await;

    let (pda_signer, bump) = Pubkey::try_find_program_address(&[&[1]], &program_id).unwrap();
    let pubkey1 = Pubkey::create_program_address(&[&[1], &[255]], &program_id).unwrap();
    assert_eq!(pda_signer, pubkey1);
    println!("{:?}", bump);

    // cpi bench 8 bytes
    // invoke checked
    {
        let data: Vec<u8> = (0..8).map(|_| rng.gen()).collect();
        println!("bench cpis 8 bytes");
        let instruction = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(Pubkey::new_from_array(NOOP_PUBKEY), false),
                AccountMeta::new(pda_signer, false),
            ],
            data: vec![vec![InstructionType::CpiBench as u8], data].concat(),
        };
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    // cpi bench 1024 bytes
    {
        let data: Vec<u8> = (0..3240).map(|_| rng.gen()).collect();
        println!("bench cpis 3240 bytes");
        let instruction = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(Pubkey::new_from_array(NOOP_PUBKEY), false),
                AccountMeta::new(pda_signer, false),
            ],
            data: vec![vec![InstructionType::CpiBench as u8], data].concat(),
        };
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}

#[ignore]
#[tokio::test]
async fn test() {
    let program_id = Pubkey::new_unique();
    let mut context = ProgramTest::new("test_program", program_id, None);
    context.add_program("spl_noop", Pubkey::new_from_array(NOOP_PUBKEY), None);
    let (mut banks_client, payer, recent_blockhash) = context.start().await;

    let (pda_signer, bump) = Pubkey::try_find_program_address(&[&[1]], &program_id).unwrap();
    let pubkey1 = Pubkey::create_program_address(&[&[1], &[255]], &program_id).unwrap();
    assert_eq!(pda_signer, pubkey1);
    println!("{:?}", bump);

    // noop
    {
        // 1,408 CU 512 bytes -> noop
        // 1,410 CU 1,024 bytes -> noop
        // 1,446 CU 10,024 bytes -> noop
        let instruction = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(Pubkey::new_from_array(NOOP_PUBKEY), false),
            ],
            data: vec![vec![0], vec![1u8; 10024]].concat(),
        };
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
    // bench try into vs copy 8
    {
        let instruction = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(Pubkey::new_from_array(NOOP_PUBKEY), false),
            ],
            data: vec![vec![1], vec![7u8; 8]].concat(),
        };
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
    // bench try into vs copy 128
    {
        let instruction = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(Pubkey::new_from_array(NOOP_PUBKEY), false),
            ],
            data: vec![vec![2], vec![7u8; 128]].concat(),
        };
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
    // bench u64 from bytes
    {
        println!("bench from bytes");
        let instruction = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(Pubkey::new_from_array(NOOP_PUBKEY), false),
            ],
            data: vec![vec![3], vec![1, 0, 0, 0, 0, 0, 0, 1]].concat(),
        };
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
    // bench u64 from bytes
    {
        println!("bench from bytes");
        let instruction = Instruction {
            program_id: program_id,
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(Pubkey::new_from_array(NOOP_PUBKEY), false),
            ],
            data: vec![4],
        };
        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
