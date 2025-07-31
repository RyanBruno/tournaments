use backend::store::dashboard::{DashboardStoreInner, RegistrationStoreInner};
use backend::store::platform::PlatformStoreInner;
use backend::{
    dashboard_route, event_details_route, generate, login_route, platform_create_route,
    platform_update_route, platform_get_route, register_event_route,
};
use backend::{
    DashboardCommand, DashboardStore, KVStore, PlatformCommand, PlatformModel,
    PlatformStore, RegistrationModel, RegistrationStore,
};
use models::{Event, DashboardView};
use http::{Request, StatusCode};
use models::{Platform, PlatformPatch, PlatformUser};
use serde_json;
use tempfile::tempdir;

fn temp_registration_store() -> RegistrationStore {
    let dir = tempdir().unwrap().into_path();
    let inner = RegistrationStoreInner::new(
        dir.join("txn"),
        KVStore::new(dir.join("snap"), dir.join("event"), 1).unwrap(),
    );
    RegistrationStore::new(inner)
}

fn temp_platform_store() -> PlatformStore {
    let dir = tempdir().unwrap().into_path();
    let inner = PlatformStoreInner::new(
        dir.join("txn"),
        KVStore::new(dir.join("snap"), dir.join("event"), 1).unwrap(),
    );
    PlatformStore::new(inner)
}

fn temp_dashboard_store() -> DashboardStore {
    let dir = tempdir().unwrap().into_path();
    let inner = DashboardStoreInner::new(
        dir.join("txn"),
        KVStore::new(dir.join("snap"), dir.join("event"), 1).unwrap(),
    );
    DashboardStore::new(inner)
}

#[test]
fn register_event_success() {
    let mut store = temp_registration_store();
    let token = generate("user@example.com").unwrap();
    let req = Request::builder()
        .header("Authorization", token)
        .body(()).unwrap();
    let response = register_event_route(
        &req,
        store.clone(),
        "e1".into(),
        "user@example.com".into(),
    )
    .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    store.fold().unwrap();
    let reg = store
        .borrow_inner()
        .query_owned("e1-user@example.com".into())
        .unwrap();
    assert!(matches!(reg, Some(RegistrationModel::Registration(_))));
}

