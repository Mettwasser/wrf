use loco_rs::prelude::request;
use serial_test::serial;
use wrf::app::App;

#[tokio::test]
#[serial]
async fn can_get_lobbies() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/api/lobbies/").await;
        assert_eq!(res.status_code(), 200);

        // you can assert content like this:
        // assert_eq!(res.text(), "content");
    })
    .await;
}
