# BNR API

This library presents a high-level, safe, idiomatic Rust API for interacting with MEI/CPI BNR devices.

## Library organization

The structure of the project's file tree roughly follows the organization of the MEI/CPI modules.

Related types are grouped together in modules. Larger modules are further split into their own set of modules, and glob-imported by the parent module.

## Future goals

The ultimate goal is to cover a large portion of the API surface, and then begin to implement all functionality in Rust.

Enough functionality is implemented to perform device initialization, configuration, deposit, dispense, and reset.

There is still a reasonable amount left to implement.
