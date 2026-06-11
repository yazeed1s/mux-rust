use mux_rust::{delivery_usage::DeliveryUsageParams, MuxClient};

fn main() {
    dotenvy::dotenv().ok();

    let token_id = std::env::var("MUX_TOKEN_ID").expect("MUX_TOKEN_ID not set");
    let token_secret = std::env::var("MUX_TOKEN_SECRET").expect("MUX_TOKEN_SECRET not set");
    let client = MuxClient::new(&token_id, &token_secret);

    // timeframe is unix timestamps, last hour here
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let one_hour_ago = now - 3600;

    println!("fetching delivery usage for the last hour...");
    let usage = client
        .delivery_usage
        .list_delivery_usage(Some(DeliveryUsageParams {
            timeframe: Some(vec![one_hour_ago.to_string(), now.to_string()]),
            ..Default::default()
        }))
        .expect("list failed");

    println!("  total assets: {}", usage.total_row_count);
    println!("  page:  {}  limit: {}", usage.page, usage.limit);
    println!();

    if usage.data.is_empty() {
        println!("  no delivery in this window");
        return;
    }

    // print a summary per asset
    println!(
        "{:<30} {:>10} {:>12} {:>10}",
        "asset id", "delivered", "resolution", "state"
    );
    println!("{}", "-".repeat(66));
    for r in &usage.data {
        println!(
            "{:<30} {:>9.1}s {:>12} {:>10}",
            r.asset_id, r.delivered_seconds, r.asset_resolution_tier, r.asset_state,
        );
    }

    // breakdown by resolution tier for the first asset
    if let Some(first) = usage.data.first() {
        println!("\nresolution breakdown for {}:", first.asset_id);
        let br = &first.delivered_seconds_by_resolution;
        if let Some(v) = br.tier_2160p {
            println!("  2160p:      {:.1}s", v)
        }
        if let Some(v) = br.tier_1440p {
            println!("  1440p:      {:.1}s", v)
        }
        if let Some(v) = br.tier_1080p {
            println!("  1080p:      {:.1}s", v)
        }
        if let Some(v) = br.tier_720p {
            println!("  720p:       {:.1}s", v)
        }
        if let Some(v) = br.tier_audio_only {
            println!("  audio only: {:.1}s", v)
        }
    }

    println!("\ndone");
}
