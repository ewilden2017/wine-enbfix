use std::fs;
use std::io::BufWriter;
use std::sync::{Mutex, OnceLock};

use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

// https://samrambles.com/guides/window-hacking-with-rust/creating-a-window-with-rust/index.html#refactoring-create_window

// Interface to wrapped library.
static LIBRARY: OnceLock<libloading::Library> = OnceLock::new();
static mut WRITER_GUARD: Option<WorkerGuard> = None;

const DLL_NAME: &str = "C:\\windows\\system32\\d3d11.dll";

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => setup_logging(),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    true
}

fn setup_logging() {
    let file = fs::File::create("dllinject.log").unwrap();
    let (writer, guard) = tracing_appender::non_blocking(BufWriter::new(file));
    // Save the guard statically, which might lose some log message on program exit, but allows
    // message to be logged otherwise.
    // Safety: WRITER_GUARD is only accessed once.
    unsafe {
        WRITER_GUARD = Some(guard);
    }

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(writer)
        .init();

    tracing::info!("Subscriber Installed");
}

fn load_library() -> libloading::Library {
    unsafe { libloading::Library::new(DLL_NAME).unwrap() }
}

fn library() -> &'static libloading::Library {
    LIBRARY.get_or_init(load_library)
}

pub mod export {
    #![allow(non_snake_case, unused_variables, clippy::too_many_arguments)]
    use libloading::Symbol;
    use tracing::instrument;

    use crate::library;

    use std::ffi::c_uint;

    use windows::core::HRESULT;
    use windows::Win32::Foundation::HMODULE;
    use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL};
    use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11DeviceContext};
    use windows::Win32::Graphics::Dxgi::{
        IDXGIAdapter, IDXGIFactory, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC,
    };

    macro_rules! instrument_symbol {
        ($name:ident( $($arg:ident: $at:ty),* $(,)?)) => {
            #[no_mangle]
            #[instrument]
            pub extern "C" fn $name($($arg: $at),*) {
                unsafe {
                    let func: Symbol<unsafe extern "C" fn($($at),*)> =
                        library().get(stringify!($name).as_bytes()).unwrap();
                    func($($arg),*);
                }
                tracing::trace!("ret=()");
            }
        };
        ($name:ident( $($arg:ident: $at:ty),* $(,)?) -> $rt:ty) => {
            #[no_mangle]
            #[instrument]
            pub extern "C" fn $name($($arg: $at),*) -> $rt {
                    let ret = unsafe {
                    let func: Symbol<unsafe extern "C" fn($($at),*) -> $rt> =
                        library().get(stringify!($name).as_bytes()).unwrap();
                    func($($arg),*)
                };
                tracing::trace!(%ret);
                ret
            }
        };
    }

    instrument_symbol!(D3D11CoreCreateDevice(
            pFactory: *mut IDXGIFactory,
            pAdapter: *mut IDXGIAdapter,
            DriverType: D3D_DRIVER_TYPE,
            Software: HMODULE,
            Flags: c_uint,
            pFeatureLevels: *const D3D_FEATURE_LEVEL,
            FeatureLevels: c_uint,
            SDKVersion: c_uint,
            ppDevice: *mut *mut ID3D11Device,
            pFeatureLevel: *mut D3D_FEATURE_LEVEL,
    ) -> HRESULT);

    // instrument_symbol!(D3D11CoreCreateLayeredDevice());
    // instrument_symbol!(D3D11CoreGetLayeredDeviceSize());
    // instrument_symbol!(D3D11CoreRegisterLayers());

    instrument_symbol!(D3D11CreateDevice(
        arg1: *mut IDXGIAdapter,
        arg2: D3D_DRIVER_TYPE,
        arg3: HMODULE,
        arg4: c_uint,
        arg5: *const D3D_FEATURE_LEVEL,
        arg6: c_uint,
        arg7: c_uint,
        arg8: *mut *mut ID3D11Device,
        arg9: *mut D3D_FEATURE_LEVEL,
        arg10: *mut *mut ID3D11DeviceContext,
    ) -> HRESULT);

    instrument_symbol!(D3D11CreateDeviceAndSwapChain(
        adapter: *mut IDXGIAdapter,
        driver_type: D3D_DRIVER_TYPE,
        swrast: HMODULE,
        flags: c_uint,
        feature_levels: *const D3D_FEATURE_LEVEL,
        levels: c_uint,
        sdk_version: c_uint,
        swapchain_desc: *const DXGI_SWAP_CHAIN_DESC,
        swapchain: *mut *mut IDXGISwapChain,
        device: *mut *mut ID3D11Device,
        obtained_feature_level: *mut D3D_FEATURE_LEVEL,
        immediate_context: *mut *mut ID3D11DeviceContext,
    ) -> HRESULT);
}
