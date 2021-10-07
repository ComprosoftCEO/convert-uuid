use std::str::FromStr;
use structopt::StructOpt;
use uuid::Uuid;
use uuid_b64::UuidB64;

/// Convert between normal and base-64 UUID representations.
/// UUIDs can be entered using 32-character, 36-character, or base-64 representation.
#[derive(Debug, StructOpt)]
struct Opt {
  /// UUID to convert  (Generates a random UUID if unset)
  input: Option<SerializeUuid>,
}

/// Special struct that can serialize both normal and base-64
#[derive(Debug)]
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
    .unwrap_or_else(|| {
      println!("=== Random UUID: ===");
      SerializeUuid(UuidB64::new())
    })
    .0;

  println!("Normal: {}", uuid.uuid());
  println!("Base64: {}", uuid);
}
