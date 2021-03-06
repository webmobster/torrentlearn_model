use FitnessEvaluator;
use parse::ParseTree;
use parse::SpecialOperator;
use std::str::FromStr;
use std::fmt;
use rand::Rng;
use std::sync::Arc;

/// Used to contain the information shared with the operator provided, wrapped in an RC
/// to keep track of uses and when the operator provider can drop whatever is holding the
/// code the function pointer references
pub trait DropHelper: Send {
    fn trait_clone(&self) -> Box<DropHelper>;
}


//FIXME: Look into how dropping can be done better in respects to dropflags and trait objects

pub trait OperatorProvider
{
    // dynamic dispatch as no paramitzed types in a trait
    fn random(&mut self, rng: &mut Rng) -> Operator;
    fn random_with_successors(&mut self, rng: &mut Rng, suc: u8) -> Operator;
    fn combine(&mut self, parts: Vec<ParseTree>) -> Operator;
    fn split(&mut self, parts: ParseTree, point: usize) -> (Operator, Operator);

}

pub struct Operator {
    pub special: SpecialOperator,
    pub successors: u8,
    pub cost: u64,
    // FIXME
    pub op: fn(&mut [u64]) -> bool,
    //FIXME: Look into how dropping can be done better in respects to dropflags and trait objects
    pub drop_helper: Option<Box<DropHelper>>,
    pub parts: Option<Arc<ParseTree>>,
}
impl Operator {
    pub fn get_sucessors(&self) -> u8 {
        self.successors
    }
    pub fn cost(&self) -> u64 {
        self.cost
    }
    pub fn get_special(&self) -> SpecialOperator {
        self.special
    }
}

impl Clone for Operator {
    fn clone(&self) -> Operator {
        Operator {
            cost: self.cost,
            special: self.special,
            op: self.op,
            successors: self.successors,
            drop_helper: match &self.drop_helper{
                &None => None,
                &Some(ref x) => Some(x.trait_clone())
            },
            parts: self.parts.clone(),
        }
    }
}

// FIXME
impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Operator")
         .field("special", &self.special)
         .field("sucessors", &self.successors)
         .finish()
    }
}

// TODO: UUID string representation should be a fixed size hexadecimal string for both debug and
// display
#[derive(Clone,Debug,Copy,Eq,PartialEq,Hash)]
pub struct UUID {
    pub x: [u64; 2],
}
impl FromStr for UUID {
    type Err = ();

    #[inline]
    #[allow(deprecated)]
    fn from_str(src: &str) -> Result<Self, ()> {
        let mut split = src.split(",");
        let part1 = split.next().unwrap();
        let part2 = split.next().unwrap();
        Ok(UUID { x: [part1.parse::<u64>().unwrap(), part2.parse::<u64>().unwrap()] })
    }
}

impl fmt::Display for UUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.x[0], self.x[1])
    }
}
