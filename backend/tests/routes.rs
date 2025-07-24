use backend::store::dashboard::RegistrationStoreInner;
use backend::store::platform::PlatformStoreInner;
use backend::{login_route, register_event_route};
use backend::{KVStore, PlatformCommand, PlatformStore, RegistrationModel, RegistrationStore};
use http::{Request, StatusCode};
use models::PlatformUser;
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
