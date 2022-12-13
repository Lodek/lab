use std::time;
use std::cmp;

pub mod follower;

type InstanceId = u64;

type MemberId = u64;

/// BallotId represents the identifier for a Ballot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct BallotId {
    id: u64,
    initiator: MemberId,
}

impl cmp::PartialOrd<Self> for BallotId {

    /// Ballot comparasion is given by comparing ids,
    /// initiator is a tie breaker.
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.id > other.id {
            Some(cmp::Ordering::Greater)
        }
        else if self.id < other.id {
            Some(cmp::Ordering::Less)
        }
        else {
            cmp::PartialOrd::partial_cmp(&self.initiator, &other.initiator)
        }
    }
}


/// Vote represents a vote cast by a protocol member during a Ballot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vote {
    ballot: BallotId,
    voter: MemberId,
}

/// Proposal represents a decree proposal sent by a leader during a Ballot.
struct Proposal<T> {
    ballot: BallotId,
    decree: T,
}

/// Ballot is a voting round which attempts to reach consensus
struct Ballot<T> {
    id: BallotId,
    decree: Option<T>,
    initiator: MemberId,
    votes: Vec<Vote>,
}

/// PaxosState represents an instance of the Synod protocol.
#[derive(Clone)]
struct PaxosState<T> {
    /// Unique identifier for this member in the paxos quorum
    /// (ie current host identifier)
    pub member: MemberId,

    /// Unique identifier for this instance of the synod protocol.
    /// (effectively this represents the id of the decree in the complete ledger of decrees)
    pub instace_id: InstanceId,

    /// Promise made to a proposer of the minimum BallotId in which this instance will vote.
    pub promise: Option<BallotId>,

    /// Last vote cast by this instance in a previous ballot
    pub last_vote: Option<Vote>,

    /// Decree proposed for last voted ballot
    pub last_voted_decree: Option<T>,

    /// Decree which was decided for this instance
    pub decree: Option<T>,
}

impl<T> PaxosState<T> {
    pub fn new(member: MemberId, instace_id: InstanceId) -> Self {
        return Self{
            promise: None,
            last_vote: None,
            last_voted_decree: None,
            decree: None,
            member,
            instace_id,
        }
    }
}

/*

struct PaxosLeader<T> {
    state: PaxosState<T>

    /// Id of the last Ballot this instance has attempted to initiate
    last_attempt: Option<Ballot<T>>,

    tentative_decree: T,
    quorum_size: u8,

    // Maybe store the set of previous votes?
}

impl<T> PaxosLeader<T> {

    /// Build next ballot id and set its internal state
    pub fn init_next_ballot() -> BallotId {

    }

    /// Builds a new ballot from the set of final votes cast by its peers.
    /// Invariant: Only builds ballot if the numbers of votes constitutes 
    /// a quorum majority.
    pub fn begin_ballot(previous_votes: &[]Vote<T>) -> Option<Ballot<T>> {
        // 
    }

    /// Return whether the set of previous votes is enough to continue with a ballot
    pub fn can_build_ballot(previous_votes: &[]Vote<T>) -> bool {

    }

    /// Return the vote for the highest ballot number in the set of previous votes
    fn gat_latest_vote(previous_votes: &[]Vote<T>) -> Vote<T> {

    }
    
    /// Determines the decree for the next ballot by looking
    /// for the latest vote cast by any instance and replicating its
    /// decree.
    /// If there is no previous vote, use tetnative_decree(?)
    fn get_ballot_decree(previous_votes: &[]Vote<T>) -> T {

    }

    /// Return true if the set of received responses for the begin ballot
    /// stage has lead to a consensus
    pub fn has_consensus<R>(beging_ballot_reponses: &[]R) -> bool {
        // TODO check data and len of responses
        // should be more than majority
    }

    // Persist data and set decision in instance
    pub fn decide() -> Resut<()> {

    }

}

trait InstanceSerializer<T> {
    fn serialize(instance: &PaxosInstance<T>) -> Vec<u8>;
    fn deserialize(bytes: &[]u8) -> PaxosInstance<T>;
}

trait ProposerService {
    fn next_ballot();
    fn begin_ballot();
    fn decide();
}

type Host = String;

/// Wraps a ProposerService to broadcast messages to all members in the quorum.
/// Returns accumulated responses until either it receives a response from the majority
/// or the messages timeout.
struct BroadcastService<S: ProposerService> {
    service: S,
    quorum: Vec<String>,
    timeout: time::Duration,
}

impl<S> BroadcastService<S> {

    fn broadcast_next_ballot(&self) {
    }

    fn broadcast_begin_ballot(&self) {
    }

    fn broadcast_decide(&self) {
    }
}

enum LeaderState {
    Initial,
    StartingBallot,
    // TODO
    
}

/// state machine executor for the paxos leader
/// initiates and attempts consensus until
/// it reaches consensus or execution is aborted (either a crash or a new leader)
struct LeaderCoordinator<T> {
    leader: PaxosLeader<T>,
}

impl LeaderCoordinator {
    fn run() {}
    fn terminate() {}
}
*/

// TODO there should be a state machine for the leader which is responsible for
// executing the steps accordingly


// rpc calls:
// nextballot
// beginballot
// decide
//
// next steps: 
// think about the rpc calls and which functions I need in my structs
// in order to implement the RPC calls
// like: next_ballot might be better suited as a vote request
// which means the instance would likely receive a ballot and decide on whether or not to vote
// based on the internal promisse
//
// next_ballot would essentially cast a promise - first stage in a two phase commit - 
// therefore it would return its last vote and update the internal promise

// this has been some good progress
// i am happy with the overall shape it is taking, there's a good separation of concerns
// each role logic is encapsulated in the struct
// some consideration has started to be given towards the RPC calls

// I am quite deep in the consensus thinking now but maybe I should draw the overall library
// interfaces and start structuring things a bit better
//
//
// overall structure:
//
// leader coordinator -mutates-> leader -manages and contains-> paxos state
// leader coordinator -consumes-> broadcast service -consumes-> leader service
//
// state machine / coordinator for follower and learner as well
//
// general state machine which receives signals to execute an instance (?)
// batchable service for the main paxos coordinator which will manages all instances
// 
// this general interface approach is quite good because it allows me to work with a simpler mental
// model and for performance reasons in the future it could be relatively well suited through a
// channel implementation / event loop
// making instances as independent as possible
//
//
