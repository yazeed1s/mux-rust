use mux_rust::{
    image_url::{
        self, AnimatedParams, StoryboardImageExtension, StoryboardParams, ThumbnailParams,
    },
    stream_url::{self, HlsParams, StaticRendition},
};

fn main() {
    let playback_id = "your-playback-id";
    let track_id = "your-track-id";
    let token = "your-signed-jwt";

    // HLS
    let hls = stream_url::hls_url(playback_id, None);
    println!("hls (public):   {}", hls);

    let signed_params = HlsParams {
        token: Some(token.into()),
        ..Default::default()
    };
    let hls_signed = stream_url::hls_url(playback_id, Some(signed_params));
    println!("hls (signed):   {}", hls_signed);

    let clip_params = HlsParams {
        asset_start_time: Some(10.0),
        asset_end_time: Some(60.0),
        default_subtitles_lang: Some("en".into()),
        ..Default::default()
    };
    let hls_clipped = stream_url::hls_url(playback_id, Some(clip_params));
    println!("hls (clipped):  {}", hls_clipped);

    // static renditions
    println!();
    let mp4_1080 =
        stream_url::static_rendition_url(playback_id, StaticRendition::Capped1080p, None);
    let mp4_720 = stream_url::static_rendition_url(playback_id, StaticRendition::High, None);
    let audio = stream_url::static_rendition_url(playback_id, StaticRendition::AudioOnly, None);
    println!("mp4 (1080p):    {}", mp4_1080);
    println!("mp4 (720p):     {}", mp4_720);
    println!("audio only:     {}", audio);

    // text tracks
    println!();
    let vtt = stream_url::text_track_url(playback_id, track_id, None);
    let txt = stream_url::transcript_url(playback_id, track_id, None);
    println!("subtitle vtt:   {}", vtt);
    println!("transcript:     {}", txt);

    // thumbnail
    println!();
    let thumb_plain = image_url::thumbnail_url(playback_id, None);
    println!("thumbnail:      {}", thumb_plain);

    let thumb_params = ThumbnailParams {
        time: Some(30.5),
        width: Some(1280),
        height: Some(720),
        fit_mode: Some("smartcrop".into()),
        ..Default::default()
    };
    let thumb = image_url::thumbnail_url(playback_id, Some(thumb_params));
    println!("thumbnail:      {}", thumb);

    // animated gif
    println!();
    let gif_params = AnimatedParams {
        start: Some(5.0),
        end: Some(10.0),
        width: Some(480),
        fps: Some(15),
        ..Default::default()
    };
    let gif = image_url::animated_url(playback_id, Some(gif_params));
    println!("animated gif:   {}", gif);

    let webp_params = AnimatedParams {
        start: Some(0.0),
        end: Some(3.0),
        width: Some(320),
        format: Some("webp".into()),
        ..Default::default()
    };
    let webp = image_url::animated_url(playback_id, Some(webp_params));
    println!("animated webp:  {}", webp);

    // storyboard
    println!();
    let board = image_url::storyboard_url(playback_id, None);
    println!("storyboard:     {}", board);

    let board_params = StoryboardParams {
        asset_start_time: Some(0.0),
        asset_end_time: Some(120.0),
        ..Default::default()
    };
    let board_vtt = image_url::storyboard_vtt_url(playback_id, Some(board_params));
    println!("storyboard vtt: {}", board_vtt);

    let board_json = image_url::storyboard_json_url(playback_id, None);
    println!("storyboard json:{}", board_json);

    let board_png =
        image_url::storyboard_image_url(playback_id, StoryboardImageExtension::Png, None);
    println!("storyboard png: {}", board_png);
}
