#!/bin/bash
set -eux

main() {
  minicom -c on -D /dev/ttyUSB0 -b 115200
}

main "$@"
