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
    system_program,
    hash,
    log::sol_log_compute_units,

};
use byteorder::{ByteOrder, LittleEndian};
use std::assert_eq;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use bigint::uint::U256;

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


impl  store {
    pub fn deposit(&mut self, commitment: &[u8], amount: u64, account: &AccountInfo){
        assert_eq!(amount, self.denominated_amount);
        if( self.amount == 0){
             self.amount = **account.lamports.borrow() - self.denominated_amount;
        }

        //msg!("was there a transfer in ? saved balance {} actual balance {:?}", self.amount, account.lamports.borrow());
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
                //msg!("commitments exists");
                self.commitments[j].copy_from_slice(&[0 as u8; 32]);
                break;
            }
            j +=1;
        }
        assert!(!exists);

        self.amount += self.denominated_amount;

        self.commitments[self.current_index].copy_from_slice(&commitment[0..32]);
        self.current_index =( self.current_index + 1 )% 16;
        //msg!("new index {} ", self.current_index);
        //msg!("new amount {} ,real {}", self.amount, account.lamports.borrow());

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
                //msg!("commitments exists");
                self.commitments[j].copy_from_slice(&[0 as u8; 32]);
                self.amount -= self.denominated_amount;
                break;
            }
            j +=1;
        }
        assert!(exists);
        //msg!("commitments exists");
        //self.commitments.get_mut(commitment) = false;
        //msg!("new amount = {} ", self.amount[index]);
    }
}
/*
pub struct merkle_tree {
    pub is_initialized: bool,
    pub levels: u32,
    pub filledSubtrees : Vec<U256>,
    pub zeros : Vec<U256>,
    pub currentRootIndex : usize,
    pub nextIndex : u32,
    pub ROOT_HISTORY_SIZE : u32,
    pub roots : Vec<U256>,
}

impl merkle_tree {
    pub fn initialize (&mut self, treelevels: u32,mut  zero_value : U256){
        msg!("starting to initialize merkle tree");

        //assert!(treelevels > 0);
        //assert!(treelevels < 32);
        let mut bytes = [0 as u8; 32];
        zero_value.to_little_endian( &mut bytes);
        //let mut z_v = hash::hash(&bytes);
        self.levels = treelevels;
        let mut current_zero = zero_value;


        msg!("Bytes result {:?}",bytes);
        sol_log_compute_units();
        self.zeros[0] = current_zero;
        self.filledSubtrees[0] = current_zero;

        for i in 0..self.levels {
            sol_log_compute_units();
            msg!(" Iteration {}", i);
            //current_zero = hashLeftRight(&bytes, &bytes);
            self.zeros.push(current_zero);
            self.filledSubtrees.push(current_zero);
            zero_value.to_little_endian( &mut bytes);
        }
        self.roots[0] = current_zero;
        msg!("Current Root {}", self.roots[0]);

    }

}

pub fn hashLeftRight(left: &[u8; 32], right: &[u8;32]) -> U256{
    //assert!(U256:from(left) < FIELD_SIZE);
    //assert!(U256:from(right) < FIELD_SIZE);
    //let mut hshr = hash::Hasher();
    let mut bytes = [0 as u8; 32];

    sol_log_compute_units();
    let mut R = U256::from(left.as_ref());
    msg!("first add {}", R);
    sol_log_compute_units();

    let mut C = U256::from(&[0 as u8;32]);
    sol_log_compute_units();

    let x = false;
    //C = hash::hash(C);
    R.overflowing_add(C);
    msg!("first add {}", R);
    R.to_little_endian( &mut bytes);
    let mut hsh = hash::hash(&bytes);
    R = U256::from(hsh.as_ref());
    msg!("hash {:?}", R);
    C = R;
    R.overflowing_add(U256::from(right));// % FIELD_SIZE;
    R.to_little_endian( &mut bytes);
    hsh = hash::hash(&bytes);
    R = U256::from(hsh.as_ref());
    msg!("hash {:?}", R);
    R.overflowing_add(C);
    R.to_little_endian( &mut bytes);
    hsh = hash::hash(&bytes);
    R = U256::from(hsh.as_ref());
    msg!("hash {:?}", R);

    R

}
*/
entrypoint!(process_instruction);
fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8])
    -> ProgramResult {
        process(program_id, accounts, instruction_data)
}



pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {

    // Expect a commitment, and the amount
    //let instruction;
    //let FIELD_SIZE
    //let FIELD_SIZE: U256 = U256::from_little_endian("21888242871839275222246405745257275088548364400416034343698204186575808495617".as_bytes());
    //let zv = String::from("21663839004416932945382355908790599225266501822907911457504978515578255421292");
    //let bytes = [0;32];//"tornado".as_bytes();
    //let ZERO_VALUE :Hasher; // = keccak256("tornado") % FIELD_SIZE
    //let tmp = hash::hash(&bytes);
    //msg!("Hash zero : {:?}", tmp);
    //let ZERO_VALUE = U256::from(tmp.as_ref());
    let account = &mut accounts.iter();
    let account1 = next_account_info(account)?;
    let account2 = next_account_info(account)?;

    //unpack instruction

    let mut commitment = [0;32];
    commitment.copy_from_slice(&instruction_data[0..32]);


    //msg!("{:?}", );
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

        data.deposit(&commitment, amount, account2);
        //msg!("New data {:?}", data.commitments[data.current_index - 1]);
        //msg!("amount after {:?}", data.amount[data.current_index-1]);

        store::pack_into_slice(&data, &mut account2.data.borrow_mut());
        msg!("Deposited {}", data.denominated_amount);

    }
    //withdraw
    else if instruction_data[40] as u8 == 0 {

        data.withdraw(&commitment);
        //let seed = "vaultx1";

        //let program_acc = Pubkey::create_with_seed(account1.key,&seed, program_id)?;
        store::pack_into_slice(&data, &mut account2.data.borrow_mut());
        let withdraw_tx = transfer(
            &account2.key, &account1.key, amount.into());


        **account2.try_borrow_mut_lamports()? -= u64::from(data.denominated_amount);

        **account1.try_borrow_mut_lamports()? += u64::from(data.denominated_amount);
        msg!("withdrawl successful");


    }
    /*
    let mut mt = merkle_tree {is_initialized: true, levels: 3,
        filledSubtrees:vec![U256::from(0);1],
        zeros: vec![U256::from(0);1],
        currentRootIndex: 0,
        nextIndex: 0,
        ROOT_HISTORY_SIZE: 10,
        roots: vec![U256::from(0); 1],
    };

    mt.initialize(3, ZERO_VALUE);
    msg!("initialized merkle tree");
    */

    Ok(())
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

        //msg!("almost completed array unpacking eg. {}", LittleEndian::read_u64(amnt));
        let mut commitments = [[0 as u8; 32];16];
        let mut i = 0;
        for it in cmts.chunks(32) {
            commitments[i].copy_from_slice(it);
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

        is_initialized_dst[0]= *is_initialized as u8;
        msg!("init {}", is_initialized_dst[0]);
        programId_dst.copy_from_slice(program_id.as_ref());

        LittleEndian::write_u64(amount_dst, *amount);
        LittleEndian::write_u64(denominated_amount_dst, *denominated_amount);
        *current_index_dst = usize::to_le_bytes(*current_index);

        msg!("amount {:?} denominated_amount {:?}", amount_dst, denominated_amount_dst);

}
}
