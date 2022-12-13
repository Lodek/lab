use super::MemberId;
use super::BallotId;
use super::Proposal;
use super::PaxosState;
use super::InstanceId;
use super::Vote;

/// PaxosFollower models a follower member of paxos.
/// The behavior of a follower are roughly the following:
/// - Receive query for last vote cast
/// - Receive request to deny casting votes for ballots before a ballotId
/// - Decide on decree
pub struct PaxosFollower<T> {
    state: PaxosState<T>,
}

impl<T> PaxosFollower<T> {

    pub fn new(member: MemberId, instance_id: InstanceId) -> Self {
        let state = PaxosState::new(member, instance_id);
        Self {
            state: state
        }
    }

    /// Check whether the given ballot id is eligible to be the new promise.
    pub fn can_promise(&self, ballot_id: BallotId) -> bool {
        self.state.promise
            .map(|id| ballot_id >= id)
            .unwrap_or(true) // lacking a promise, it can promise anythign
    }

    /// Check whether the given ballot id is eligible to receive
    /// the instances vote, that is, it does not violate a previous promise.
    /// This function panics if the instance has made not previous promises,
    /// as voting without a promise violates the protocol
    pub fn can_vote(&self, ballot_id: BallotId) -> bool {
        if self.state.promise.is_none() {
            panic!("follower has made no promise hence cannot vote: instance_id = {} member_id = {}", self.state.instance_id, self.state.member);
        }

        let promise_id = self.state.promise.as_ref().unwrap();
        &ballot_id >= promise_id
    }

    /// Return whether Paxos instance has reached consensus
    pub fn has_decided(&self) -> bool {
        self.state.decree.is_some()
    }
}

impl<T: Clone> PaxosFollower<T> {
    
    /// Set a new promise for the Follower.
    /// Setting a promises creates a new follower whose state is the same
    /// as the original one, except for the promise.
    pub fn make_promise(&self, ballot_id: BallotId) -> Option<PaxosFollower<T>> {
        if !self.can_promise(ballot_id) {
            return None;
        }

        let mut state = self.state.clone();
        state.promise = Some(ballot_id);
        let instance = PaxosFollower {
            state: state
        };
        Some(instance)
    }

    /// Return decided decree for instance
    pub fn get_decided_decree(&self) -> Option<T> {
        self.state.decree.clone()
    }

    /// Cast a vote for the received proposal.
    /// If the proposal can be accepted by the current instance,
    /// return an option containing a vote, otherwise return None
    pub fn cast_vote(&mut self, proposal: &Proposal<T>) -> Option<Vote> {
        if !self.can_vote(proposal.ballot) {
            return None;
        }

        let vote = Vote {
            ballot: proposal.ballot,
            voter: self.state.member,
        };

        self.state.last_vote = Some(vote);
        self.state.last_voted_decree = Some(proposal.decree.clone());

        Some(vote)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const leader_id: MemberId = 10;

    fn make_id(id: u64) -> BallotId {
        BallotId {
            id: id,
            initiator: leader_id,
        }
    }

    #[test]
    fn test_new_follower_can_promise() {
        let follower = PaxosFollower::<u64>::new(1, 1);
        let id = make_id(10);

        let can_promise = follower.can_promise(id);

        assert!(can_promise);
    }

    #[test]
    fn test_cast_vote_return_expected_vote() {
        let follower = PaxosFollower::new(1, 1);
        let id = make_id(10);
        let mut follower = follower.make_promise(id).unwrap();

        let prop = Proposal {
            ballot: id,
            decree: 1,
        };

        let vote_opt = follower.cast_vote(&prop);
        assert!(vote_opt.is_some());

        let expected = Vote {
            ballot: id,
            voter: follower.state.member,
        };
        assert_eq!(expected, vote_opt.unwrap());

    }

    #[test]
    fn test_follower_cannot_promise_for_ballot_less_than_current_promise() {
        let follower = PaxosFollower::new(1, 1);
        let id = make_id(10);
        let mut follower = follower.make_promise(id).unwrap();

        let id = make_id(5);
        let prop = Proposal {
            ballot: id,
            decree: 2,
        };
        let vote_opt = follower.cast_vote(&prop);
        assert!(vote_opt.is_none());
    }

    #[test]
    fn test_follower_promise_return_new_follower_with_correct_promise() {
        let follower = PaxosFollower::<u64>::new(1, 1);
        let id = make_id(10);

        let new_follower = follower.make_promise(id);

        assert_eq!(follower.state.promise, None);
        assert_eq!(new_follower.unwrap().state.promise, Some(id));
    }
}
