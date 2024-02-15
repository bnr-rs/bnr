# BNR XFS

This library is used to communicate with MEI/CPI BNR cash recycler devices.

The XFS protocol is an XML-encoded messaging format over a USB connection.

Communication happens over four endpoints:

- XFS response (device-to-host)
- XFS call (host-to-device)
- XFS callback call (device-to-host)
- XFS callback response (host-to-device)

## Usage

The main interface for device interaction is the [DeviceHandle](src/device_handle.rs):

```rust
use bnr_xfs::{CallbackArg, DeviceHandle};

// Callback handler for when an async call completes
//
// See OperationCompletedFn for details.
fn op_com(_call_id: i32, _op_id: i32, _res: i32, _ext_res: i32, _cb_arg: &mut dyn CallbackArg) {
    // process the completion event...
}

// Callback handler for when an intermediate event occurs
//
// See IntermediateOccurredFn for details.
fn int_oc(_call_id: i32, _op_id: i32, _reason: i32, _cb_arg: &mut dyn CallbackArg) {
    // process the intermediate event...
}

// Callback handler for when a status event occurs
//
// See StatusOccurredFn for details.
fn st_oc(_call_id: i32, _op_id: i32, _reason: i32, _cb_arg: &mut dyn CallbackArg) {
    // process the status event...
}

let device_handle = DeviceHandle::open(Some(op_com), Some(int_oc), Some(st_oc)).unwrap();

let _status = device_handle.get_status().unwrap();
```

## Testing

End-to-end device tests live in the [tests/e2e_tests](tests/e2e_tests) directory.

These tests currently require a BNR device to be connected to the computer running the tests.

To run the tests:

```bash
$ cargo test --all --features e2e_tests
```

## WIP

There is still a reasonable amount of the BNR XFS API surface from the C library that needs to be implemented in Rust.

Implementations should follow the pattern of existing functions, and any additional types need to implement conversion to-and-from XFS XML.

Helper macros exist for implementing conversions of most of the basic XFS types.

If you notice an unimplemented type, please submit a merge request!
