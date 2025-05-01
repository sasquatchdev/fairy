/// module that contains common code,
/// which is shared accross the entire
/// project. 
/// this includes things like the error
/// handling system, logging, configuration,
/// etc.
pub mod common;
pub use common::errs::*;
use common::logs;
use glfw::Context;

/// module that contains all built-in
/// components.
/// these include very basic components like
/// transform, camera, etc. but also more
/// advanced components regarding physics,
/// rendering, audio and more.
pub mod components;

/// module that contains code which abstracts
/// away the raw opengl and glfw calls (which are
/// inherently unsafe) and provides a higher level,
/// safe abstraction.
pub mod display;

/// module that contains abstractions around
/// the lifecycle of the engine (and game).
pub mod system;

fn main() -> ResultVoid {
    logs::init()?;
    log::info!("starting up at {}.", logs::time());

    let (mut glfw, mut window, events, _gl) = display::window::init()?;

    while !window.should_close() {
        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => {
                    log::info!("window close event received.");
                    window.set_should_close(true);
                },
                glfw::WindowEvent::Key(key, _, action, _) => {
                    log::info!("key event received: key: {:?}, action: {:?}", key, action);
                    if action == glfw::Action::Press && key == glfw::Key::Escape {
                        window.set_should_close(true);
                    }
                },
                _ => {}
            }
        }
    }

    // ...

    log::info!("shutting down at {}.", logs::time());
    Ok(())
}
