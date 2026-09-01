use crate::domain::{Jsp, solution::Schedule};

pub trait Solver {
    fn solve(&self, jsp: &Jsp) -> Option<Schedule>;
}
