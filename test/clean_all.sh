#! /usr/bin/env bash
set -euxo pipefail
(cd find_lib_name/mylib && cargo clean)
(cd static_method/mylib && cargo clean)
(cd class_method/mylib && cargo clean)
(cd class_implements/mylib && cargo clean)
