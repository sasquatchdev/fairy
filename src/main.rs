/// module that contains common code,
/// which is shared accross the entire
/// project. 
/// this includes things like the error
/// handling system, logging, configuration,
/// etc.
pub mod common;
pub use common::errs::*;
use common::logs;

/// module that contains all built-in
/// components.
/// these include very basic components like
/// transform, camera, etc. but also more
/// advanced components regarding physics,
/// rendering, audio and more.
pub mod components;

/// module that contains code regarding
/// the entity component system (ecs).
/// this includes entities, components and
/// systems and therefore resembles the
/// highest level of abstraction.
pub mod system;

/// module that contains code which abstracts
/// away the raw opengl and glfw calls (which are
/// inherently unsafe) and provides a higher level,
/// safe abstraction.
pub mod display;

fn main() -> ResultVoid {
    logs::init()?;
    log::info!("starting up at {}.", logs::time());

    // ...

    log::info!("shutting down at {}.", logs::time());
    Ok(())
}
