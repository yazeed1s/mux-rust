// System API
pub const SYSTEM_SIGNING_KEYS: &str = "/system/v1/signing-keys";
pub const WHOAMI: &str = "/system/v1/whoami";

#[inline]
pub fn signing_key(id: &str) -> String {
    format!("{}/{}", SYSTEM_SIGNING_KEYS, id)
}

// Video API
pub const LIVE_STREAMS: &str = "/video/v1/live-streams";
pub const ASSETS: &str = "/video/v1/assets";
pub const PLAYBACK_IDS: &str = "/video/v1/playback-ids";
pub const URL_SIGNING_KEYS: &str = "/video/v1/signing-keys";
pub const UPLOADS: &str = "/video/v1/uploads";
pub const DELIVERY_USAGE: &str = "/video/v1/delivery-usage";
pub const PLAYBACK_RESTRICTIONS: &str = "/video/v1/playback-restrictions";
pub const DRM_CONFIGURATIONS: &str = "/video/v1/drm-configurations";
pub const TRANSCRIPTION_VOCABS: &str = "/video/v1/transcription-vocabularies";

#[inline]
pub fn live_stream(id: &str) -> String {
    format!("{}/{}", LIVE_STREAMS, id)
}

#[inline]
pub fn live_stream_playback_ids(id: &str) -> String {
    format!("{}/playback-ids", live_stream(id))
}

#[inline]
pub fn live_stream_playback_id(id: &str, playback_id: &str) -> String {
    format!("{}/playback-ids/{}", live_stream(id), playback_id)
}

#[inline]
pub fn live_stream_reset_stream_key(id: &str) -> String {
    format!("{}/reset-stream-key", live_stream(id))
}

#[inline]
pub fn live_stream_complete(id: &str) -> String {
    format!("{}/complete", live_stream(id))
}

#[inline]
pub fn live_stream_disable(id: &str) -> String {
    format!("{}/disable", live_stream(id))
}

#[inline]
pub fn live_stream_enable(id: &str) -> String {
    format!("{}/enable", live_stream(id))
}

#[inline]
pub fn live_stream_embedded_subtitles(id: &str) -> String {
    format!("{}/embedded-subtitles", live_stream(id))
}

#[inline]
pub fn live_stream_generated_subtitles(id: &str) -> String {
    format!("{}/generated-subtitles", live_stream(id))
}

#[inline]
pub fn live_stream_simulcast_targets(id: &str) -> String {
    format!("{}/simulcast-targets", live_stream(id))
}

#[inline]
pub fn live_stream_simulcast_target(id: &str, target_id: &str) -> String {
    format!("{}/simulcast-targets/{}", live_stream(id), target_id)
}

#[inline]
pub fn live_stream_static_renditions(id: &str) -> String {
    format!("{}/new-asset-settings/static-renditions", live_stream(id))
}

#[inline]
pub fn asset(id: &str) -> String {
    format!("{}/{}", ASSETS, id)
}

#[inline]
pub fn asset_input_info(id: &str) -> String {
    format!("{}/input-info", asset(id))
}

#[inline]
pub fn asset_playback_ids(id: &str) -> String {
    format!("{}/playback-ids", asset(id))
}

#[inline]
pub fn asset_playback_id(id: &str, playback_id: &str) -> String {
    format!("{}/playback-ids/{}", asset(id), playback_id)
}

#[inline]
pub fn asset_mp4_support(id: &str) -> String {
    format!("{}/mp4-support", asset(id))
}

#[inline]
pub fn asset_master_access(id: &str) -> String {
    format!("{}/master-access", asset(id))
}

#[inline]
pub fn asset_static_renditions(id: &str) -> String {
    format!("{}/static-renditions", asset(id))
}

#[inline]
pub fn asset_static_rendition(id: &str, rendition_id: &str) -> String {
    format!("{}/static-renditions/{}", asset(id), rendition_id)
}

#[inline]
pub fn asset_tracks(id: &str) -> String {
    format!("{}/tracks", asset(id))
}

#[inline]
pub fn asset_track(id: &str, track_id: &str) -> String {
    format!("{}/tracks/{}", asset(id), track_id)
}

#[inline]
pub fn asset_track_generate_subtitles(id: &str, track_id: &str) -> String {
    format!("{}/generate-subtitles", asset_track(id, track_id))
}

#[inline]
pub fn playback_id(playback_id: &str) -> String {
    format!("{}/{}", PLAYBACK_IDS, playback_id)
}

#[inline]
pub fn url_signing_key(id: &str) -> String {
    format!("{}/{}", URL_SIGNING_KEYS, id)
}

#[inline]
pub fn upload(id: &str) -> String {
    format!("{}/{}", UPLOADS, id)
}

#[inline]
pub fn upload_cancel(id: &str) -> String {
    format!("{}/cancel", upload(id))
}

#[inline]
pub fn playback_restriction(id: &str) -> String {
    format!("{}/{}", PLAYBACK_RESTRICTIONS, id)
}

