#!/bin/bash
set -eux

main() {
  find . | grep -v /target | grep -v "/\." | entr -s \
    'date && cargo build && cargo build --examples'
}

main "$@"
