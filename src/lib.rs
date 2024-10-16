#![no_std]

extern crate alloc;

pub mod transport {
    pub mod frame;
    pub mod frame_flag;
    pub mod packet;
    pub mod serice_index;
    pub mod service_command;
    pub mod transport_error;
}

pub mod service {
    pub mod button;
    pub mod control;
    pub mod packet;
    pub mod packet_type;
    pub mod reports;
    pub mod serivce_error;
    pub mod service;
}

pub mod brain;