#[inline]
pub fn playback_restriction_referrer(id: &str) -> String {
    format!("{}/referrer", playback_restriction(id))
}

#[inline]
pub fn playback_restriction_user_agent(id: &str) -> String {
    format!("{}/user_agent", playback_restriction(id))
}

#[inline]
pub fn drm_configuration(id: &str) -> String {
    format!("{}/{}", DRM_CONFIGURATIONS, id)
}

#[inline]
pub fn transcription_vocabulary(id: &str) -> String {
    format!("{}/{}", TRANSCRIPTION_VOCABS, id)
}

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
pub fn video_view(id: &str) -> String {
    format!("{}/{}", VIDEO_VIEWS, id)
}

#[inline]
pub fn filter_values(filter_id: &str) -> String {
    format!("{}/{}", FILTERS, filter_id)
}

#[inline]
pub fn exports_views() -> &'static str {
    "/data/v1/exports/views"
}

#[inline]
pub fn metric_breakdown(metric_id: &str) -> String {
    format!("{}/{}/breakdown", METRICS, metric_id)
}

#[inline]
pub fn metric_overall(metric_id: &str) -> String {
    format!("{}/{}/overall", METRICS, metric_id)
}

#[inline]
pub fn metric_insights(metric_id: &str) -> String {
    format!("{}/{}/insights", METRICS, metric_id)
}

#[inline]
pub fn metric_timeseries(metric_id: &str) -> String {
    format!("{}/{}/timeseries", METRICS, metric_id)
}

#[inline]
pub fn metrics_comparison() -> &'static str {
    "/data/v1/metrics/comparison"
}

#[inline]
pub fn engagement_asset_heatmap(asset_id: &str) -> String {
    format!("/data/v1/engagement/assets/{}/heatmap", asset_id)
}

#[inline]
pub fn engagement_video_heatmap(video_id: &str) -> String {
    format!("/data/v1/engagement/video/{}/heatmap", video_id)
}

#[inline]
pub fn engagement_playback_heatmap(playback_id: &str) -> String {
    format!("/data/v1/engagement/playback/{}/heatmap", playback_id)
}

#[inline]
pub fn engagement_asset_hotspots(asset_id: &str) -> String {
    format!("/data/v1/engagement/assets/{}/hotspots", asset_id)
}

#[inline]
pub fn engagement_video_hotspots(video_id: &str) -> String {
    format!("/data/v1/engagement/video/{}/hotspots", video_id)
}

#[inline]
pub fn engagement_playback_hotspots(playback_id: &str) -> String {
    format!("/data/v1/engagement/playback/{}/hotspots", playback_id)
}

#[inline]
pub fn monitoring_dimensions() -> &'static str {
    "/data/v1/monitoring/dimensions"
}

#[inline]
pub fn monitoring_metrics() -> &'static str {
    "/data/v1/monitoring/metrics"
}

#[inline]
pub fn monitoring_breakdown(metric_id: &str) -> String {
    format!("/data/v1/monitoring/metrics/{}/breakdown", metric_id)
}

#[inline]
pub fn monitoring_breakdown_timeseries(metric_id: &str) -> String {
    format!(
        "/data/v1/monitoring/metrics/{}/breakdown-timeseries",
        metric_id
    )
}

#[inline]
pub fn monitoring_histogram_timeseries(metric_id: &str) -> String {
    format!(
        "/data/v1/monitoring/metrics/{}/histogram-timeseries",
        metric_id
    )
}

#[inline]
pub fn monitoring_timeseries(metric_id: &str) -> String {
    format!("/data/v1/monitoring/metrics/{}/timeseries", metric_id)
}

#[inline]
pub fn realtime_dimensions() -> &'static str {
    "/data/v1/realtime/dimensions"
}

#[inline]
pub fn realtime_metrics() -> &'static str {
    "/data/v1/realtime/metrics"
}

#[inline]
pub fn realtime_breakdown(metric_id: &str) -> String {
    format!("/data/v1/realtime/metrics/{}/breakdown", metric_id)
}

#[inline]
pub fn realtime_histogram_timeseries(metric_id: &str) -> String {
    format!(
        "/data/v1/realtime/metrics/{}/histogram-timeseries",
        metric_id
    )
}

#[inline]
pub fn realtime_timeseries(metric_id: &str) -> String {
    format!("/data/v1/realtime/metrics/{}/timeseries", metric_id)
}

#[inline]
pub fn dimension_values(dimension_id: &str) -> String {
    format!("{}/{}", DIMENSIONS, dimension_id)
}

#[inline]
pub fn dimension_elements(dimension_id: &str) -> String {
    format!("{}/{}/elements", DIMENSIONS, dimension_id)
}

#[inline]
pub fn incident(id: &str) -> String {
    format!("{}/{}", INCIDENTS, id)
}

#[inline]
pub fn related_incidents(id: &str) -> String {
    format!("{}/{}/related", INCIDENTS, id)
}

#[inline]
pub fn annotation(id: &str) -> String {
    format!("{}/{}", ANNOTATIONS, id)
}
