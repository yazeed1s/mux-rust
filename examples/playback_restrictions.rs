use mux_rust::{
    playback_restrictions::{
        CreatePlaybackRestrictionRequest, ListPlaybackRestrictionsParams, ReferrerRestriction,
        UserAgentRestriction,
    },
    MuxClient,
};

fn main() {
    dotenvy::dotenv().ok();

    let token_id = std::env::var("MUX_TOKEN_ID").expect("MUX_TOKEN_ID not set");
    let token_secret = std::env::var("MUX_TOKEN_SECRET").expect("MUX_TOKEN_SECRET not set");
    let client = MuxClient::new(&token_id, &token_secret);

    // create
    println!("creating playback restriction...");
    let referrer = ReferrerRestriction {
        allowed_domains: vec!["example.com".into(), "*.example.com".into()],
        allow_no_referrer: Some(false),
    };
    let user_agent = UserAgentRestriction {
        allow_high_risk_user_agent: true,
        allow_no_user_agent: false,
    };
    let req = CreatePlaybackRestrictionRequest {
        referrer,
        user_agent,
    };
    let restriction = client
        .playback_restrictions
        .create_playback_restriction(req)
        .expect("create failed");
    let id = restriction.data.id.clone();
    println!("  id:         {}", id);
    println!("  created at: {}", restriction.data.created_at);

    // list
    println!("\nlisting restrictions...");
    let list_params = ListPlaybackRestrictionsParams {
        limit: Some(10),
        ..Default::default()
    };
    let list = client
        .playback_restrictions
        .list_playback_restrictions(Some(list_params))
        .expect("list failed");
    println!("  found {}", list.data.len());

    // get
    println!("\ngetting restriction {}...", id);
    let got = client
        .playback_restrictions
        .get_playback_restriction(&id)
        .expect("get failed");
    println!("  allowed domains: {:?}", got.data.referrer.allowed_domains);

    // update referrer
    println!("\nopening to all domains...");
    let new_referrer = ReferrerRestriction {
        allowed_domains: vec!["*".into()],
        allow_no_referrer: Some(true),
    };
    client
        .playback_restrictions
        .update_referrer_restriction(&id, new_referrer)
        .expect("update referrer failed");

    // update user agent
    println!("tightening user agent policy...");
    let new_ua = UserAgentRestriction {
        allow_high_risk_user_agent: false,
        allow_no_user_agent: false,
    };
    client
        .playback_restrictions
        .update_user_agent_restriction(&id, new_ua)
        .expect("update user agent failed");

    // delete
    println!("\ndeleting restriction...");
    client
        .playback_restrictions
        .delete_playback_restriction(&id)
        .expect("delete failed");
    println!("  deleted");

    println!("\ndone");
}
