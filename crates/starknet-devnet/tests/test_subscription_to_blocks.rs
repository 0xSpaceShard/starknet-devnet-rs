#![cfg(test)]
pub mod common;

mod websocket_subscription_support {
    use std::collections::HashMap;

    use serde_json::json;
    use tokio_tungstenite::connect_async;

    use crate::common::background_devnet::BackgroundDevnet;
    use crate::common::utils::{assert_no_notifications, receive_rpc_via_ws, send_text_rpc_via_ws};

    #[tokio::test]
    async fn subscribe_to_new_block_heads_happy_path() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();
        let (mut ws, _) = connect_async(devnet.ws_url()).await.unwrap();

        let subscription_confirmation =
            send_text_rpc_via_ws(&mut ws, "starknet_subscribeNewHeads", json!({})).await.unwrap();
        let subscription_id = subscription_confirmation["result"].as_i64().unwrap();

        // test with multiple blocks created, number 0 was origin, so we start at 1
        for block_i in 1..=2 {
            let created_block_hash = devnet.create_block().await.unwrap();

            let notification = receive_rpc_via_ws(&mut ws).await.unwrap();
            assert_eq!(notification["method"], "starknet_subscriptionNewHeads");
            assert_eq!(
                notification["params"]["result"]["block_hash"].as_str().unwrap(),
                created_block_hash.to_hex_string().as_str()
            );

            assert_eq!(notification["params"]["result"]["block_number"].as_i64().unwrap(), block_i);
            assert_eq!(
                notification["params"]["subscription_id"].as_i64().unwrap(),
                subscription_id
            );
        }
    }

    #[tokio::test]
    async fn should_not_receive_block_notification_if_not_subscribed() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();
        let (mut ws, _) = connect_async(devnet.ws_url()).await.unwrap();

        devnet.create_block().await.unwrap();
        assert_no_notifications(&mut ws).await;
    }

    #[tokio::test]
    async fn multiple_block_subscribers_happy_path() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();

        let n_subscribers = 5;

        let mut subscribers = HashMap::new();
        for _ in 0..n_subscribers {
            let (mut ws, _) = connect_async(devnet.ws_url()).await.unwrap();
            let subscription_confirmation =
                send_text_rpc_via_ws(&mut ws, "starknet_subscribeNewHeads", json!({}))
                    .await
                    .unwrap();

            let subscription_id = subscription_confirmation["result"].as_i64().unwrap();
            subscribers.insert(subscription_id, ws);
        }

        assert_eq!(subscribers.len(), n_subscribers); // assert all IDs are different

        let created_block_hash = devnet.create_block().await.unwrap();

        for (subscription_id, mut ws) in subscribers {
            let notification = receive_rpc_via_ws(&mut ws).await.unwrap();
            assert_eq!(notification["method"], "starknet_subscriptionNewHeads");
            assert_eq!(
                notification["params"]["result"]["block_hash"].as_str().unwrap(),
                created_block_hash.to_hex_string().as_str()
            );

            assert_eq!(notification["params"]["result"]["block_number"].as_i64().unwrap(), 1);
            assert_eq!(
                notification["params"]["subscription_id"].as_i64().unwrap(),
                subscription_id
            );
        }
    }

    #[tokio::test]
    async fn subscription_to_an_old_block_by_number_should_notify_of_all_blocks_up_to_latest() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();
        let (mut ws, _) = connect_async(devnet.ws_url()).await.unwrap();

        let n_blocks = 5;
        for _ in 0..n_blocks {
            devnet.create_block().await.unwrap();
        }

        // request notifications for all blocks starting with genesis
        let starting_block = 0;
        let subscription_confirmation = send_text_rpc_via_ws(
            &mut ws,
            "starknet_subscribeNewHeads",
            json!({ "block_id": { "block_number": starting_block } }),
        )
        .await
        .unwrap();
        let subscription_id = subscription_confirmation["result"].as_i64().unwrap();

        for block_i in starting_block..=n_blocks {
            let notification = receive_rpc_via_ws(&mut ws).await.unwrap();
            assert_eq!(notification["method"], "starknet_subscriptionNewHeads");

            assert_eq!(notification["params"]["result"]["block_number"].as_i64().unwrap(), block_i);
            assert_eq!(
                notification["params"]["subscription_id"].as_i64().unwrap(),
                subscription_id
            );
        }

        assert_no_notifications(&mut ws).await;
    }

    #[tokio::test]
    async fn subscription_to_an_old_block_by_hash_should_notify_of_all_blocks_up_to_latest() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();
        let (mut ws, _) = connect_async(devnet.ws_url()).await.unwrap();

        let genesis_block = devnet.get_latest_block_with_tx_hashes().await.unwrap();

        let n_blocks = 5;
        for _ in 0..n_blocks {
            devnet.create_block().await.unwrap();
        }

        // request notifications for all blocks starting with genesis
        let starting_block = 0;
        let subscription_confirmation = send_text_rpc_via_ws(
            &mut ws,
            "starknet_subscribeNewHeads",
            json!({ "block_id": { "block_hash": genesis_block.block_hash } }),
        )
        .await
        .unwrap();
        let subscription_id = subscription_confirmation["result"].as_i64().unwrap();

        for block_i in starting_block..=n_blocks {
            let notification = receive_rpc_via_ws(&mut ws).await.unwrap();
            assert_eq!(notification["method"], "starknet_subscriptionNewHeads");

            assert_eq!(notification["params"]["result"]["block_number"].as_i64().unwrap(), block_i);
            assert_eq!(
                notification["params"]["subscription_id"].as_i64().unwrap(),
                subscription_id
            );
        }

        assert_no_notifications(&mut ws).await;
    }

    #[tokio::test]
    async fn subscription_to_pending_block_is_same_as_latest() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();
        let (mut ws_latest, _) = connect_async(devnet.ws_url()).await.unwrap();
        let (mut ws_pending, _) = connect_async(devnet.ws_url()).await.unwrap();

        // create two subscriptions: one to latest, one to pending
        let subscription_latest = send_text_rpc_via_ws(
            &mut ws_latest,
            "starknet_subscribeNewHeads",
            json!({ "block_id": "latest" }),
        )
        .await
        .unwrap();
        let subscription_id_latest = subscription_latest["result"].as_i64().unwrap();

        let subscription_pending = send_text_rpc_via_ws(
            &mut ws_pending,
            "starknet_subscribeNewHeads",
            json!({ "block_id": "pending" }),
        )
        .await
        .unwrap();
        let subscription_id_pending = subscription_pending["result"].as_i64().unwrap();

        assert_ne!(subscription_id_latest, subscription_id_pending);

        devnet.create_block().await.unwrap();

        // assert notification equality after taking subscription IDs out
        let mut notification_latest = receive_rpc_via_ws(&mut ws_latest).await.unwrap();
        assert_eq!(notification_latest["params"]["subscription_id"].take(), subscription_id_latest);
        assert_no_notifications(&mut ws_latest).await;

        let mut notification_pending = receive_rpc_via_ws(&mut ws_pending).await.unwrap();
        assert_eq!(
            notification_pending["params"]["subscription_id"].take(),
            subscription_id_pending
        );
        assert_no_notifications(&mut ws_pending).await;

        assert_eq!(notification_latest, notification_pending);
    }

    #[tokio::test]
    async fn test_multiple_subscribers_one_unsubscribes() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();

        let n_subscribers = 3;

        let mut subscribers = HashMap::new();
        for _ in 0..n_subscribers {
            let (mut ws, _) = connect_async(devnet.ws_url()).await.unwrap();
            let subscription_confirmation =
                send_text_rpc_via_ws(&mut ws, "starknet_subscribeNewHeads", json!({}))
                    .await
                    .unwrap();

            let subscription_id = subscription_confirmation["result"].as_i64().unwrap();
            subscribers.insert(subscription_id, ws);
        }

        assert_eq!(subscribers.len(), n_subscribers); // assert all IDs are different

        // randomly choose one subscriber for unsubscription
        let unsubscriber_id = *subscribers.keys().next().expect("Should have at least one");

        // unsubscribe
        let mut unsubscriber_ws = subscribers.remove(&unsubscriber_id).unwrap();
        let unsubscription_resp = send_text_rpc_via_ws(
            &mut unsubscriber_ws,
            "starknet_unsubscribe",
            json!({ "subscription_id": unsubscriber_id }),
        )
        .await
        .unwrap();
        assert_eq!(unsubscription_resp, json!({ "jsonrpc": "2.0", "id": 0, "result": true }));

        // create block and assert only subscribers are notified
        let created_block_hash = devnet.create_block().await.unwrap();

        for (subscription_id, mut ws) in subscribers {
            let notification = receive_rpc_via_ws(&mut ws).await.unwrap();
            assert_eq!(notification["method"], "starknet_subscriptionNewHeads");
            assert_eq!(
                notification["params"]["result"]["block_hash"].as_str().unwrap(),
                created_block_hash.to_hex_string().as_str()
            );
            assert_eq!(
                notification["params"]["subscription_id"].as_i64().unwrap(),
                subscription_id
            );
        }

        assert_no_notifications(&mut unsubscriber_ws).await;
    }

    #[tokio::test]
    async fn test_unsubscribing_invalid_id() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();
        let (mut ws, _) = connect_async(devnet.ws_url()).await.unwrap();

        let dummy_id = 123;

        let unsubscription_resp = send_text_rpc_via_ws(
            &mut ws,
            "starknet_unsubscribe",
            json!({ "subscription_id": dummy_id }),
        )
        .await
        .unwrap();

        assert_eq!(
            unsubscription_resp,
            json!({
                "jsonrpc": "2.0",
                "id": 0,
                "error": {
                    "code": 66,
                    "message": "Invalid subscription id",
                }
            })
        );
    }
}