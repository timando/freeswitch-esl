#![deny(missing_docs)]

//! Esl Create for interacting with freeswitch
//!
//! # Examples
//!
//! ## Inbound Connection
//!
//!```rust,no_run
//! use freeswitch_esl::{Esl, EslError};
//! use tokio::net::TcpStream;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), EslError> {
//!     let addr = "localhost:8021"; // Freeswitch host
//!     let password = "ClueCon";
//!     let stream = TcpStream::connect(addr).await?;
//!     let inbound = Esl::inbound(stream, "ClueCon").await?;
//!
//!     let reloadxml = inbound.api("reloadxml").await?;
//!     println!("reloadxml response : {:?}", reloadxml);
//!
//!     let reloadxml = inbound.bgapi("reloadxml").await?;
//!     println!("reloadxml response : {:?}", reloadxml);
//!
//!     Ok(())
//! }
//! ```
//! ## Outbound Connection
//!
//!```rust,no_run
//! use freeswitch_esl::{Esl, EslConnection, EslError};
//! use tokio::net::TcpListener;
//!
//! async fn process_call(conn: EslConnection) -> Result<(), EslError> {
//!     conn.answer().await?;
//!     println!("answered call");
//!     conn.playback("ivr/ivr-welcome.wav").await?;
//!     let digit = conn
//!         .play_and_get_digits(
//!             1,
//!             1,
//!             3,
//!             3000,
//!             "#",
//!             "conference/conf-pin.wav",
//!             "conference/conf-bad-pin.wav",
//!         )
//!         .await?;
//!     println!("got digit {}", digit);
//!     conn.playback("ivr/ivr-you_entered.wav").await?;
//!     conn.playback(&format!("digits/{}.wav", digit)).await?;
//!     conn.hangup("NORMAL_CLEARING").await?;
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), EslError> {
//!     let addr = "0.0.0.0:8085"; // Listening address
//!     println!("Listening on {}", addr);
//!     let listener = TcpListener::bind(addr).await?;
//!
//!     loop {
//!         let (socket, _) = listener.accept().await?;
//!         tokio::spawn(async move {
//!             let stream = Esl::outbound(socket).await.unwrap();
//!             process_call(stream).await.unwrap();
//!         });
//!     }
//!     
//! }
//! ```

pub(crate) mod code;
pub(crate) mod connection;
pub(crate) mod dp_tools;
pub(crate) mod error;
pub(crate) mod esl;
pub(crate) mod event;
pub(crate) mod io;

pub use connection::EslConnection;
pub use connection::EslConnectionSimple;
pub use error::*;
pub use esl::*;
pub use event::*;
