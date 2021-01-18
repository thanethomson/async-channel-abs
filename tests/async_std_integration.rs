use async_channel_abs::{App, AsyncStd};

#[async_std::test]
async fn async_std_app() {
    let (app, driver) = App::<AsyncStd>::new("Hello async-std!");
    let driver_handle = async_std::task::spawn(async move { driver.run().await });

    let init_state = app.get_state().await.unwrap();
    assert_eq!(init_state, "Hello async-std!");

    let new_state = app.update_state("Hello again!").await.unwrap();
    assert_eq!(new_state, "Hello again!");

    app.terminate().await.unwrap();
    let _ = driver_handle.await;
}
