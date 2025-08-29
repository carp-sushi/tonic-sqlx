#!/bin/bash

grpcurl -plaintext \
  -import-path ./proto \
  -proto ./proto/gsdx/v1/gsdx.proto \
  -d '{"task_id": "c98dd4ac-6255-481e-b8b0-e67521e81c32"}' \
  "[::]:9090" \
  gsdx.v1.GsdxService/DeleteTask | jq -r
