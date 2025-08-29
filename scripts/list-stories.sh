#!/bin/bash

grpcurl -plaintext \
  -import-path ./proto \
  -proto ./proto/gsdx/v1/gsdx.proto \
  -d '{"cursor": 1, "limit": 100}' \
  "[::]:9090" \
  gsdx.v1.GsdxService/ListStories | jq -r
