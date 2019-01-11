# `CANUST rust CAN API`

> a API for building CAN applications in rust for the stm32 board.

## Rust versions
`rustc 1.30.0-nightly (2f1547c0a 2018-09-11)`  
`rustup 1.13.0 (ea9259c1b 2018-07-16)`  

## Build
Clone repository  
`git clone https://github.com/superspindel/CANUST.git`  
Change into directory  
`cd CANUST`  
copy example into src  
`cp examples/led_handle.rs main.rs`  
run the code in release mode  
`cargo run --release`  

## Board
The specific board used in the examples was created during embedded course at LTU ( Luleå technological university ).  
It contained a CAN tranciever that would take the output from the can tx pin and map it to the can bus to follow the CAN protocol. Information received would be mapped to the can rx pin.  
For furher understanding of the board created check out  
`https://gitlab.henriktjader.com/E7020E-2018/Project.zip/tree/fiskbranch`.

## Usage of CANUST
This is not a complete API. It's focused on providing the various functionalities of CAN on a stm32f0x board in the most basic of ways. Therefore most of the setup and decision making on registers are left to the user.
By checking the different examples one can see how to setup and use the CAN bus to transfer and receive messages and keep up to date with the different interrupts active on the CAN. 
