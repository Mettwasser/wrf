use loco_rs::testing;
use serial_test::serial;
use wrf::app::App;

#[tokio::test]
#[serial]
async fn can_get_register_sessions() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/register_sessions/").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_POST() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/register_sessions/POST").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_GET() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/register_sessions/GET").await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}
