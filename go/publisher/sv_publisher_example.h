#ifndef SV_PUBLISHER_EXAMPLE_H
#define SV_PUBLISHER_EXAMPLE_H

#include <stdint.h>

typedef struct SVPublisher* SVPublisher;
typedef struct SVPublisher_ASDU* SVPublisher_ASDU;
typedef union Timestamp* Timestamp;
typedef uint64_t msSinceEpoch;

SVPublisher SVPublisher_create(void* parameters, const char* interfaceId);
SVPublisher_ASDU SVPublisher_addASDU(SVPublisher self, const char* svID, const char* datset, uint32_t confRev);
int SVPublisher_ASDU_addFLOAT(SVPublisher_ASDU self);
int SVPublisher_ASDU_addTimestamp(SVPublisher_ASDU self);
void SVPublisher_setupComplete(SVPublisher self);
uint64_t Hal_getTimeInMs(void);
void Timestamp_clearFlags(Timestamp* self);
void Timestamp_setTimeInMilliseconds(Timestamp* self, msSinceEpoch msTime);
void SVPublisher_ASDU_increaseSmpCnt(SVPublisher_ASDU self);
void SVPublisher_publish(SVPublisher self);
void SVPublisher_destroy(SVPublisher self);
void SVPublisher_ASDU_setFLOAT(SVPublisher_ASDU self, int index, float value);
void SVPublisher_ASDU_setTimestamp(SVPublisher_ASDU self, int index, Timestamp value);

#endif