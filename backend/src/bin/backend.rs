use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

use backend::PlatformStore;
use backend::{
    dashboard_store, platform_store, registration_store, AsyncHttpRequest, AsyncTcpListener,
    DashboardCommand, DashboardStore, NetExecutor, PlatformCommand,
};

use models::{DashboardUser, Event, Platform, PlatformUser};

use backend::{
    dashboard_route, error_route, event_details_route, login_route, not_found_route,
    register_event_route,
};
fn clear_directory(path: &str) -> io::Result<()> {
    if Path::new(path).exists() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(path)?;
            } else if path.is_dir() {
                fs::remove_dir_all(path)?;
            }
        }
    }
    Ok(())
}

pub fn seed_platform(platform_store: &mut PlatformStore) {
    let platform = Platform {
        tenant_id: "bucket-golf".into(),
        community_name: "Bucket Golf Leagues".into(),
        community_description: "Join the most exciting bucket golf leagues in the area!".into(),
        platform_url: "https://bucketgolf.example.com".into(),
    };

    if let Err(e) = platform_store.command(&PlatformCommand::CreatePlatform(platform.clone())) {
        eprintln!("Failed to insert platform: {:?}", e);
    }

    let user = PlatformUser::new("ryanbruno506@gmail.com".into(), "hashed_password".into());
    if let Err(e) = platform_store.command(&PlatformCommand::CreateUser(user.clone())) {
        eprintln!("Failed to insert user: {:?}", e);
    }

    println!("✅ Platform seeded.");
}

pub fn seed_example_events(dashboard_store: &mut DashboardStore) {
    let examples = vec![
        Event {
            tenant_id: "bucket-golf".into(),
            id: "1".into(),
            name: "Arlington Bucket Golf League".into(),
            location: "Quincy Park, VA".into(),
            date: "Saturday, July 13 – 3:00 PM".into(),
            image: "/static/bucket-golf.jpg".into(),
            banner: Some("⚡ Almost Sold Out".into()),
            upsell: Some("Only 3 slots left".into()),
        },
        Event {
            tenant_id: "bucket-golf".into(),
            id: "2".into(),
            name: "Launch Meetup".into(),
            location: "Liberty Park, DC".into(),
            date: "Monday, July 15 – 5:00 PM".into(),
            image: "/static/launch-meetup.jpg".into(),
            banner: None,
            upsell: None,
        },
        Event {
            tenant_id: "bucket-golf".into(),
            id: "3".into(),
            name: "Weekly Planning Session".into(),
            location: "National Mall, DC".into(),
            date: "Wednesday, July 17 – 12:00 PM".into(),
            image: "/static/planning.jpg".into(),
            banner: Some("🆕 New".into()),
            upsell: Some("Limited spots available".into()),
        },
    ];

    for event in examples {
        if let Err(e) = dashboard_store.command(&DashboardCommand::CreateEvent(event.clone())) {
            eprintln!("Failed to insert event {:?}: {:?}", event.name, e);
        }
    }

    let user = DashboardUser::new("ryanbruno506@gmail.com".into(), "hashed_password".into());
    if let Err(e) = dashboard_store.command(&DashboardCommand::CreateUser(user.clone())) {
        eprintln!("Failed to insert user: {:?}", e);
    }

    dashboard_store
        .command(&DashboardCommand::SetName((
            "bucket-golf".into(),
            "Bucket Golf Leagues".into(),
        )))
        .unwrap();

    dashboard_store.command(&DashboardCommand::SetAnnouncement((
    "bucket-golf".into(),
    "⛳️ New summer leagues of bucket golf just dropped. Rally your crew and start swinging!".into(),
  ))).unwrap();

    println!("✅ Example events seeded.");
}

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    /* Clear and seed data for development */
    clear_directory("data/")?;
    let mut dashboard_store = dashboard_store()?;
    let mut platform_store = platform_store()?;
    let mut registration_store = registration_store()?;
    seed_example_events(&mut dashboard_store);
    seed_platform(&mut platform_store);
    dashboard_store.fold()?;
    platform_store.fold()?;
    registration_store.fold()?;

    /* Start the server */
    let executor = NetExecutor::new();
    let listener = AsyncTcpListener::new(8000, executor.clone()).unwrap();
    let server = AsyncHttpRequest::new(listener, executor.clone());

    executor.clone().spawn(async move {
        loop {
            /* Wait for a new request */
            let (request, mut stream) = server.next_request().await.unwrap();

            /* Process the request */
            let response = match request.uri().path() {
                "/dashboard" => dashboard_route(
                    &request,
                    dashboard_store.clone(),
                    request
                        .headers()
                        .get("x-tenant_id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or_default()
                        .to_string(),
                ),
                "/dashboard/event" => event_details_route(
                    &request,
                    dashboard_store.clone(),
                    request
                        .headers()
                        .get("x-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or_default()
                        .to_string(),
                ),
                "/dashboard/register_event" => register_event_route(
                    &request,
                    registration_store.clone(),
                    request
                        .headers()
                        .get("x-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or_default()
                        .to_string(),
                    request
                        .headers()
                        .get("x-email")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("guest@example.com")
                        .to_string(),
                ),
                "/dashboard/login" => panic!(),
                "/platform/login" => login_route(
                    &request,
                    platform_store.clone(),
                    request
                        .headers()
                        .get("x-email")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or_default()
                        .to_string(),
                    request
                        .headers()
                        .get("x-password")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or_default()
                        .to_string(),
                ),

                _ => not_found_route(),
            }
            .unwrap_or_else(|e| {
                error!("Error processing request: {e}");
                error_route()
            });

            /* Write the response back to the client */
            AsyncHttpRequest::write_response(&mut stream, response).await;
        }
    });

    executor.run();
    Ok(())
}
