use std::time::Duration;

use crate::domain::{Jsp, solution::Schedule};

pub trait Solver {
    fn solve_with_timeout(&self, jsp: &Jsp, timeout: Option<Duration>) -> Option<Schedule>;

    fn solve(&self, jsp: &Jsp) -> Option<Schedule> {
        self.solve_with_timeout(jsp, None)
    }
}
