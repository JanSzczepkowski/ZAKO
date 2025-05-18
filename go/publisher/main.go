package main

/*
#cgo CFLAGS: -I../../../src
#cgo LDFLAGS: -L../../../build -liec61850
#include <stdio.h>
#include <signal.h>
#include <stdbool.h>
#include "sv_publisher_example.h"

bool running = true;

void sigint_handler(int signalId)
{
    running = 0;
}
*/
import "C"
import (
	"fmt"
	"time"
)

func main() {
	C.signal(C.SIGINT, (*[0]byte)(C.sigint_handler))

	svPublisher := C.SVPublisher_create(nil, C.CString("veth1"))

	if svPublisher != nil {

		asdu1 := C.SVPublisher_addASDU(svPublisher, C.CString("svpub1"), nil, 1)
		
		var float1 C.int = C.SVPublisher_ASDU_addFLOAT(asdu1)
		var float2 C.int = C.SVPublisher_ASDU_addFLOAT(asdu1)
		var ts1 C.int = C.SVPublisher_ASDU_addTimestamp(asdu1)

		asdu2 := C.SVPublisher_addASDU(svPublisher, C.CString("svpub2"), nil, 1)
		var float3 C.int = C.SVPublisher_ASDU_addFLOAT(asdu2)
		var float4 C.int = C.SVPublisher_ASDU_addFLOAT(asdu2)
		var ts2 C.int = C.SVPublisher_ASDU_addTimestamp(asdu2)

		C.SVPublisher_setupComplete(svPublisher)

		var fVal1 C.float = C.float(1234.5678)
		var fVal2 C.float = C.float(0.12345)

		for C.running == true {
			var ts C.Timestamp
			C.Timestamp_clearFlags(&ts)
			C.Timestamp_setTimeInMilliseconds(&ts, C.Hal_getTimeInMs())

			C.SVPublisher_ASDU_setFLOAT(asdu1, float1, fVal1)
			C.SVPublisher_ASDU_setFLOAT(asdu1, float2, fVal2)
			C.SVPublisher_ASDU_setTimestamp(asdu1, ts1, ts)

			C.SVPublisher_ASDU_setFLOAT(asdu2, float3, fVal1 * 2)
			C.SVPublisher_ASDU_setFLOAT(asdu2, float4, fVal2 * 2)
			C.SVPublisher_ASDU_setTimestamp(asdu2, ts2, ts)

			C.SVPublisher_ASDU_increaseSmpCnt(asdu1)
			C.SVPublisher_ASDU_increaseSmpCnt(asdu2)

			fVal1 += 1.1
			fVal2 += 0.1

			C.SVPublisher_publish(svPublisher)

			time.Sleep(50 * time.Millisecond)
		}

		C.SVPublisher_destroy(svPublisher)

	} else {
		fmt.Println("Failed to create SV publisher.\n")
	}
}