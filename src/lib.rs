pub(crate) mod colors;
pub(crate) mod commands;
pub(crate) mod components;
pub(crate) mod frac_logic;
pub(crate) mod fractals;
pub(crate) mod helpers;

mod app_state;
pub(crate) use app_state::AppState;

mod app;
pub use app::App;

mod logging;
pub use logging::VERSION;

