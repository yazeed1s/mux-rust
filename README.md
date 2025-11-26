# Unofficial Rust Client for Mux Video

This is a lightweight, work-in-progress Rust SDK for the full Mux Video and Data APIs.  
I’m building it for a project of mine, and the feature list grows whenever I have enough caffeine.

---

## Status

- Authentication (Token ID + Token Secret)
- Base HTTP client with inspection, retries, and sane error handling
- Centralized endpoint definitions (`common/endpoints.rs`)
- Initial Live Streams API support

---

## Video API Coverage

### Live Streams API
- [x] Create live stream  
- [x] Get live stream  
- [x] List live streams  
- [ ] Update live stream  
- [x] Disable/delete live stream  
- [ ] Reset stream key  
- [ ] Manage playback IDs  
- [ ] Manage simulcast targets  
- [ ] Manage tracks (subtitles, captions)  
- [ ] Manage static renditions  

### Assets API
- [ ] Create asset  
- [ ] Get asset  
- [ ] List assets  
- [ ] Delete asset  
- [ ] Update asset  
- [ ] Get input info  
- [ ] Manage playback IDs  
- [ ] Manage tracks  
- [ ] Manage static renditions  

### Uploads API
- [ ] Create direct upload  
- [ ] Get upload  
- [ ] List uploads  
- [ ] Cancel upload  

### Playback IDs API
- [ ] Get playback info  

### Signing Keys
- [ ] Create signing key  
- [ ] List signing keys  
- [ ] Get signing key  
- [ ] Delete signing key  

### Delivery Usage
- [ ] List delivery usage items  

### Playback Restrictions
- [ ] Create restriction  
- [ ] Get restriction  
- [ ] List restrictions  
- [ ] Update restriction  
- [ ] Delete restriction  

### DRM Configurations
- [ ] List configurations  
- [ ] Get configuration  

### Transcription Vocabularies
- [ ] Create vocabulary  
- [ ] List vocabularies  
- [ ] Get vocabulary  
- [ ] Update vocabulary  
- [ ] Delete vocabulary  

---

## Data API Coverage

### Video Views
- [ ] List video views  
- [ ] Get view by ID  

### Errors
- [ ] List error types  
- [ ] Get error counts  

### Filters
- [ ] List filters  
- [ ] List filter values  

### Exports
- [ ] List exports  
- [ ] Get export URLs  

### Metrics
- [ ] List metrics  
- [ ] Metric breakdowns  
- [ ] Metric totals  
- [ ] Metric timeseries  

### Monitoring
- [ ] List monitoring dimensions  
- [ ] List monitoring metrics  
- [ ] Monitoring breakdowns  
- [ ] Monitoring histograms  
- [ ] Monitoring timeseries  

### Real-Time
- [ ] List realtime dimensions  
- [ ] List realtime metrics  
- [ ] Realtime breakdowns  
- [ ] Realtime timeseries  

### Dimensions
- [ ] List dimensions  
- [ ] List dimension values  
- [ ] List dimension elements  

### Incidents
- [ ] List incidents  
- [ ] Get incident  
- [ ] List related incidents  

### Annotations
- [ ] List annotations  
- [ ] Create annotation  
- [ ] Get annotation  
- [ ] Update annotation  
- [ ] Delete annotation  

---

## Utilities & Internal

- [ ] Webhook signature verification  
- [ ] Unified error types  
- [ ] Rate-limit–aware HTTP client  
- [ ] Pagination helpers  
- [ ] Typed models matching the Mux schema  
- [ ] Examples & documentation  

---

PRs are welcome. Complaints are not.
