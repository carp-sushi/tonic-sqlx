#!/bin/bash

grpcurl -plaintext \
  -import-path ./proto \
  -proto ./proto/gsdx/v1/gsdx.proto \
  -d '{"task_id": "8543fe7c-6482-4087-9933-44ab655a25fc", "status": "incomplete"}' \
  "[::]:9090" \
  gsdx.v1.GsdxService/UpdateTask | jq -r
