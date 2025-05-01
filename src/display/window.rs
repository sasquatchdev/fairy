use std::{cell::{Cell, RefCell}, ffi::CStr, rc::Rc};

use gl33::GlFns;
use glfw::{fail_on_errors, Context, Glfw, PWindow};

use crate::{Error, Result};

/// function to initialize the `display::window` module,
/// this intern initializes the `glfw` library and creates
/// a window.
pub fn init() -> Result<(Glfw, PWindow, Events, GlFns)> {
    log::info!("initializing display::window.");

    let mut glfw = init_glfw()?;
    let (mut window, events) = init_glfw_window(&mut glfw)?;
    let gl = init_gl(&mut window)?;

    log::info!("display::window initialized.");

    Ok((glfw, window, events, gl))
}

pub fn glfw_error_callback(_err: glfw::Error, description: String) {
    log::error!("glfw error: {}", description);
}

pub const GLFW_MAJOR_VERSION: u32 = 3;
pub const GLFW_MINOR_VERSION: u32 = 3;

fn init_glfw() -> Result<Glfw> {
    log::info!("initializing glfw.");
    
    // init glfw
    let mut handle = glfw::init(fail_on_errors!())?;
    log::debug!("glfw (handle) initialized.");

    // set err callback
    handle.set_error_callback(glfw_error_callback);
    log::debug!("glfw error callback set.");

    // get and log glfw version
    let version = glfw::get_version();
    log::info!("glfw version: {}.{}.{}", version.major, version.minor, version.patch);

    // set window hints
    handle.window_hint(glfw::WindowHint::ContextVersionMajor(GLFW_MAJOR_VERSION));
    handle.window_hint(glfw::WindowHint::ContextVersionMinor(GLFW_MINOR_VERSION));
    handle.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    log::debug!("glfw window hints set.");

    log::info!("glfw initialized.");

    Ok(handle)
}

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;
pub const TITLE: &str = "fairy engine";
pub const MODE: glfw::WindowMode = glfw::WindowMode::Windowed;

pub type Events = glfw::GlfwReceiver<(f64, glfw::WindowEvent)>;

fn init_glfw_window(glfw: &mut Glfw) -> Result<(PWindow, Events)> {
    log::info!("initializing glfw window.");

    // create window
    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, TITLE, MODE)
        .ok_or_else(|| {
            log::error!("failed to create glfw window");
            Error::GlfwWindow
        })?;
    
    log::debug!("glfw window created.");

    // make context current
    window.make_current();
    log::debug!("glfw window context made current.");

    // enable polling
    window.set_all_polling(true);

    // set swap interval (i.e. enable v-sync)
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    log::debug!("glfw window swap interval set.");

    // set callbacks
    // ...

    Ok((window, events))
}

fn init_gl(window: &mut PWindow) -> Result<GlFns> {
    log::info!("initializing open-gl.");

    // wrap window in ref cell
    let window = Rc::new(RefCell::new(window));
    let count = Cell::new(0);

    // load open-gl functions
    let gl = unsafe {
        GlFns::load_from(&|s| {
            let name = CStr::from_ptr(s as *const i8);
            let name = name.to_str().unwrap();
            count.set(count.get() + 1);
            window.borrow_mut().get_proc_address(name)
        })
    }.map_err(|err| {
        log::error!("failed to load open-gl functions: {}", err);
        Error::GlInit { context: err.to_string() }
    })?;
    log::debug!("{} open-gl functions loaded.", count.get());

    log::info!("open-gl gl initialized.");

    Ok(gl)
}
