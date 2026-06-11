use crate::image_url::models::{
    AnimatedImageParams, AnimatedParams, StoryboardImageExtension, StoryboardParams,
    ThumbnailParams,
};

const BASE: &str = "https://image.mux.com";

/// [Fetch a thumbnail image from a video](https://docs.mux.com/guides/get-images-from-a-video)
/// at a specified time with optional transformations.
///
/// URL pattern: `https://image.mux.com/{PLAYBACK_ID}/thumbnail.{ext}`
pub fn thumbnail_url(playback_id: &str, params: Option<ThumbnailParams>) -> String {
    let ext = params
        .as_ref()
        .and_then(|p| p.format.as_deref())
        .unwrap_or("jpg");

    let base = format!("{}/{}/thumbnail.{}", BASE, playback_id, ext);

    let Some(p) = params else {
        return base;
    };

    let mut qs: Vec<String> = Vec::new();
    if let Some(v) = p.time {
        qs.push(format!("time={}", v))
    }
    if let Some(v) = p.width {
        qs.push(format!("width={}", v))
    }
    if let Some(v) = p.height {
        qs.push(format!("height={}", v))
    }
    if let Some(v) = p.rotate {
        qs.push(format!("rotate={}", v))
    }
    if let Some(v) = p.flip_h {
        qs.push(format!("flip_h={}", v))
    }
    if let Some(v) = p.flip_v {
        qs.push(format!("flip_v={}", v))
    }
    if let Some(v) = p.fit_mode {
        qs.push(format!("fit_mode={}", v))
    }
    if let Some(v) = p.latest {
        qs.push(format!("latest={}", v))
    }
    if let Some(v) = p.program_time {
        qs.push(format!("program_time={}", v))
    }
    if let Some(v) = p.token {
        qs.push(format!("TOKEN={}", v))
    }

    append_query(base, qs)
}

/// [Fetch an animated GIF or WebP image](https://docs.mux.com/guides/get-images-from-a-video#get-an-animated-gif-from-a-video)
/// from a video segment with optional transformations.
///
/// URL pattern: `https://image.mux.com/{PLAYBACK_ID}/animated.{ext}`
pub fn animated_url(playback_id: &str, params: Option<AnimatedParams>) -> String {
    let ext = params
        .as_ref()
        .and_then(|p| p.format.as_deref())
        .unwrap_or("gif");

    let base = format!("{}/{}/animated.{}", BASE, playback_id, ext);

    let Some(p) = params else {
        return base;
    };

    let mut qs: Vec<String> = Vec::new();
    if let Some(v) = p.start {
        qs.push(format!("start={}", v))
    }
    if let Some(v) = p.end {
        qs.push(format!("end={}", v))
    }
    if let Some(v) = p.width {
        qs.push(format!("width={}", v))
    }
    if let Some(v) = p.height {
        qs.push(format!("height={}", v))
    }
    if let Some(v) = p.fps {
        qs.push(format!("fps={}", v))
    }
    if let Some(v) = p.token {
        qs.push(format!("TOKEN={}", v))
    }

    append_query(base, qs)
}

/// Retrieve an animated image from a video.
pub fn animated_image_url(playback_id: &str, params: Option<AnimatedImageParams>) -> String {
    animated_url(playback_id, params)
}

/// [Fetch a storyboard image](https://docs.mux.com/guides/create-timeline-hover-previews)
/// composed of multiple thumbnails for use in timeline hover previews.
///
/// URL pattern: `https://image.mux.com/{PLAYBACK_ID}/storyboard.{ext}`
pub fn storyboard_url(playback_id: &str, params: Option<StoryboardParams>) -> String {
    let ext = params
        .as_ref()
        .and_then(|p| p.format.as_deref())
        .unwrap_or("jpg");

    let base = format!("{}/{}/storyboard.{}", BASE, playback_id, ext);
    storyboard_query(base, params)
}

/// Retrieve a storyboard image for timeline hover previews.
pub fn storyboard_image_url(
    playback_id: &str,
    extension: StoryboardImageExtension,
    params: Option<StoryboardParams>,
) -> String {
    let base = format!("{}/{}/storyboard.{}", BASE, playback_id, extension.as_str());
    storyboard_query(base, params)
}

/// [Fetch metadata for the storyboard image in WebVTT format](https://docs.mux.com/guides/create-timeline-hover-previews#webvtt),
/// detailing the coordinates and time ranges of each thumbnail.
///
/// URL pattern: `https://image.mux.com/{PLAYBACK_ID}/storyboard.vtt`
pub fn storyboard_vtt_url(playback_id: &str, params: Option<StoryboardParams>) -> String {
    let base = format!("{}/{}/storyboard.vtt", BASE, playback_id);
    storyboard_query(base, params)
}

/// [Fetch metadata for the storyboard image in JSON format](https://docs.mux.com/guides/create-timeline-hover-previews#json),
/// detailing the coordinates and time ranges of each thumbnail.
///
/// URL pattern: `https://image.mux.com/{PLAYBACK_ID}/storyboard.json`
pub fn storyboard_json_url(playback_id: &str, params: Option<StoryboardParams>) -> String {
    let base = format!("{}/{}/storyboard.json", BASE, playback_id);
    storyboard_query(base, params)
}

fn storyboard_query(base: String, params: Option<StoryboardParams>) -> String {
    let Some(p) = params else {
        return base;
    };

    let mut qs: Vec<String> = Vec::new();
    if let Some(v) = p.asset_start_time {
        qs.push(format!("asset_start_time={}", v))
    }
    if let Some(v) = p.asset_end_time {
        qs.push(format!("asset_end_time={}", v))
    }
    if let Some(v) = p.program_start_time {
        qs.push(format!("program_start_time={}", v))
    }
    if let Some(v) = p.program_end_time {
        qs.push(format!("program_end_time={}", v))
    }
    if let Some(v) = p.token {
        qs.push(format!("TOKEN={}", v))
    }

    append_query(base, qs)
}

fn append_query(base: String, qs: Vec<String>) -> String {
    if qs.is_empty() {
        base
    } else {
        format!("{}?{}", base, qs.join("&"))
    }
}
