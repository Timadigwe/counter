#![cfg_attr(not(feature = "std"), no_std)]

//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType)]
pub struct State {
    // Your state
    counter: i8,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serialize, SchemaType)]
pub enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams,
    /// Your error
    OwnerError,
    IncrementError,
    DecrementError
}

/// Init function that creates a new smart contract.
#[init(contract = "counter_2")]
fn init(_ctx: &InitContext, _state_builder: &mut StateBuilder) -> InitResult<State> {
    // Your code

    Ok(State {
      counter: 0  
    })
}

pub type IncrementVal = i8;

#[receive(
    contract = "counter_2",
    name = "increment",
    parameter = "IncrementVal",
    error = "Error",
    mutable
)]
fn increment(ctx: &ReceiveContext, host: &mut Host<State>) -> Result<(), Error> {
    // Your code
    let param: IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(ctx.sender().matches_account(&ctx.owner()), Error::OwnerError);
    ensure!(param > 0, Error::ParseParams);
    state.counter  += param;
    Ok(())
}

#[receive(contract = "counter_2", name = "decrement", parameter = "IncrementVal", error = "Error", mutable)]
fn decrement(ctx: &ReceiveContext, host: &mut Host<State>) -> Result<(), Error> {
    let param :IncrementVal = ctx.parameter_cursor().get()?;
    let state = host.state_mut();
    ensure!(ctx.sender().matches_account(&ctx.owner()), Error::OwnerError);
    ensure!(param < 0, Error::DecrementError);
    state.counter += param;
    Ok(())
}

#[receive(contract = "counter_2", name = "view", return_value = "i8")]
fn view<'a, 'b>(_ctx: &'a ReceiveContext, host: &'b Host<State>) -> ReceiveResult<i8> {
    Ok(host.state().counter)
}


//concordium-client account show ACCOUNT --grpc-port 20000 --grpc-ip node.testnet.concordium.com
//Module reference d43fd5c0e4e557077a89d9aff9186158f9fbecec38d6b0c05df28470e4818d0b was successfully named 'counter_1'.
//Module reference 52f60c20223b668910cea6959ef4e551ea8092288b29cd99000784e580cbd5bb was successfully named 'counter_2'.
//Contract successfully initialized with address: {"index":7374,"subindex":0}