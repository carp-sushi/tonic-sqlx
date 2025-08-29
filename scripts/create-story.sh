#!/bin/bash

grpcurl -plaintext \
  -import-path ./proto \
  -proto ./proto/gsdx/v1/gsdx.proto \
  -d '{"name": "Books Not To Read"}' \
  "[::]:9090" \
  gsdx.v1.GsdxService/CreateStory | jq -r
