//! tests is a special folder in rust .

use candid::{CandidType, Decode, Encode, Nat};
use ic_agent::{export::Principal, Agent};
use serde::Deserialize;

#[derive(CandidType)]
struct Argument {
  amount: Option<Nat>,
}

#[derive(CandidType, Deserialize)]
struct CreateCanisterResult {
  canister_id: Principal,
}

async fn create_a_canister() -> Result<Principal, Box<dyn std::error::Error>> {
  // should do this or check if ic-replica is running : dfx start --background
  // and which port is it ? or how to specify ?
  //   Agent:A low level Agent to make calls to a Replica endpoint.
  let agent = Agent::builder()
    .with_url("http://localhost:8000") // Replace with the appropriate URL
    // ! pgrep replica  # get pid from name
    // !  lsof -i -P -n | grep LISTEN | grep <pid>
    // .with_identity(create_identity())
    .build()?;
  // Only do the following call when not contacting the IC main net (e.g. a
  // local emulator). This is important as the main net public key is static
  // and a rogue network could return a different key.
  // If you know the root key ahead of time, you can use
  // `agent.set_root_key(root_key);`.
  agent.fetch_root_key().await?;
  let management_canister_id = Principal::from_text("aaaaa-aa")?;

  // Create a call to the management canister to create a new canister ID,
  // and wait for a result.
  // The effective canister id must belong to the canister ranges of the subnet
  // at which the canister is created.
  let effective_canister_id =
    Principal::from_text("rwlgt-iiaaa-aaaaa-aaaaa-cai").unwrap();
  let response = agent
    .update(
      &management_canister_id,
      "provisional_create_canister_with_cycles",
    )
    .with_effective_canister_id(effective_canister_id)
    .with_arg(Encode!(&Argument { amount: None })?)
    .call_and_wait()
    .await?;

  let result = Decode!(response.as_slice(), CreateCanisterResult)?;
  let canister_id: Principal = result.canister_id;
  Ok(canister_id)
}

#[tokio::test]
// not #[cfg(test)] //unit-test attribute(#) in rust syntax
// #[tokio::test1]
// async fn test1() {
async fn test1() {
  test_run().await;
}
// if not await , test_run return nothing.

async fn test_run() {
  let canister_id = create_a_canister().await.unwrap();
  eprintln!("{}", canister_id);
}