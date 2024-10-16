- [jacdac-rs](#jacdac-rs)
  - [Introduction](#introduction)
    - [What is jacdac ?](#what-is-jacdac-)
    - [What is jacdac-rs](#what-is-jacdac-rs)
  - [Features](#features)
    - [Available](#available)
      - [Transport](#transport)
      - [Service](#service)
    - [Planned](#planned)
      - [Transport](#transport-1)
      - [Service](#service-1)
      - [General](#general)


# jacdac-rs
## Introduction
### What is jacdac ?
> Jacdac is an integrated hardware and software stack that makes it easier to connect programmable microcontrollers, sensors, actuators, and peripherals together in a client/server architecture. It also provides a bridge from the embedded world to the web browser and beyond. Key features of Jacdac are:
> 
> - a dedicated 3-wire cable and connector
> - a true bus topology
> - service advertisement and dynamic device discovery
> - a standardized service abstraction
> - a full stack that supports platforms from 8-bit MCUs to web development.
> 
> In addition to the functionality provided by common networking stacks, the Jacdac protocol specifies electrical and mechanical requirements that improve interoperability at the hardware level.

_jacdac Specification [https://microsoft.github.io/jacdac-docs/reference/](https://microsoft.github.io/jacdac-docs/reference/)_

### What is jacdac-rs
jacdac-rs is a library written entirely in Rust, with the aim of providing jacdac support for embedded development.

The library will be [embedded-hal](https://crates.io/crates/embedded-hal) dependent where necessary, to ensure maximum interoperability.

jacdac-rs is in the early stages of development, and the api is subject to major changes between versions.

## Features

### Available

#### Transport
 - [x] Decode jacdac frame from buffer (`&[u8]`)
 - [x] Decode jacdac packet from buffer (`&[u8]`)

#### Service
 - [x] Decode transport packet to service packet

### Planned

#### Transport
 - [ ] Decode jacdac frame from Reader
 - [ ] Decode jacdac packetg from Reader
 - [ ] Encode jacdac frame to buffer
 - [ ] Encode jacdac frame to Writer

#### Service
 - [ ] Register all the current available service from [jacdac services](https://microsoft.github.io/jacdac-docs/services/)

#### General
 - [ ] Add examples