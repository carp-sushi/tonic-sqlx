#!/bin/bash

grpcurl -plaintext \
  -import-path ./proto \
  -proto ./proto/gsdx/v1/gsdx.proto \
  -d '{"story_id": "ab5e260f-9996-4980-a8f0-949612eaf241"}' \
  "[::]:9090" \
  gsdx.v1.GsdxService/DeleteStory | jq -r
