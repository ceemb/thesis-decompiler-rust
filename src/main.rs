// #[derive(hDebug, Clone, Copy)]
mod xnap {
  pub mod src {
    pub mod message_factory;
    pub mod message_base;
    pub mod xn_setup_request;
    pub mod global_gnb_id;
    pub mod decompile_gnb_id;
    pub mod plmn_id;
    pub mod tai_support_item;
    pub mod broadcast_plmn_in_tai_support;
    pub mod xnap_s_nssai;
    pub mod decompile_tai_support_list;
    pub mod decompile_asn_types;
    pub mod served_cell_info;
    pub mod decompile_served_cells;
    pub mod decompile_plmn_id;
    pub mod to_int;
    pub mod nr_cgi;
    pub mod array_from_length_value;
    pub mod nr_mode_info_tdd;
    pub mod nr_frequency_info;
    pub mod nr_transmission_bandwidth;
  }
}
use core::ffi::c_void;
use std::env;
use std::time::Instant;
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;
mod bindings;
use bindings::{wrapper_create_context, wrapper_asn_decode, wrapper_free_asn_message, wrapper_delete_context, wrapperXnapPdu_create};
use xnap::src::message_factory;
use crate::xnap::src::xn_setup_request::PrintMessage;
pub struct WrapperXnapPdu {
  pub obj: *mut c_void
}
impl WrapperXnapPdu {
  fn new(obj: *mut c_void) -> Self {
    Self {
      obj: obj,
    }
  }
}
fn main() {
   unsafe {
    let args: Vec<String> = env::args().collect();
    let mut i: i32 = 0;
    let mut d: i32 = 0;
    for (index, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-i" => i = args[index + 1].parse().unwrap(),
            "-d" => d = args[index + 1].parse().unwrap(),
            _ => {}
        }
    }
    let ctxt = wrapper_create_context();
    if ctxt.is_null() {
      panic!("Failed to create context");
    }
    let mut duration_ns: Vec<u128> = Vec::new();
    // let mut decoding_times: Vec<Vec<u128>> = Vec::new();
    // let mut decompilation_times: Vec<Vec<u128>> = Vec::new();
    let mut start_time = Instant::now();
    let mut start_decode_time = Instant::now();
    let mut decode_time = Instant::now(); 
    let mut decompile_time = Instant::now();
    let mut end_time = Instant::now();
    let mut output_file = File::create("output.csv").expect("Unable to create file");
    output_file.write_all(b"Iteration, decoding average, decompiling average, decompile [%]\n").expect("Unable to write to file");
    for argd in 0..d {
      let mut decoding: Vec<u128> = Vec::new();
      let mut decompilation: Vec<u128> = Vec::new();
      start_time = Instant::now();
      for _argi in 0..i {
        let mut buffer = vec![0x00, 0x11, ...]; // redacted
        // println!("Loop count: {}", argi);
        start_decode_time = Instant::now();
        let decoded_msg = wrapper_asn_decode(ctxt, &mut buffer[0], buffer.len() as u64);
        if decoded_msg.is_null() {
          panic!("Failed to decode message");
        }
        decode_time = Instant::now();
        let message = WrapperXnapPdu::new(wrapperXnapPdu_create(decoded_msg));
        let decompiled_message = message_factory::create(message);
        decompiled_message.print_message();
        decompile_time = Instant::now();
        decoding.push(decode_time.duration_since(start_decode_time).as_nanos());
        decompilation.push(decompile_time.duration_since(decode_time).as_nanos());
        wrapper_free_asn_message(decoded_msg);
      }

      // decoding_times.push(decoding); // replaced with calc average
      let sum: u128 = decoding.iter().sum();
      let decomp_sum: u128 = decompilation.iter().sum();
      let decode_average = sum as f64/i as f64;
      let decompile_average = decomp_sum as f64/i as f64;
      // decompilation_times.push(decompilation); //replaced with calc average
      end_time = Instant::now();
      // duration_ns.push(end_time.duration_since(start_time).as_nanos());
      
      // write decoding and decompile averages to CSV
      output_file.write_fmt(format_args!("{},{},{},{}\n", argd, decode_average, decompile_average, 100.0*decompile_average/(decode_average+decompile_average))).expect("Unable to write to file");
      let duration = Duration::from_nanos((1000000000-end_time.duration_since(start_time).as_nanos()) as u64);
      sleep(duration);
    }
    // write to output file (CSV)
    // for argd in 0..d {
        
        // for argi in 0..i {
        //     output_file.write_fmt(format_args!("{},-,{},{},{},{}\n",
        //         argd,
        //         argi,
        //         decoding_times[argd as usize][argi as usize],
        //         decompilation_times[argd as usize][argi as usize],
        //         100 * decompilation_times[argd as usize][argi as usize] / (decompilation_times[argd as usize][argi as usize] + decoding_times[argd as usize][argi as usize])
        //     )).expect("Unable to write to file");
        // }
    // }
    output_file.flush().expect("Unable to flush file");
    println!("Iteration {:?}", decompilation_times);
    wrapper_delete_context(ctxt);
  }
}