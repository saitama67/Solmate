use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    log::sol_log_compute_units,
    account_info::{ next_account_info, AccountInfo },
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::io::ErrorKind::InvalidData;
#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct TopicVote{
    pub topic_name: String,
    pub vote_status: String
}
// vote status 0 no decision, 1 if in favour , 2 if in against
const DUMMY_TOPIC: &str = "000000000000000000000000000";
const DUMMY_VOTE : &str = "000000000000000000000000000";

pub fn get_init_vote()->TopicVote{
    let data=TopicVote{topic_name:String::from(DUMMY_TOPIC),vote_status:String::from(DUMMY_VOTE)};
    return data

}
pub fn get_init_votes_number(newslots:u32)->Vec<TopicVote>{
    let mut topics =Vec::new();
    for _ in 0..newslots{
        topics.push(get_init_vote());
    }
    return topics;
}

entrypoint!(process_instruction);
// &program_id, &accounts, &instruction_data
fn process_instruction(
    program_id: &Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]

)->ProgramResult
{
    // Iterating accounts is safer then indexing
    let iter_accounts = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(iter_accounts)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("error program can edit account");
    }

    let  instruction_message = TopicVote::try_from_slice(instruction_data).map_err(|err| {
        msg!("Attempt to deserialize instruction data has failed. {:?}", err);
        ProgramError::InvalidInstructionData
    })?;


    let mut existing_votes = match <Vec<TopicVote>>::try_from_slice(&account.data.borrow_mut()){
        Ok(data) => data,
        Err(err) => {
            if err.kind() == InvalidData {
                msg!("first time votes init memory for votes");
                // no semi colon it terminates the thing and data here new place holders do not go back
                get_init_votes_number(2)
            } else {
                panic!("some decoding error")
            }
        }
    };
    msg!("earlier values here {:?}",existing_votes);
    // if instruction_message.vote_status==1 || instruction_message.vote_status==2 {
    //     panic!("invalid choice made either choose yes (1) or no(2)");
    // }
        // gets the dummy topic,this is the index where we can write our stuff
    let latest_free_index= existing_votes.iter().position(|topic| topic.topic_name==String::from(DUMMY_TOPIC)).unwrap();
    msg!("the free index {}",latest_free_index);

    existing_votes[latest_free_index]=instruction_message;

    let updated_data = existing_votes.try_to_vec().expect("Failed to encode data.");
    msg!("the problem is here {:?} ",existing_votes);
    msg!("Attempting save data {:?}.",updated_data.len());
    let data=&mut &mut account.data.borrow_mut();
    data[..updated_data.len()].copy_from_slice(&updated_data);
    let saved_data =  <Vec<TopicVote>>::try_from_slice(data)?;
    msg!("ChatMessage has been saved to account data. {:?}", saved_data[latest_free_index]);

    // msg!("ChatMessage has been saved to account data. {:?}", saved_data);
    sol_log_compute_units();
    // panic!("noice");
    msg!("End program.");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    #[test]
    fn trying_test() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let messages = get_init_votes_number(1);
        let mut data = messages.try_to_vec().unwrap();
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default()
        );
        let topic ="000200000000000000000000000";
        let vote= "000200000000000000000000000";
        let topic_vote= TopicVote{topic_name:String::from(topic),vote_status:String::from(vote)};
        let topic_vote= topic_vote.try_to_vec().unwrap();
        let accounts=vec![account];
        process_instruction(&program_id,&accounts,&topic_vote).unwrap();
        let vote = &<Vec<TopicVote>>::try_from_slice(&accounts[0].data.borrow())
        .unwrap()[0];
        let topic_name = &vote.topic_name;
        let topic_vote_status = &vote.vote_status;
        println!(" this is topic name {:?}",topic_name);
        println!(" this is topic status{:?}",topic_vote_status);
        // assert_eq!(String::from(topic_name).eq("Superman"))

        // let mut newTopic= TopicVote{TopicName:"I have low spice tolerance",VoteStatus:2};
    }
}