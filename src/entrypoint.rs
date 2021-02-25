use solana_program::{
    program_error::ProgramError,
    account_info::{next_account_info, AccountInfo,IntoAccountInfo},
    entrypoint, entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
    system_instruction::transfer,
    program::{invoke, invoke_signed},
    instruction::{AccountMeta, Instruction},
    system_program

};
use byteorder::{ByteOrder, LittleEndian};
use std::convert::TryInto;
use std::assert_eq;
use std::mem;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use std::str::FromStr;
//use std::str;
//use nom::number::complete::recognize_float;

pub struct store {
    // is this the right account ? I might hardcode this
    //pub initialized: bool,
    pub is_initialized: bool,
    pub program_id: Pubkey,
    // store of secrets to check eligibility of withdrawls
    pub secrets: [ [u8; 32]; 10],
    //pub secret_owners: [Pubkey; 10],
    pub current_index: usize,
    pub amount: [u32; 10],
}
impl  store {
    pub fn deposit(&mut self, secret: &[u8], amount: u32){

        self.secrets[self.current_index].copy_from_slice(&secret[0..32]);
        //self.secret_owners[self.current_index] = secret_owner;
        self.amount[self.current_index] = amount;
        msg!("new amount {:?} ", self.amount);

        self.current_index =( self.current_index + 1 )% 10;
        msg!("new index {} ", self.current_index);
    }
    pub fn withdraw(&mut self, secret: &[u8], amount: u32, index: usize){
        assert_eq!(self.secrets[index], secret);
        msg!("Secrets match");
        //assert_eq!(self.secret_owners[index], secret_owner);
        //msg!("Secret Owners match");
        assert!(self.amount[index] >= amount, "requested amount is higher than balance");
        self.amount[index] = self.amount[index] - amount;
        msg!("new amount = {} ", self.amount[index]);
    }
}

entrypoint!(process_instruction);
fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8])
    -> ProgramResult {
        process(program_id, accounts, instruction_data)
}



pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {


    //initialize contract by creating a new storage account
/*
    let seed = "vault";
    let storage_acc = Pubkey::create_with_seed(&program_id, &seed, &program_id);

    if accounts.len() == 1 {
        create_account_with_seed(

        )
    }
    */

    // Expect the secret, the position of the secret in the array, the amount,
    //let instruction;

    let account = &mut accounts.iter();
    let account1 = next_account_info(account)?;
    let account2 = next_account_info(account)?;

    //msg!("accounts1 {:?}", account1);
    //msg!("accounts2 {:?}", account2);

    //unpack instruction
    let mut hash = [0;32];
    hash.copy_from_slice(&instruction_data[0..32]);

    //msg!("{:?}", hash);
    let mut amount_arr = [0; 4];
    amount_arr.copy_from_slice(&instruction_data[32..36]);
    let amount = u32::from_le_bytes(amount_arr);
    //msg!("{}", amount);

    //unpack storage account data
    //assert!(programId == account2.owner);
    msg!("account 2 {:?}", account2);

    let mut data = store::unpack(&account2.data.borrow())?;

    data.program_id = *program_id;
    msg!("Starting with index {}", data.current_index);

    //msg!("Data of storage account {:?}", data.secrets[data.current_index -1]);

    if instruction_data[36] as u8 == 1 {
        if(data.current_index >= 10 as usize) {
            data.current_index = 0 as usize;
        }

        //msg!("starting deposit");
        //msg!("amount after {:?}", data.amount[data.current_index]);
        //msg!("amount requested {}", amount);

        data.deposit(&hash, amount);
        //msg!("New data {:?}", data.secrets[data.current_index - 1]);
        //msg!("amount after {:?}", data.amount[data.current_index-1]);

        store::pack_into_slice(&data, &mut account2.data.borrow_mut());

    }
    //withdraw
    else if instruction_data[36] as u8 == 0 {
        msg!("starting withdrawl");
        msg!(" Checking at index {}", 1);
        let system_program_acc = next_account_info(account)?;
        data.withdraw(&hash, amount, 1 as usize);
        msg!("withdrawl successful");
        let seed = "vaultx1";

        let program_acc = Pubkey::create_with_seed(account1.key,&seed, program_id)?;
        //let program_acc = Pubkey::find_program_address(&[b"vaultx1"], program_id);
        msg!(" account owned by program {}", program_acc);
        store::pack_into_slice(&data, &mut account2.data.borrow_mut());
        let withdraw_tx = transfer(
            &account2.key, &account1.key, amount.into());
            /*
        let withdraw_tx = transfer(
            account2.key,
            account1.key,
            amount.into(),
        );
        */
        /*
        msg!("withdraw tx {:?}", withdraw_tx);
        msg!(" seed as bytes {:?} ", program_id.to_bytes());
        //msg!("program acc {:?}", program_acc);

        //let programid = Pubkey::from_str(&"invoker111111111111111111111111111111111111").unwrap();
        let swap_bytes = seed.as_bytes();
        msg!(" bytes {:?}", swap_bytes);
        let authority_signature_seeds = [&swap_bytes[..], &account1.key.to_bytes(), &program_id.to_bytes()];
        let signers = &[&authority_signature_seeds[..]];
        msg!(" signers {:?}", signers);


        invoke_signed (
            &withdraw_tx,
            &[
                account2.clone(),
                account1.clone(),
                system_program_acc.clone(),
                ]
            ,
            signers,
        )?;
        */

        **account2.try_borrow_mut_lamports()? -= 100000000;
        // Deposit five lamports into the destination
        **account1.try_borrow_mut_lamports()? += 100000000;

    }


    msg!(
        "process_instruction: {}: nr {} accounts data len {}, {:?}, amount {:?}",
        program_id,
        accounts.len(),
        instruction_data.len(),
        hash,
        amount,
    );

    msg!(
        "process_instruction: {}: nr {} accounts data len {},",
        program_id,
        accounts.len(),
        instruction_data.len(),
    );

    Ok(())
}
impl Sealed for store{}
impl IsInitialized for store {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for store{
    const LEN: usize = 401;
    fn unpack_from_slice(input:  &[u8]) ->  Result<Self, ProgramError>{
        let input = array_ref![input, 0, store::LEN];
        let (
            is_initialized,
            programId,
            secret,
            c_i,
            amnt,
        ) = array_refs![input,1, 32, 320, 8, 40];
        //msg!("almost completed array unpacking eg. {:?} {:?}", Pubkey::new_from_array(*programId), usize::from_le_bytes(*c_i));
        //let tmp_secrets = array_ref![secret, 0, 320];

        let mut tmp1 = [ [0 as u8; 32]; 10];
        let mut i = 0;
        for it in secret.chunks(32) {
            tmp1[i].copy_from_slice(it);
            i += 1;
            //msg!("i = {}")
        }

        let mut tmp2 = [0 as u32; 10];
        i = 0;
        for it in amnt.chunks(4) {
            tmp2[i] = LittleEndian::read_u32(it);
            i += 1;
        }
        msg!("amounts before  {:?}", tmp2);
        Ok(
            store {
                is_initialized: true,
                program_id: Pubkey::new_from_array(*programId),
                secrets: tmp1,
                current_index: usize::from_le_bytes(*c_i),
                amount: tmp2,
            }
        )
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        msg!("using pack_into_slice");
        let dst = array_mut_ref![dst, 0, store::LEN];
        let (
            is_initialized_dst,
            programId_dst,
            secret_dst,
            c_i_dst,
            amnt_dst,
        ) = mut_array_refs![dst,1, 32, 320, 8, 40];

        let store {
            is_initialized,
            program_id,
            secrets,
            current_index,
            amount,
        } = self;
        let mut tmp1 = [ 0 as u8; 320];


        //let mut it = 0;
        let mut j = 0;

        for it in secrets {
                let mut i =0;
                //msg!("writing secrets {:?}", it);

                for i in it.chunks(1){
                    tmp1[j] = i[0];
                    //msg!("{}", i[0]);
                    j+=1;
                }



        }

        //msg!("secrets worked");

        //msg!("amounts before {:?}", amount);

        let mut tmp2 = [0 as u8; 40];
        let mut i = 0;
        let mut x = 0;
        loop {
            if i >= 40 {
                break;
            }
            //msg!("writing amount {}", amount[x]);
            LittleEndian::write_u32(&mut tmp2[i..i+4],amount[x]);
            //msg!("writing amount {:?}", amount[x]);

            x+=1;
            i +=4;
        }
        //msg!("amounts after {:?}", tmp2);

        //msg!("saving index {}", self.current_index);
        is_initialized_dst[0]= *is_initialized as u8;
        //msg!("init {:?}", is_initialized[0]);
        programId_dst.copy_from_slice(program_id.as_ref());
        //msg!("init {:?}", programId_dst);
        secret_dst.copy_from_slice(tmp1.as_ref());
        //msg!("init {:?}", secret_dst);

        *c_i_dst = usize::to_le_bytes(*current_index);
        //msg!("current_index {:?}", c_i_dst);

        amnt_dst.copy_from_slice(tmp2.as_ref());
        //msg!("amount {:?}", amnt_dst);

        //msg!("test {:?}",programId_dst);

}
}
