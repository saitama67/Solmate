use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    log::sol_log_compute_units,
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,

};

use std::io::ErrorKind::InvalidData;
/// Define the type of state stored in accounts
const DUMMY_TX:&str="0000000000000000000000000000000000000000000000";
const CREATED_ON:&str="0000000000000000";
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct registerMarriage {
    /// number of greetings
    pub certificate: String,
    pub toAccount: String,
    pub createdOn: String

}

  pub fn is_signer(accounts: &[&AccountInfo]) -> ProgramResult {
    for acc in &mut accounts.iter() {
      if !acc.is_signer {
        return Err(ProgramError::InvalidAccountData);
      }
    }
    Ok(())
  }

pub fn registerOnce(data:Pubkey)->registerMarriage{
    return registerMarriage{
        certificate: String::from(DUMMY_TX),
        toAccount:data.to_string(),
        createdOn:String::from(CREATED_ON)

    }
}
// Declare and export the program's entrypoint



entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {

    msg!("welcome to the code base");
    sol_log_compute_units();

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();
    //
    //  Get the account to say hello to
    let account1 = next_account_info(accounts_iter)?;
    let account2=next_account_info(accounts_iter)?;
    self::is_signer(&[account1])?;
    self::is_signer(&[account2])?;

    //
    // // The account must be owned by the program in order to modify its data

    if account1.owner != program_id {
        msg!("not owner by this account");
        return Err(ProgramError::IncorrectProgramId);
    }
    if account2.owner!=program_id{
        msg!("not owned by this account");
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction_data_message = registerMarriage::try_from_slice(_instruction_data).map_err(|err|{
        msg!("problem in deserialization {:?}",err);
        ProgramError::InvalidInstructionData
    })?;
    // let data= &mut&mut account1.data.borrow_mut();
    // let a = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // data.copy_from_slice(&a[..]);
    // msg!("this is the instruction data {:?}",data);
    let mut existing_data1= match registerMarriage::try_from_slice(&account1.data.borrow()){
        Ok(data)=>data,
        Err(err)=>{
            registerOnce(*account2.key)
        }
        };
        let mut existing_data2= match registerMarriage::try_from_slice(&account2.data.borrow()){
        Ok(data)=>data,
        Err(err)=>{
            registerOnce(*account1.key)
        }
        };

    existing_data1.certificate=instruction_data_message.certificate.clone();
    existing_data2.certificate=instruction_data_message.certificate;
    existing_data1.createdOn=instruction_data_message.createdOn.clone();
    existing_data2.createdOn=instruction_data_message.createdOn;
    // msg!("this is data {:?}",existing_data);

    // existing_data.serialize(&mut &mut account1.data.borrow_mut()[..])?;
    // let updated_data= existing_data.try_to_vec().expect("error");
    let account_data1 =&mut &mut account1.data.borrow_mut();
    let account_data2= &mut &mut account2.data.borrow_mut();
    let new_data1=existing_data1.try_to_vec()?;
    let new_data2=existing_data2.try_to_vec()?;
    account_data1[..new_data1.len()].copy_from_slice(&new_data1);
    account_data2[..new_data2.len()].copy_from_slice(&new_data2);

    // account_data.copy_from_slice(account_data[..new_data.len()]);

      msg!("{:?}",new_data1.len());

    // let account_data= &mut&mut account1.data.borrow_mut();
    // account_data.serialize(&mut new_data);

    sol_log_compute_units();
    Ok(())


    //
    // // Increment and store the number of times the account has been greeted
    // let data= &account.data.borrow();
}

    // pub fn process(
    //     program_id: &Pubkey,
    //     accounts: &[AccountInfo],
    //     instruction_data: &[u8],
    // ) -> ProgramResult {
    //
    // }


// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;
    use std::{println as info, println as warn};

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::new_unique();
        let mut lamports1 = 0;
        let mut data1 = vec![0;200];
        let mut lamports2 = 0;
        let mut data2 = vec![0; 200];
        let owner = Pubkey::default();
        let account1 = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports1,
            &mut data1,
            &owner,
            false,
            Epoch::default(),
        );
        let account2 = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports2,
            &mut data2,
            &owner,
            false,
            Epoch::default(),
        );
        let seed:&str="thisissdfasdfasdfasdfasdfsseed";
        // msg!("{:?}",Pubkey::new_unique().to_string());
        let new_pubkey=Pubkey::create_with_seed(&owner,seed,&owner).expect("something");
        let instruction_data: Vec<u8> = registerMarriage{
            certificate:String::from("QmURdqKCWxFRL93gEUtxAibMdegStKxqKSr7DwANwNhEii"),
            createdOn:String::from("0000000000000020"),
            toAccount:new_pubkey.to_string(),
        }.try_to_vec().unwrap();




        let accounts = vec![account1,account2];
        let returned_data=process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        // returned_data();
        let borrow=&accounts[0].data.borrow_mut();

    let finaldata = registerMarriage::try_from_slice(&borrow[..117]);
        msg!("this is the final data {:?}",finaldata);
        // panic!("had to see the resutl here")
         let borrow2=&accounts[1].data.borrow_mut();
        let finaldata2 = registerMarriage::try_from_slice(&borrow2[..117]);
        msg!("this is the final data {:?}",finaldata2);
        panic!("had to see the resutl here")


    }

}
