use backend::store::dashboard::RegistrationStoreInner;
use backend::store::platform::PlatformStoreInner;
use backend::{login_route, platform_create_route, platform_update_route, register_event_route};
use backend::{
    KVStore, PlatformCommand, PlatformModel, PlatformStore, RegistrationModel, RegistrationStore,
};
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

#[test]
fn register_event_success() {
    let mut store = temp_registration_store();
    let response = register_event_route(
        &Request::default(),
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
    let response = register_event_route(&Request::default(), store, "".into(), "".into()).unwrap();
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
fn platform_create_route_success() {
    let store = temp_platform_store();
    let platform = Platform {
        tenant_id: "t1".into(),
        community_name: "Test".into(),
        community_description: "desc".into(),
        platform_url: "http://example.com".into(),
    };
    let body = serde_json::to_vec(&platform).unwrap();
    let req = Request::new(body);
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
    let req = Request::new(body);
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
