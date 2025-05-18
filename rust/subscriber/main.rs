use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

extern "C" {
    fn SVReceiver_create() -> *mut c_void;
    fn SVReceiver_setInterfaceId(receiver: *mut c_void, interface_id: *const c_char);
    fn SVReceiver_addSubscriber(receiver: *mut c_void, subscriber: *mut c_void);
    fn SVReceiver_start(receiver: *mut c_void);
    fn SVReceiver_stop(receiver: *mut c_void);
    fn SVReceiver_isRunning(receiver: *mut c_void) -> bool;
    fn SVReceiver_destroy(receiver: *mut c_void);

    fn SVSubscriber_create(name: *const c_char, app_id: c_uint) -> *mut c_void;
    fn SVSubscriber_setListener(
        subscriber: *mut c_void,
        listener: extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
        parameter: *mut c_void,
    );

    fn SVSubscriber_ASDU_getSvId(asdu: *mut c_void) -> *const c_char;
    fn SVSubscriber_ASDU_getSmpCnt(asdu: *mut c_void) -> c_int;
    fn SVSubscriber_ASDU_getConfRev(asdu: *mut c_void) -> c_uint;
    fn SVSubscriber_ASDU_getDataSize(asdu: *mut c_void) -> c_int;
    fn SVSubscriber_ASDU_getFLOAT32(asdu: *mut c_void, index: c_int) -> f32;

    fn Thread_sleep(seconds: c_int);

    fn signal(sig: c_int, handler: extern "C" fn(c_int)) -> usize;
}

static mut RUNNING: bool = true;

extern "C" fn sigint_handler(_sig: c_int) {
    unsafe {
        RUNNING = false;
    }
}

extern "C" fn sv_update_listener(
    _subscriber: *mut c_void,
    _parameter: *mut c_void,
    asdu: *mut c_void,
) {
    println!("svUpdateListener called");

    unsafe {
        let sv_id_ptr = SVSubscriber_ASDU_getSvId(asdu);
        if !sv_id_ptr.is_null() {
            let cstr = CStr::from_ptr(sv_id_ptr);
            println!("  svID=({})", cstr.to_string_lossy());
        }

        let smp_cnt = SVSubscriber_ASDU_getSmpCnt(asdu);
        let conf_rev = SVSubscriber_ASDU_getConfRev(asdu);

        println!("  smpCnt: {}", smp_cnt);
        println!("  confRev: {}", conf_rev);

        let data_size = SVSubscriber_ASDU_getDataSize(asdu);
        if data_size >= 8 {
            let data0 = SVSubscriber_ASDU_getFLOAT32(asdu, 0);
            let data1 = SVSubscriber_ASDU_getFLOAT32(asdu, 4);
            println!("   DATA[0]: {}", data0);
            println!("   DATA[1]: {}", data1);
        }
    }
}

fn main() {
    unsafe {
        signal(2, sigint_handler);

        let receiver = SVReceiver_create();

        println!("Using interface veth1");
        let iface = CString::new("veth1").unwrap();
        SVReceiver_setInterfaceId(receiver, iface.as_ptr());

        let subscriber = SVSubscriber_create(ptr::null(), 0x4000);
        SVSubscriber_setListener(subscriber, sv_update_listener, ptr::null_mut());
        SVReceiver_addSubscriber(receiver, subscriber);
        SVReceiver_start(receiver);

        if SVReceiver_isRunning(receiver) {
            while RUNNING {
                Thread_sleep(1);
            }

            SVReceiver_stop(receiver);
        } else {
            println!("Failed to start SV subscriber.");
        }

        SVReceiver_destroy(receiver);
    }
}
