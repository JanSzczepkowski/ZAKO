#!/bin/bash

# Kompilacja pliku C do obiektu
gcc -I/home/patryk-welenc/pg/semVI/ZAKo/projekt/libiec61850-1.6/src \
    -I/home/patryk-welenc/pg/semVI/ZAKo/projekt/libiec61850-1.6/hal/inc \
    -I/home/patryk-welenc/pg/semVI/ZAKo/projekt/libiec61850-1.6/src/sampled_values \
    -I/home/patryk-welenc/pg/semVI/ZAKo/projekt/libiec61850-1.6/src/common/inc \
    -I/home/patryk-welenc/pg/semVI/ZAKo/projekt/libiec61850-1.6/src/mms/inc \
    -I/home/patryk-welenc/pg/semVI/ZAKo/projekt/libiec61850-1.6/src/iec61850/inc \
    -I/home/patryk-welenc/pg/semVI/ZAKo/projekt/libiec61850-1.6/src/logging \
    -I/home/patryk-welenc/pg/semVI/ZAKo/projekt/libiec61850-1.6/src/r_session \
    -c sv_subscriber_example.c -o sv_subscriber_example.o

# Tworzenie biblioteki statycznej
ar rcs libsvwrapper.a sv_subscriber_example.o

# Kompilacja Rust z linkowaniem
rustc main.rs \
  -L . \
  -L ../../build \
  -l static=svwrapper \
  -l static=iec61850 \
  -o sv_subscriber_runner

# Uruchomienie programu
sudo ./sv_subscriber_runner
