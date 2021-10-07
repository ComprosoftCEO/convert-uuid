use std::str::FromStr;
use structopt::StructOpt;
use uuid::Uuid;
use uuid_b64::UuidB64;

// Packages needed only for the clipboard
#[cfg(feature = "clipboard")]
use rust_clipboard::{ClipboardContext, ClipboardProvider};
#[cfg(feature = "clipboard")]
use std::error::Error;

/// Convert between normal and base-64 UUID representations.
///
/// UUIDs can be entered using 32-character, 36-character, or base-64 representation.
#[derive(Debug, StructOpt)]
struct Opt {
  /// UUID to convert  (Generates a random UUID if unset)
  input: Option<SerializeUuid>,

  #[cfg(feature = "clipboard")]
  /// Copy the normal UUID to the system clipboard
  #[structopt(short = "n", long, conflicts_with = "copy-b64")]
  copy_normal: bool,

  #[cfg(feature = "clipboard")]
  /// Copy the base-64 UUID to the system clipboard
  #[structopt(short = "b", long, conflicts_with = "copy-normal")]
  copy_b64: bool,
}

/// Special struct that can serialize both normal and base-64
#[derive(Debug, Clone)]
struct SerializeUuid(UuidB64);

impl FromStr for SerializeUuid {
  type Err = String;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    // Try to serialize normally
    let uuid_error = match Uuid::from_str(input) {
      Ok(uuid) => return Ok(SerializeUuid(UuidB64::from(uuid))),
      Err(e) => e,
    };

    // Try to serialize base-64
    let uuid_b64_error = match UuidB64::from_str(input) {
      Ok(uuid_b64) => return Ok(SerializeUuid(uuid_b64)),
      Err(e) => e,
    };

    Err(format!(
      "Invalid UUID string: {}\n  {}\n  {}",
      input, uuid_error, uuid_b64_error
    ))
  }
}

fn main() {
  let opt = Opt::from_args();

  // Get the UUID or pick a random one
  let uuid = opt
    .input
    .clone()
    .unwrap_or_else(|| {
      println!("=== Random UUID: ===");
      SerializeUuid(UuidB64::new())
    })
    .0;

  // Display both representations
  println!("Normal: {}", uuid.uuid());
  println!("Base64: {}", uuid);

  // Possibly copy to clipboard
  #[cfg(feature = "clipboard")]
  if opt.copy_normal || opt.copy_b64 {
    let clipboard_result: Result<(), Box<dyn Error>> = (|| {
      let mut ctx: ClipboardContext = ClipboardProvider::new()?;

      if opt.copy_normal {
        ctx.set_contents(uuid.uuid().to_string())?;
      }
      if opt.copy_b64 {
        ctx.set_contents(uuid.to_string())?;
      }

      Ok(())
    })();

    if let Err(e) = clipboard_result {
      println!("Failed to copy to clipboard: {}", e);
    } else {
      println!("Copied to clipboard!");
    }
  }
}
