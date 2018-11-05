#! /usr/bin/env bash
set -euxo pipefail
(cd class_implements/mylib && cargo clean)
(cd class_method/mylib && cargo clean)
(cd find_lib_name/mylib && cargo clean)
(cd jni_raw_java/mylib && cargo clean)
(cd primitive_types/mylib && cargo clean)
(cd simple_conversion/mylib && cargo clean)
(cd static_method/mylib && cargo clean)
