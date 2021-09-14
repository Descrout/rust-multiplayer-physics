#!/bin/sh
# Generate both js and rust protobuf codes from one .proto file.
pb-rs --dont_use_cow -o server/src/proto/proto-all.rs proto-all.proto
pbf proto-all.proto --browser > client/proto-all.js