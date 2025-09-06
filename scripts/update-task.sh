#!/bin/bash

grpcurl -plaintext \
  -import-path ./proto \
  -proto ./proto/gsdx/v1/gsdx.proto \
  -d '{"task_id": "0054bb43-e279-4d1a-92a6-1dc29eb87dd0", "status": "TASK_STATUS_COMPLETE"}' \
  "[::]:9090" \
  gsdx.v1.GsdxService/UpdateTask | jq -r
