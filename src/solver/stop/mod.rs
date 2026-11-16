//! Condiciones de parada y presupuesto de búsqueda.

mod cancel;
mod search_budget;
mod search_control;
mod stop_condition;
mod stop_handle;
mod timeout;

pub use cancel::CancelCondition;
pub use search_budget::SearchBudget;
pub use search_control::SearchControl;
pub use stop_condition::StopCondition;
pub use stop_handle::StopHandle;
pub use timeout::TimeoutCondition;
