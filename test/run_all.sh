#! /usr/bin/env bash
set -euxo pipefail
(cd class_implements && ./build.sh)
(cd class_method && ./build.sh)
(cd find_lib_name && ./build.sh)
(cd jni_raw_java && ./build.sh)
(cd primitive_types && ./build.sh)
(cd simple_conversion && ./build.sh)
(cd static_method && ./build.sh)
