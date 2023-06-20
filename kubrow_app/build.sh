#!/bin/bash

export CPATH="/usr/lib/clang/15.0.7/include:$CPATH"

flutter_rust_bridge_codegen \
  --rust-input rust/src/api.rs \
  --dart-output ./lib/bridge_generated.dart \
  --dart-decl-output ./lib/bridge_definitions.dart
