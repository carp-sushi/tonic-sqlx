#!/bin/bash

grpcurl -plaintext \
  -import-path ./proto \
  -proto ./proto/gsdx/v1/gsdx.proto \
  -d '{"story_id": "42add4c3-fcde-4e4c-a53f-410d3a903356", "name": "Books Read"}' \
  "[::]:9090" \
  gsdx.v1.GsdxService/UpdateStory | jq -r
