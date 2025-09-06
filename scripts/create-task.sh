#!/bin/bash

grpcurl -plaintext \
  -import-path ./proto \
  -proto ./proto/gsdx/v1/gsdx.proto \
  -d '{"story_id": "768d2dfb-6cbc-4d77-8deb-6e3049986d8f", "name": "Test Task 3", "status": "TASK_STATUS_INCOMPLETE"}' \
  "[::]:9090" \
  gsdx.v1.GsdxService/CreateTask | jq -r
