extern "C" {
    fn SVPublisher_create(src_mac: *const u8, interface_id: *const i8) -> *mut std::ffi::c_void;
    fn SVPublisher_addASDU(
        publisher: *mut std::ffi::c_void,
        sv_id: *const i8,
        dat_set: *const i8,
        smp_mod: u8,
    ) -> *mut std::ffi::c_void;
    fn SVPublisher_ASDU_addFLOAT(asdu: *mut std::ffi::c_void) -> i32;
    fn SVPublisher_ASDU_addTimestamp(asdu: *mut std::ffi::c_void) -> i32;
    fn SVPublisher_setupComplete(publisher: *mut std::ffi::c_void);
    fn SVPublisher_ASDU_setFLOAT(asdu: *mut std::ffi::c_void, index: i32, val: f32);
    fn SVPublisher_ASDU_setTimestamp(asdu: *mut std::ffi::c_void, index: i32, timestamp: Timestamp);
    fn SVPublisher_ASDU_increaseSmpCnt(asdu: *mut std::ffi::c_void);
    fn SVPublisher_publish(publisher: *mut std::ffi::c_void);
    fn SVPublisher_destroy(publisher: *mut std::ffi::c_void);

    fn Hal_getTimeInMs() -> u64;
    fn Thread_sleep(seconds: u32);
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Timestamp {
    seconds: u32,
    nanoseconds: u32,
    accuracy: u8,
    flags: u8,
}

extern "C" {
    fn Timestamp_clearFlags(t: *mut Timestamp);
    fn Timestamp_setTimeInMilliseconds(t: *mut Timestamp, ms: u64);
}

use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    /*ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");*/

    let interface = CString::new("veth0").unwrap();
    println!("Using interface veth0");

    unsafe {
        let publisher = SVPublisher_create(std::ptr::null(), interface.as_ptr());

        if publisher.is_null() {
            println!("Failed to create SV publisher");
            return;
        }

        let asdu1 = SVPublisher_addASDU(publisher, CString::new("svpub1").unwrap().as_ptr(), std::ptr::null(), 1);
        let float1 = SVPublisher_ASDU_addFLOAT(asdu1);
        let float2 = SVPublisher_ASDU_addFLOAT(asdu1);
        let ts1 = SVPublisher_ASDU_addTimestamp(asdu1);

        let asdu2 = SVPublisher_addASDU(publisher, CString::new("svpub2").unwrap().as_ptr(), std::ptr::null(), 1);
        let float3 = SVPublisher_ASDU_addFLOAT(asdu2);
        let float4 = SVPublisher_ASDU_addFLOAT(asdu2);
        let ts2 = SVPublisher_ASDU_addTimestamp(asdu2);

        SVPublisher_setupComplete(publisher);

        let mut f_val1 = 1234.5678f32;
        let mut f_val2 = 0.12345f32;

        while running.load(Ordering::SeqCst) {
            let mut ts = Timestamp {
                seconds: 0,
                nanoseconds: 50,
                accuracy: 0,
                flags: 0,
            };

            Timestamp_clearFlags(&mut ts);
            Timestamp_setTimeInMilliseconds(&mut ts, Hal_getTimeInMs());

            SVPublisher_ASDU_setFLOAT(asdu1, float1, f_val1);
            SVPublisher_ASDU_setFLOAT(asdu1, float2, f_val2);
            SVPublisher_ASDU_setTimestamp(asdu1, ts1, ts);

            SVPublisher_ASDU_setFLOAT(asdu2, float3, f_val1 * 2.0);
            SVPublisher_ASDU_setFLOAT(asdu2, float4, f_val2 * 2.0);
            SVPublisher_ASDU_setTimestamp(asdu2, ts2, ts);

            SVPublisher_ASDU_increaseSmpCnt(asdu1);
            SVPublisher_ASDU_increaseSmpCnt(asdu2);

            f_val1 += 1.1;
            f_val2 += 0.1;

            SVPublisher_publish(publisher);
            Thread_sleep(1);
        }

        SVPublisher_destroy(publisher);
    }
}
