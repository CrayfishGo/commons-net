//! # FTP Protocol
//! ftp is an FTP client written in Rust.
//!
//! ### Usage
//!
//! Here is a basic usage example:
//!
//! ```rust,no_run
//! use commons_net::ftp::ftp_client::FtpClient;
//! use commons_net::ftp::types::FtpError;
//! async fn test_ftp(addr: &str, user: &str, pass: &str) -> Result<(), FtpError> {
//! let mut ftp_client = FtpClient::connect((addr, 21)).await?;
//!     ftp_client.login(user, pass).await?;
//!     println!("current dir: {}", ftp_client.pwd().await?);
//!
//!     // ftp_stream.make_directory("test_data").await?;
//!
//!     ftp_client.cwd("RCX/03021008-RCX/A00").await?;
//!
//!     println!("current dir: {}", ftp_client.pwd().await?);
//!     // An easy way to retrieve a File
//!     let cursor = ftp_client.simple_retr("03021008-RCX_A00(0000).txt").await?;
//!     let vec = cursor.into_inner();
//!     let text = str::from_utf8(&vec).unwrap();
//!     println!("got data: {}", text);
//!     ftp_client.logout().await?;
//!     Ok(())
//! }
//!
//! fn main() {
//!     let future = test_ftp("192.168.32.204", "ftpuser", "ftp123");
//!
//!     tokio::runtime::Builder::new_current_thread()
//!         .enable_all()
//!         .build()
//!         .unwrap()
//!         .block_on(future)
//!         .unwrap();
//!
//!     println!("test successful")
//! }
//! ```
//!
//!
//! ### FTPS
//!
//! The client supports FTPS on demand. To enable it the client should be
//! compiled with feature `openssl` enabled what requires
//! [openssl](https://crates.io/crates/openssl) dependency.
//!
//! The client uses explicit mode for connecting FTPS what means you should
//! connect the server as usually and then switch to the secure mode (TLS is used).
//! For better security it's the good practice to switch to the secure mode
//! before authentication.
//!
//! ### FTPS Usage
//!
//! ```rust,no_run
//! use std::convert::TryFrom;
//! use std::path::Path;
//! use tokio_rustls::rustls::{ClientConfig, RootCertStore, ServerName};
//! use commons_net::ftp::ftp_client::FtpClient;
//!
//! async {
//!   let ftp_client = FtpClient::connect("192.168.32.204:21").await.unwrap();
//!
//!   let mut root_store = RootCertStore::empty();
//!   // root_store.add_pem_file(...);
//!   let conf = ClientConfig::builder().with_safe_defaults().with_root_certificates(root_store).with_no_client_auth();
//!   let domain = ServerName::try_from("www.cert-domain.com").expect("invalid DNS name");
//!
//!   // Switch to the secure mode
//!   let mut ftp_client = ftp_client.into_secure(conf, domain).await.unwrap();
//!   ftp_client.login("anonymous", "anonymous").await.unwrap();
//!   // Do other secret stuff
//!   // Switch back to the insecure mode (if required)
//!   let mut ftp_client = ftp_client.into_insecure().await.unwrap();
//!   // Do all public stuff
//!   let _ = ftp_client.quit().await;
//! };
//! ```
//!
//!
//! # SMTP Protocol
//! asap...

pub mod ftp;
pub mod smtp;

pub trait StringExt {
    fn substring(&self, start_index: usize, end_index: usize) -> &str;
}

impl StringExt for str {
    fn substring(&self, start_index: usize, end_index: usize) -> &str {
        if end_index <= start_index {
            return "";
        }

        let mut indices = self.char_indices();

        let obtain_index = |(index, _char)| index;
        let str_len = self.len();

        unsafe {
            self.slice_unchecked(
                indices.nth(start_index).map_or(str_len, &obtain_index),
                indices
                    .nth(end_index - start_index - 1)
                    .map_or(str_len, &obtain_index),
            )
        }
    }
}