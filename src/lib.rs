mod anttweakbar;

use std::fs;
use std::io::BufWriter;
use std::sync::OnceLock;

use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

// https://samrambles.com/guides/window-hacking-with-rust/creating-a-window-with-rust/index.html#refactoring-create_window

// Interface to wrapped library.
static LIBRARY: OnceLock<libloading::Library> = OnceLock::new();
static mut WRITER_GUARD: Option<WorkerGuard> = None;

const DLL_NAME: &str = "d3d11enb.dll";

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

    use crate::anttweakbar::*;
    use crate::library;

    use std::ffi::{c_char, c_int, c_uint, c_void};

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
                tracing::trace!(?ret);
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

    instrument_symbol!(D3D11CoreCreateLayeredDevice(arg1: i64, arg2: i64, arg3: i64, arg4: i64, arg5: i64) -> i64);
    instrument_symbol!(D3D11CoreGetLayeredDeviceSize(arg1: i64, arg2: i64) -> u64);
    instrument_symbol!(D3D11CoreRegisterLayers(arg1: i64, arg2: i64) -> u64);

    instrument_symbol!(DirtyHack(arg1: u64, arg2: u64) -> bool);

    instrument_symbol!(ENBGetGameIdentifier() -> u64);
    instrument_symbol!(ENBGetParameter(arg1: i64, arg2: i64, arg3: i64, arg4: i64, arg5: i64) -> i32);
    instrument_symbol!(ENBGetRenderInfo() -> *mut c_void);
    instrument_symbol!(ENBGetSDKVersion());
    instrument_symbol!(ENBGetState(arg1: u64) -> u32);
    instrument_symbol!(ENBGetVersion() -> u64);
    instrument_symbol!(ENBSetCallbackFunction(arg1: i64));
    instrument_symbol!(ENBSetParameter(arg1: i64, arg2: i64, arg3: i64, arg4: u64) -> u64);

    // instrument_symbol!(NvOptimusEnablement()); // This is a global variable

    instrument_symbol!(TwAddButton(
        bar: *mut TwBar,
        name: *const c_char,
        callback: TwButtonCallback,
        clientData: *mut c_void,
        def: *const c_char,
    ) -> c_int);

    instrument_symbol!(TwAddSeparator(bar: *mut TwBar, name: *const c_char, def: *const c_char) -> c_int);
    instrument_symbol!(TwAddVarCB(
        bar: *mut TwBar,
        name: *const c_char,
        type_: TwType,
        setCallback: TwSetVarCallback,
        getCallback: TwGetVarCallback,
        clientData: *mut c_void,
        def: *const c_char,
    ) -> c_int);
    instrument_symbol!(TwAddVarRO(
        bar: *mut TwBar,
        name: *const c_char,
        type_: TwType,
        var: *const c_void,
        def: *const c_char,
    ) -> c_int);

    instrument_symbol!(TwAddVarRW(
        bar: *mut TwBar,
        name: *const c_char,
        type_: TwType,
        var: *mut c_void,
        def: *const c_char,
    ) -> c_int);
    instrument_symbol!(TwCopyCDStringToClientFunc(copyCDStringFunc: TwCopyCDStringToClient));
    instrument_symbol!(TwCopyCDStringToLibrary(
        destinationLibraryStringPtr: *mut *mut c_char,
        sourceClientString: *const c_char,
    ));

    instrument_symbol!(TwCopyStdStringToClientFunc(copyStdStringToClient: TwCopyStdStringToClient));
    instrument_symbol!(TwCopyStdStringToLibrary(
        destinationLibraryString: *mut c_void,
        sourceClientString: *const c_void,
    ));

    instrument_symbol!(TwDefine(def: *const c_char) -> c_int);
    instrument_symbol!(TwDefineEnum(
        name: *const c_char,
        enumValues: *const TwEnumVal,
        nbValues: c_uint,
    ) -> TwType);
    instrument_symbol!(TwDefineEnumFromString(
        name: *const c_char,
        enumString: *const c_char,
    ) -> TwType);
    instrument_symbol!(TwDefineStruct(
        name: *const c_char,
        structMembers: *const TwStructMember,
        nbMembers: c_uint,
        structSize: usize,
        summaryCallback: TwSummaryCallback,
        summaryClientData: *mut c_void,
    ) -> TwType);
    instrument_symbol!(TwDeleteAllBars() -> c_int);
    instrument_symbol!(TwDeleteBar(bar: *mut TwBar) -> c_int);
    instrument_symbol!(TwDraw() -> c_int);
    instrument_symbol!(TwEventWin(
        wnd: *mut c_void,
        msg: c_uint,
        wParam: c_uint,
        lParam: c_int,
    ) -> c_int);
    instrument_symbol!(TwEventWin32(
        wnd: *mut c_void,
        msg: c_uint,
        wParam: c_uint,
        lParam: c_int,
    ) -> c_int);

    instrument_symbol!(TwGetBarByIndex(barIndex: c_int) -> *mut TwBar);
    instrument_symbol!(TwGetBarByName(barName: *const c_char) -> *mut TwBar);
    instrument_symbol!(TwGetBarCount() -> c_int);
    instrument_symbol!(TwGetBarName(bar: *const TwBar) -> *const c_char);
    instrument_symbol!(TwGetBottomBar() -> *mut TwBar);
    instrument_symbol!(TwGetCurrentWindow() -> c_int);
    instrument_symbol!(TwGetLastError() -> *const c_char);
    instrument_symbol!(TwGetParam(
        bar: *mut TwBar,
        varName: *const c_char,
        paramName: *const c_char,
        paramValueType: TwParamValueType,
        outValueMaxCount: c_uint,
        outValues: *mut c_void,
    ) -> c_int);
    instrument_symbol!(TwGetTopBar() -> *mut TwBar);
    instrument_symbol!(TwHandleErrors(errorHandler: TwErrorHandler));
    instrument_symbol!(TwInit(
        graphAPI: TwGraphAPI,
        device: *mut c_void,
    ) -> c_int);
    instrument_symbol!(TwKeyPressed(
        key: c_int,
        modifiers: c_int,
    ) -> c_int);
    instrument_symbol!(TwKeyTest(
        key: c_int,
        modifiers: c_int,
    ) -> c_int);
    instrument_symbol!(TwMouseButton(action: TwMouseAction, button: TwMouseButtonID) -> c_int);
    instrument_symbol!(TwMouseMotion(
        mouseX: c_int,
        mouseY: c_int,
    ) -> c_int);
    instrument_symbol!(TwMouseWheel(pos: c_int) -> c_int);
    instrument_symbol!(TwNewBar(barName: *const c_char) -> *mut TwBar);
    instrument_symbol!(TwRefreshBar(bar: *mut TwBar) -> c_int);
    instrument_symbol!(TwRemoveAllVars(bar: *mut TwBar) -> c_int);
    instrument_symbol!(TwRemoveVar(
        bar: *mut TwBar,
        name: *const c_char,
    ) -> c_int);

    // instrument_symbol!(TwSetBarFontSize());

    instrument_symbol!(TwSetBottomBar(bar: *const TwBar) -> c_int);
    instrument_symbol!(TwSetCurrentWindow(windowID: c_int) -> c_int);
    instrument_symbol!(TwSetParam(
        bar: *mut TwBar,
        varName: *const c_char,
        paramName: *const c_char,
        paramValueType: TwParamValueType,
        inValueCount: c_uint,
        inValues: *const c_void,
    ) -> c_int);
    instrument_symbol!(TwSetTopBar(bar: *const TwBar) -> c_int);
    instrument_symbol!(TwTerminate() -> c_int);
    instrument_symbol!(TwWindowExists(windowID: c_int) -> c_int);
    instrument_symbol!(TwWindowSize(
        width: c_int,
        height: c_int,
    ) -> c_int);
}
