#! /usr/bin/env bash
set -euxo pipefail
(cd find_lib_name && ./build.sh)
(cd static_method && ./build.sh)
(cd class_method && ./build.sh)
(cd class_implements && ./build.sh)
(cd jni_raw_java && ./build.sh)
