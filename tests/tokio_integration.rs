//! Example app using Tokio.

#[cfg(feature = "with-tokio")]
mod tokio_integration {
    use async_channel_abs::{App, Tokio};

    #[tokio::test]
    async fn tokio_app() {
        let (app, driver) = App::<Tokio>::new("Hello Tokio!");
        let driver_handle = tokio::spawn(async move { driver.run().await });

        let init_state = app.get_state().await.unwrap();
        assert_eq!(init_state, "Hello Tokio!");

        let new_state = app.update_state("Hello again!").await.unwrap();
        assert_eq!(new_state, "Hello again!");

        app.terminate().await.unwrap();
        let _ = driver_handle.await.unwrap();
    }
}
