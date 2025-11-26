// Video API
pub const LIVE_STREAMS: &str = "/video/v1/live-streams";
pub const ASSETS: &str = "/video/v1/assets";
pub const PLAYBACK_IDS: &str = "/video/v1/playback-ids";
pub const SIGNING_KEYS: &str = "/video/v1/signing-keys";
pub const UPLOADS: &str = "/video/v1/uploads";
pub const DELIVERY_USAGE: &str = "/video/v1/delivery-usage";
pub const PLAYBACK_RESTRICTIONS: &str = "/video/v1/playback-restrictions";
pub const DRM_CONFIGURATIONS: &str = "/video/v1/drm-configurations";
pub const TRANSCRIPTION_VOCABS: &str = "/video/v1/transcription-vocabularies";
// Data API
pub const VIDEO_VIEWS: &str = "/data/v1/video-views";
pub const ERRORS: &str = "/data/v1/errors";
pub const FILTERS: &str = "/data/v1/filters";
pub const EXPORTS: &str = "/data/v1/exports";
pub const METRICS: &str = "/data/v1/metrics";
pub const MONITORING: &str = "/data/v1/monitoring";
pub const REALTIME: &str = "/data/v1/realtime";
pub const DIMENSIONS: &str = "/data/v1/dimensions";
pub const INCIDENTS: &str = "/data/v1/incidents";
pub const ANNOTATIONS: &str = "/data/v1/annotations";

#[inline]
pub fn live_stream(id: &str) -> String {
    format!("{}/{}", LIVE_STREAMS, id)
}
