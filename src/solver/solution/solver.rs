use crate::{
    domain::{Jsp, solution::Schedule},
    solver::stop::StopHandle,
};

pub trait Solver {
    fn solve_with_stop(&self, jsp: &Jsp, stop: &mut StopHandle) -> Option<Schedule>;

    fn solve(&self, jsp: &Jsp) -> Option<Schedule> {
        let mut stop = StopHandle::new();
        self.solve_with_stop(jsp, &mut stop)
    }
}
