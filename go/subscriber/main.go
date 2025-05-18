package main

/*
#cgo CFLAGS: -I../../src
#cgo LDFLAGS: -L../../build -liec61850
#include "sv_subscriber_example.h"
#include <stdio.h>
#include <signal.h>
#include <stdbool.h>

void svUpdateListener(SVSubscriber subscriber, void* parameter, SVSubscriber_ASDU asdu) {
    printf("svUpdateListener called\n");

    const char* svID = SVSubscriber_ASDU_getSvId(asdu);
    if (svID != NULL) {
        printf("  svID=(%s)\n", svID);
    }

    printf("  smpCnt: %i\n", SVSubscriber_ASDU_getSmpCnt(asdu));
    printf("  confRev: %u\n", SVSubscriber_ASDU_getConfRev(asdu));
    if (SVSubscriber_ASDU_getDataSize(asdu) >= 8) {
        printf("   DATA[0]: %f\n", SVSubscriber_ASDU_getFLOAT32(asdu, 0));
        printf("   DATA[1]: %f\n", SVSubscriber_ASDU_getFLOAT32(asdu, 4));
    }
}

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
	receiver := C.SVReceiver_create()
	C.SVReceiver_setInterfaceId(receiver, C.CString("veth0"))

	subscriber := C.SVSubscriber_create(nil, 0x4000)

	C.SVSubscriber_setListener(subscriber, (*[0]byte)(C.svUpdateListener), nil)

	C.SVReceiver_addSubscriber(receiver, subscriber)
	C.SVReceiver_start(receiver)

	if C.SVReceiver_isRunning(receiver) != C.int(0) {
		C.signal(C.SIGINT, (*[0]byte)(C.sigint_handler))

        for C.running {
            time.Sleep(1 * time.Millisecond)
        }
		C.SVReceiver_stop(receiver)
	} else {
		fmt.Println("Failed to start SV subscriber. Reason can be that the Ethernet interface doesn't exist or root permission are required.\n")
	}

	C.SVReceiver_destroy(receiver)
}