#[test]
fn register_event_invalid() {
    let store = temp_registration_store();
    let token = generate("user@example.com").unwrap();
    let req = Request::builder()
        .header("Authorization", token)
        .body(()).unwrap();
    let response = register_event_route(&req, store, "".into(), "".into()).unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn login_route_success() {
    let mut store = temp_platform_store();
    store
        .command(&PlatformCommand::CreateUser(PlatformUser::new(
            "u@example.com".into(),
            "pw".into(),
        )))
        .unwrap();
    store.fold().unwrap();
    let res = login_route(
        &Request::default(),
        store,
        "u@example.com".into(),
        "pw".into(),
    )
    .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.body();
    let v: serde_json::Value = serde_json::from_slice(body).unwrap();
    assert!(v.get("token").is_some());
}

#[test]
fn login_route_fail() {
    let mut store = temp_platform_store();
    store
        .command(&PlatformCommand::CreateUser(PlatformUser::new(
            "u@example.com".into(),
            "pw".into(),
        )))
        .unwrap();
    store.fold().unwrap();
    let res = login_route(
        &Request::default(),
        store,
        "u@example.com".into(),
        "wrong".into(),
    )
    .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn login_route_missing_credentials() {
    let store = temp_platform_store();
    let res = login_route(&Request::default(), store, "".into(), "".into()).unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn platform_create_route_success() {
    let mut store = temp_platform_store();
    let platform = Platform {
        tenant_id: "t1".into(),
        community_name: "Test".into(),
        community_description: "desc".into(),
        platform_url: "http://example.com".into(),
    };
    let body = serde_json::to_vec(&platform).unwrap();
    let token = generate("user@example.com").unwrap();
    let req = Request::builder()
        .header("Authorization", token)
        .body(body)
        .unwrap();
    let res = platform_create_route(req, store.clone()).unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    store.fold().unwrap();
    let saved = store
        .borrow_inner()
        .query_owned("platform-t1".into())
        .unwrap();
    assert!(matches!(saved, Some(PlatformModel::Platform(_))));
}

#[test]
fn platform_update_route_success() {
    let mut store = temp_platform_store();
    let platform = Platform {
        tenant_id: "t2".into(),
        community_name: "Old".into(),
        community_description: "old".into(),
        platform_url: "http://old.com".into(),
    };
    store
        .command(&PlatformCommand::CreatePlatform(platform.clone()))
        .unwrap();
    store.fold().unwrap();

    let patch = PlatformPatch {
        tenant_id: "t2".into(),
        community_name: Some("New".into()),
        community_description: None,
        platform_url: None,
    };
    let body = serde_json::to_vec(&patch).unwrap();
    let token = generate("user@example.com").unwrap();
    let req = Request::builder()
        .header("Authorization", token)
        .body(body)
        .unwrap();
    let res = platform_update_route(req, store.clone()).unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    store.fold().unwrap();
    let updated = store
        .borrow_inner()
        .query_owned("platform-t2".into())
        .unwrap();
    if let Some(PlatformModel::Platform(p)) = updated {
        assert_eq!(p.community_name, "New");
    } else {
        panic!("platform not found");
    }
}

#[test]
fn register_event_unauthorized() {
    let store = temp_registration_store();
    let response = register_event_route(&Request::default(), store, "e1".into(), "user@example.com".into()).unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn platform_create_route_unauthorized() {
    let mut store = temp_platform_store();
    let platform = Platform {
        tenant_id: "t1".into(),
        community_name: "Test".into(),
        community_description: "desc".into(),
        platform_url: "http://example.com".into(),
    };
    let body = serde_json::to_vec(&platform).unwrap();
    let req = Request::new(body);
    let res = platform_create_route(req, store.clone()).unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn platform_update_route_unauthorized() {
    let mut store = temp_platform_store();
    let platform = Platform {
        tenant_id: "t2".into(),
        community_name: "Old".into(),
        community_description: "old".into(),
        platform_url: "http://old.com".into(),
    };
    store
        .command(&PlatformCommand::CreatePlatform(platform.clone()))
        .unwrap();
    store.fold().unwrap();
    let patch = PlatformPatch {
        tenant_id: "t2".into(),
        community_name: Some("New".into()),
        community_description: None,
        platform_url: None,
    };
    let body = serde_json::to_vec(&patch).unwrap();
    let req = Request::new(body);
    let res = platform_update_route(req, store.clone()).unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn dashboard_route_success() {
    let mut store = temp_dashboard_store();
    let event = Event {
        tenant_id: "t1".into(),
        id: "e1".into(),
        name: "Test Event".into(),
        location: "Loc".into(),
        date: "2025".into(),
        image: "img".into(),
        banner: None,
        upsell: None,
        active: true,
    };
    store
        .command(&DashboardCommand::CreateEvent(event.clone()))
        .unwrap();
    store.fold().unwrap();
    let token = generate("user@example.com").unwrap();
    let req = Request::builder()
        .header("Authorization", token)
        .body(()).unwrap();
    let res = dashboard_route(&req, store.clone(), "t1".into()).unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[test]
fn dashboard_route_unauthorized() {
    let store = temp_dashboard_store();
    let res = dashboard_route(&Request::default(), store, "t1".into()).unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[test]
fn event_details_route_success() {
    let mut store = temp_dashboard_store();
    let event = Event {
        tenant_id: "t1".into(),
        id: "e1".into(),
        name: "Test Event".into(),
        location: "Loc".into(),
        date: "2025".into(),
        image: "img".into(),
        banner: None,
        upsell: None,
        active: true,
    };
    store
        .command(&DashboardCommand::CreateEvent(event.clone()))
        .unwrap();
    store.fold().unwrap();
    let token = generate("user@example.com").unwrap();
    let req = Request::builder()
        .header("Authorization", token)
        .body(()).unwrap();
    let res = event_details_route(&req, store.clone(), "e1".into()).unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[test]
fn event_details_route_unauthorized() {
    let store = temp_dashboard_store();
    let res = event_details_route(&Request::default(), store, "e1".into()).unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[test]
fn platform_get_route_success() {
    let mut store = temp_platform_store();
    let platform = Platform {
        tenant_id: "t3".into(),
        community_name: "CN".into(),
        community_description: "desc".into(),
        platform_url: "http://ex.com".into(),
    };
    store
        .command(&PlatformCommand::CreatePlatform(platform.clone()))
        .unwrap();
    store.fold().unwrap();
    let token = generate("u@example.com").unwrap();
    let req = Request::builder()
        .header("Authorization", token)
        .body(()).unwrap();
    let res = platform_get_route(&req, store.clone(), "t3".into()).unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let got: Platform = serde_json::from_slice(res.body()).unwrap();
    assert_eq!(got, platform);
}

#[test]
fn platform_get_route_unauthorized() {
    let store = temp_platform_store();
    let res = platform_get_route(&Request::default(), store, "t1".into()).unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
