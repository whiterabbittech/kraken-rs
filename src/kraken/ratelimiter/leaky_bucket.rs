#![allow(dead_code)]

use crate::kraken::AccountTier;
use flume::{Receiver, TrySendError};
use std::collections::HashMap;
use std::time::Duration;

/// A LeakyBucket refers to a strategy for rate limiting
/// where a channel of a fixed size is created, and elements
/// are removed from that channels as requests are consumed.
/// When the channel has been emptied, requests have hit their
/// limit. The bucket refills by having a separate thread fill
/// the channel on a given schedule.
pub struct LeakyBucket {
    recv: Receiver<()>,
}

impl LeakyBucket {
    pub fn new(tier: AccountTier) -> Self {
        let config = Self::bucket_configuration(tier);
        let (sender, receiver) = flume::bounded(config.max_size.into());
        // Fill the bucket, initializing the semaphore resources
        for _ in 0..config.max_size {
            sender.send(()).unwrap();
        }
        tokio::spawn(async move {
            loop {
                std::thread::sleep(config.fill_rate);
                let res = sender.try_send(());
                if let Err(TrySendError::Disconnected(_)) = res {
                    println!("Channel closed. Exiting.");
                    break;
                }
            }
        });
        Self { recv: receiver }
    }

    pub async fn consume(&self) {
        self.recv.recv_async().await.unwrap();
    }

    pub async fn use_rate_limit(&self, count: usize) {
        for _ in 0..count {
            self.consume().await;
        }
    }

    fn bucket_configuration(tier: AccountTier) -> BucketDescription {
        let configurations = HashMap::from([
            (
                AccountTier::Starter,
                BucketDescription {
                    max_size: 15,
                    fill_rate: Duration::from_secs(3),
                },
            ),
            (
                AccountTier::Intermediate,
                BucketDescription {
                    max_size: 20,
                    fill_rate: Duration::from_secs(2),
                },
            ),
            (
                AccountTier::Pro,
                BucketDescription {
                    max_size: 20,
                    fill_rate: Duration::from_secs(1),
                },
            ),
        ]);
        let bucket = configurations.get(&tier).unwrap();
        *bucket
    }
}

#[derive(Clone, Copy)]
struct BucketDescription {
    max_size: u8,
    /// fill_rate is the amount of time that must
    /// pass for a single element to be added back into
    /// the bucket.
    fill_rate: Duration,
}
