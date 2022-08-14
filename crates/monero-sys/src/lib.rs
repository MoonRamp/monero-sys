use std::pin::Pin;

use anyhow::anyhow;
use autocxx::prelude::*;

autocxx::include_cpp! {
  #include "wallet/api/wallet2_api.h"
  safety!(unsafe)
  generate_ns!("Monero")
}

pub use ffi::Monero::NetworkType;
use ffi::ToCppString;

pub struct Wallet<'a> {
    inner: Option<Pin<&'a mut ffi::Monero::Wallet>>,
}

impl<'a> Wallet<'a> {
    pub fn new(
        path: &str,
        password: &str,
        lang: &str,
        network: NetworkType,
        kdf_rounds: u64,
    ) -> anyhow::Result<Wallet<'a>> {
        let w = unsafe {
            let f = ffi::Monero::WalletManagerFactory::getWalletManager();
            let p = Pin::new_unchecked(&mut *f);
            p.createWallet1(
                &path.into_cpp(),
                &password.into_cpp(),
                &lang.into_cpp(),
                network,
                kdf_rounds,
            )
        };

        if w.is_null() {
            Err(anyhow!(
                "Error creating wallet at path {} for network {:?}",
                path,
                network
            ))
        } else {
            let w = unsafe { Pin::new_unchecked(&mut *w) };
            let status = w.status().0;
            if status != 0 {
                Err(anyhow!(
                    "Error creating wallet at path {} for network {:?} - '{} (Status Code {})'",
                    path,
                    network,
                    w.errorString(),
                    status,
                ))
            } else {
                Ok(Wallet { inner: Some(w) })
            }
        }
    }

    pub fn address(&self) -> anyhow::Result<String> {
        match &self.inner {
            Some(w) => Ok(w.mainAddress().to_string()),
            None => Err(anyhow!("Invalid wallet")),
        }
    }
}

impl<'a> Drop for Wallet<'a> {
    fn drop(&mut self) {
        let p = unsafe {
            let f = ffi::Monero::WalletManagerFactory::getWalletManager();
            Pin::new_unchecked(&mut *f)
        };
        match self.inner.take() {
            Some(inner) => unsafe {
                let w = Pin::into_inner_unchecked(inner);
                p.closeWallet(w, true);
            },
            None => (),
        }
    }
}

#[test]
fn test_create_wallet() {
    assert!(Wallet::new("./test2.xmr", "12345", "english", NetworkType::MAINNET, 1).is_ok());
}
