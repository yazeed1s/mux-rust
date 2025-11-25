# Unofficial Rust Client for Mux Video

This client is needed for one of my projects and is still under active development.

# TODO:
- [x] Authentication (Basic auth with Token ID + Token Secret)
- [x] Base HTTP client with retries and error handling

## Video APIs
- [ ] Live Streams API
  - [ ] Create live stream
  - [ ] Get live stream
  - [ ] List live streams
  - [ ] Update live stream
  - [ ] Disable live stream
- [ ] Assets API
  - [ ] Get asset
  - [ ] List assets
  - [ ] Delete asset
  - [ ] Get playback IDs
- [ ] Uploads API
  - [ ] Create direct upload
  - [ ] Get upload status
- [ ] Playback IDs API
  - [ ] Get playback info

## Data APIs
- [ ] Metrics
  - [ ] List available metrics
  - [ ] Get metric breakdowns
  - [ ] Get metric totals
- [ ] Real-Time Metrics
  - [ ] View real-time viewer counts
  - [ ] Get concurrent viewer timelines
- [ ] Insights
  - [ ] Get video performance insights
  - [ ] Get playback experience insights
- [ ] Errors
  - [ ] List error types
  - [ ] Get error counts
- [ ] Exports
  - [ ] List exports
  - [ ] Retrieve export URLs

## Utilities
- [ ] Webhooks signature verification
- [ ] Error types
- [ ] Consistent models matching Mux schema
- [ ] Examples & docs

