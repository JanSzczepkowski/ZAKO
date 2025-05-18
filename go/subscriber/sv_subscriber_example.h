#ifndef SV_SUBSCRIBER_EXAMPLE_H
#define SV_SUBSCRIBER_EXAMPLE_H

#include <stdint.h>

typedef struct SVReceiver* SVReceiver;
typedef struct SVSubscriber* SVSubscriber;
typedef struct SVSubscriber_ASDU* SVSubscriber_ASDU;

SVReceiver SVReceiver_create();
void SVReceiver_setInterfaceId(SVReceiver receiver, const char* iface);
void SVReceiver_addSubscriber(SVReceiver receiver, SVSubscriber subscriber);
void SVReceiver_start(SVReceiver receiver);
int SVReceiver_isRunning(SVReceiver receiver); 
void SVReceiver_stop(SVReceiver self);
void SVReceiver_destroy(SVReceiver receiver);

SVSubscriber SVSubscriber_create(void* parameter, uint16_t appId);
void SVSubscriber_setListener(SVSubscriber subscriber, void (*callback)(SVSubscriber, void*, SVSubscriber_ASDU), void* parameter);

int SVSubscriber_ASDU_getDataSize(SVSubscriber_ASDU self);
float SVSubscriber_ASDU_getFLOAT32(SVSubscriber_ASDU self, int index);
uint16_t SVSubscriber_ASDU_getSmpCnt(SVSubscriber_ASDU self);
uint32_t SVSubscriber_ASDU_getConfRev(SVSubscriber_ASDU self);
const char* SVSubscriber_ASDU_getSvId(SVSubscriber_ASDU self);

#endif
