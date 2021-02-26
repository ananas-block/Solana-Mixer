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
//use std::convert::TryInto;
use std::assert_eq;
//use std::mem;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
//use std::str::FromStr;
use bigint::uint::U256;
//use std::collections::HashMap;
use sha2::{Sha256};
pub struct store {
    // is this the right account ? I might hardcode this
    pub is_initialized: bool,
    pub program_id: Pubkey,
    pub denominated_amount: u64,
    pub amount: u64,
    // store of commitments to check eligibility of withdrawls
    pub commitments: [ [u8; 32]; 16],
    pub current_index: usize,
}
/*
pub struct merkle_tree {
    pub is_initialized: bool,
    pub levels: u32,
    pub filledSubtrees : [U256; 32],
    pub zeros : [U256; 32],
    pub currentRootIndex : usize,
    pub nextIndex : u32,
    pub ROOT_HISTORY_SIZE : u32,
    pub roots : [U256; 10],
}
*/
impl  store {
    pub fn deposit(&mut self, commitment: &[u8], amount: u64, account: &AccountInfo){
        assert_eq!(amount, self.denominated_amount);
        if( self.amount == 0){
             self.amount = **account.lamports.borrow() - self.denominated_amount;
        }

        msg!("was there a transfer in ? saved balance {} actual balance {:?}", self.amount, account.lamports.borrow());
        assert!(self.amount  == **account.lamports.borrow() - self.denominated_amount);

        //self.amount = account.lamports;
        let mut exists = false;
        //for &mut  it in self.commitments.iter()
        let mut j = 0;
        loop {
            //msg!("seaching for commitment ");
            if(j == 16) {
                break;
            }
            if(U256::from(self.commitments[j]) == U256::from(commitment)){
                exists = true;
                msg!("commitments exists");
                self.commitments[j].copy_from_slice(&[0 as u8; 32]);
                break;
            }
            j +=1;
        }
        assert!(!exists);

        self.amount += self.denominated_amount;

        self.commitments[self.current_index].copy_from_slice(&commitment[0..32]);
        self.current_index =( self.current_index + 1 )% 16;
        msg!("new index {} ", self.current_index);
        msg!("new amount {} ,real {}", self.amount, account.lamports.borrow());

        //add_to_merkle_tree(secret);
    }
    pub fn withdraw(&mut self, commitment: &[u8]){
        let mut exists = false;
        //for &mut  it in self.commitments.iter()
        let mut j = 0;
        loop {
            //msg!("seaching for commitment ");
            if(j == 16) {
                break;
            }
            if(U256::from(self.commitments[j]) == U256::from(commitment)){
                exists = true;
                msg!("commitments exists");
                self.commitments[j].copy_from_slice(&[0 as u8; 32]);
                self.amount -= self.denominated_amount;
                break;
            }
            j +=1;
        }
        assert!(exists);
        msg!("commitments exists");
        //self.commitments.get_mut(commitment) = false;
        //msg!("new amount = {} ", self.amount[index]);
    }
}

entrypoint!(process_instruction);
fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8])
    -> ProgramResult {
        process(program_id, accounts, instruction_data)
}



pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {

    // Expect the secret, the position of the secret in the array, the amount,
    //let instruction;
    //let FIELD_SIZE
    let FIELD_SIZE: U256 = U256::from("21888242871839275222246405745257275088548364400416034343698204186575808495617".as_bytes());
    let ZERO_VALUE: U256 = U256::from("21663839004416932945382355908790599225266501822907911457504978515578255421292".as_bytes()); // = keccak256("tornado") % FIELD_SIZE

    let account = &mut accounts.iter();
    let account1 = next_account_info(account)?;
    let account2 = next_account_info(account)?;

    //msg!("accounts1 {:?}", account1);
    //msg!("accounts2 {:?}", account2);

    //unpack instruction
    let mut hash = [0;32];
    hash.copy_from_slice(&instruction_data[0..32]);

    msg!("{:?}", hash);
    let mut amount_arr = [0; 8];
    amount_arr.copy_from_slice(&instruction_data[32..40]);
    let amount = u64::from_le_bytes(amount_arr);
    msg!("{}", amount);

    //unpack storage account data
    //assert!(programId == account2.owner);
    msg!("account 2 {:?}", account2);

    let mut data = store::unpack(&account2.data.borrow())?;
    data.denominated_amount = 1000000000;
    data.program_id = *program_id;
    //msg!("Starting with index {}", data.current_index);

    //msg!("Data of storage account {:?}", data.commitments[data.current_index -1]);

    if instruction_data[40] as u8 == 1 {


        //msg!("starting deposit");
        //msg!("amount after {:?}", data.amount[data.current_index]);
        //msg!("amount requested {}", amount);

        data.deposit(&hash, amount, account2);
        //msg!("New data {:?}", data.commitments[data.current_index - 1]);
        //msg!("amount after {:?}", data.amount[data.current_index-1]);

        store::pack_into_slice(&data, &mut account2.data.borrow_mut());

    }
    //withdraw
    else if instruction_data[40] as u8 == 0 {
        msg!("starting withdrawl");
        msg!(" Checking at index {}", 1);
        //let system_program_acc = next_account_info(account)?;
        data.withdraw(&hash);
        msg!("withdrawl successful");
        let seed = "vaultx1";

        let program_acc = Pubkey::create_with_seed(account1.key,&seed, program_id)?;
        //let program_acc = Pubkey::find_program_address(&[b"vaultx1"], program_id);
        msg!(" account owned by program {}", program_acc);
        store::pack_into_slice(&data, &mut account2.data.borrow_mut());
        let withdraw_tx = transfer(
            &account2.key, &account1.key, amount.into());


        **account2.try_borrow_mut_lamports()? -= u64::from(data.denominated_amount);

        **account1.try_borrow_mut_lamports()? += u64::from(data.denominated_amount);

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


fn initMerkletree(){

}


impl Sealed for store{}
impl IsInitialized for store {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for store{
    const LEN: usize = 569;
    fn unpack_from_slice(input:  &[u8]) ->  Result<Self, ProgramError>{
        let input = array_ref![input, 0, store::LEN];
        let (
            is_initialized,
            programId,
            d_amnt,
            amnt,
            cmts,
            c_i,
        ) = array_refs![input,1, 32, 8, 8, 512, 8];// HashMap (32 + 1) * 32

        //let mut commitments: HashMap<[u8;64], bool> = HashMap::with_capacity(16);
        //pub commitments: [[u8; 64]; 16]
        msg!("almost completed array unpacking eg. {}", LittleEndian::read_u64(amnt));
        let mut commitments = [[0 as u8; 32];16];
        let mut i = 0;
        for it in cmts.chunks(32) {
            commitments[i].copy_from_slice(it);//.insert(it[0..64], it[64]);
            //msg!("added = with key {:?}", commitments[i]);
            i += 1;
        }

        Ok(
            store {
                is_initialized: true,
                denominated_amount: LittleEndian::read_u64(d_amnt),
                program_id: Pubkey::new_from_array(*programId),
                amount: LittleEndian::read_u64(amnt),
                commitments: commitments,
                current_index: usize::from_le_bytes(*c_i),
            }
        )
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        msg!("using pack_into_slice");
        let dst = array_mut_ref![dst, 0, store::LEN];
        let (
            is_initialized_dst,
            programId_dst,
            denominated_amount_dst,
            amount_dst,
            commitments_dst,
            current_index_dst,
        ) = mut_array_refs![dst,1, 32, 8, 8, 512, 8];

        let store {
            is_initialized,
            program_id,
            denominated_amount,
            amount,
            commitments,
            current_index,
        } = self;
        let mut tmp_commitments = [ 0 as u8; 512];


        let mut j = 0;
        for it in commitments.iter() {
            for i in it.iter(){
                commitments_dst[j] = *i;
                j += 1;
            }
        }

        /*
        for (key, val) in commitments.iter() {
                dst_commitments[j..j+64].copy_from_slice(key);
                dst_commitments[j+64] = val.copy().into();
                //msg!("writing commitments {:?}, {}", dst_commitments[j..j+64], dst_commitments[j+64]);
                j +=65;
        }*/

        msg!("commitments worked");


        //msg!("saving index {}", self.current_index);
        is_initialized_dst[0]= *is_initialized as u8;
        msg!("init {}", is_initialized_dst[0]);
        programId_dst.copy_from_slice(program_id.as_ref());
        //msg!("init {:?}", programId_dst);
        //msg!("init {:?}", secret_dst);
        //msg!("current_index {:?}", c_i_dst);
        //commitments_dst.copy_from_slice(tmp)
        LittleEndian::write_u64(amount_dst, *amount);
        LittleEndian::write_u64(denominated_amount_dst, *denominated_amount);
        *current_index_dst = usize::to_le_bytes(*current_index);

        msg!("amount {:?} denominated_amount {:?}", amount_dst, denominated_amount_dst);

}
}
