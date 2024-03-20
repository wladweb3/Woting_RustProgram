#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Error {
    CandidateAlreadyExists,
    CandidateNotFound,
    NotAuthorized,
    AlreadyVoted,
    NoVoteRecorded,
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct VotingContract {
    candidates: Vec<(String, u32)>,
    voters: Vec<String>,
}

impl VotingContract {
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
            voters: Vec::new(),
        }
    }

    pub fn propose_candidate(&mut self, candidate: String) -> Result<()> {
        if self.candidates.iter().any(|(name, _)| name == &candidate) {
            return Err(Error::CandidateAlreadyExists);
        }
        self.candidates.push((candidate, 0));
        Ok(())
    }

    pub fn vote_for_candidate(&mut self, voter: String, candidate: String) -> Result<()> {
        if self.voters.contains(&voter) {
            return Err(Error::AlreadyVoted);
        }
        if let Some((_, votes)) = self
            .candidates
            .iter_mut()
            .find(|(name, _)| name == &candidate)
        {
            *votes += 1;
            self.voters.push(voter);
            Ok(())
        } else {
            Err(Error::CandidateNotFound)
        }
    }

    pub fn get_candidate_votes(&self, candidate: &str) -> Result<u32> {
        if let Some((_, votes)) = self
            .candidates
            .iter()
            .find(|(name, _)| name == candidate)
        {
            Ok(*votes)
        } else {
            Err(Error::CandidateNotFound)
        }
    }

    pub fn get_winner(&self) -> Result<&str> {
        if let Some((name, _)) = self
            .candidates
            .iter()
            .max_by_key(|&(_, votes)| votes)
        {
            Ok(name)
        } else {
            Err(Error::NoVoteRecorded)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voting_contract() {
        let mut contract = VotingContract::new();
        contract.propose_candidate("Candidate A".to_string()).unwrap();
        contract.propose_candidate("Candidate B".to_string()).unwrap();
        contract.propose_candidate("Candidate C".to_string()).unwrap();

        assert_eq!(contract.vote_for_candidate("Voter1".to_string(), "Candidate A".to_string()), Ok(()));
        assert_eq!(contract.vote_for_candidate("Voter2".to_string(), "Candidate B".to_string()), Ok(()));
        assert_eq!(contract.vote_for_candidate("Voter3".to_string(), "Candidate C".to_string()), Ok(()));
        assert_eq!(contract.vote_for_candidate("Voter4".to_string(), "Candidate A".to_string()), Ok(()));

        assert_eq!(contract.get_candidate_votes("Candidate A"), Ok(2));
        assert_eq!(contract.get_candidate_votes("Candidate B"), Ok(1));
        assert_eq!(contract.get_candidate_votes("Candidate C"), Ok(1));

        assert_eq!(contract.get_winner(), Ok("Candidate A"));
    }
}
