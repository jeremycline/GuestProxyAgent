// Copyright (c) Microsoft Corporation
// SPDX-License-Identifier: MIT
#![deny(warnings)]

pub mod acl;
pub mod common;
pub mod host_clients;
pub mod key_keeper;
pub mod monitor;
pub mod provision;
pub mod proxy;
pub mod proxy_agent_status;
pub mod redirector;
pub mod service;
pub mod telemetry;
pub mod test_mock;

use common::helpers;
use proxy_agent_shared::misc_helpers;
use std::{process, time::Duration};

#[cfg(windows)]
use common::{constants, logger};
#[cfg(windows)]
use service::windows;
#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use windows_service::{define_windows_service, service_dispatcher};
#[cfg(windows)]
define_windows_service!(ffi_service_main, proxy_agent_windows_service_main);

fn main() {
    // start the Instant to calculate the elapsed time
    _ = helpers::get_elapsed_time_in_millisec();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1].to_lowercase() == "console" {
            let handle = service::start_service();
            println!("Press Enter to end it.");
            let mut temp = String::new();
            _ = std::io::stdin().read_line(&mut temp);
            handle.stop();
        } else if args[1].to_lowercase() == "--version" {
            println!("{}", misc_helpers::get_current_version());
        } else if args[1].to_lowercase() == "--status" {
            let mut wait_time: u64 = 0;
            if args.len() >= 4 && args[2].to_lowercase() == "--wait" {
                wait_time = args[3].parse::<u64>().unwrap_or(0);
            }
            let status =
                provision::get_provision_status_wait(None, Some(Duration::from_secs(wait_time)));
            if status.0 == false {
                // exit code 1 means provision not finished yet.
                process::exit(1);
            } else {
                // provision finished
                if status.1.len() > 0 {
                    // exit code 2 means provision finished but failed.
                    println!("{}", status.1);
                    process::exit(2);
                }
                // provision finished and success
                return;
            }
        } else {
            println!("Invalid argument: {}", args[1]);
        }
    } else {
        #[cfg(windows)]
        {
            _ = service_dispatcher::start(constants::PROXY_AGENT_SERVICE_NAME, ffi_service_main);
        }

        #[cfg(not(windows))]
        {
            service::start_service_wait();
        }
    }
}

#[cfg(windows)]
fn proxy_agent_windows_service_main(args: Vec<OsString>) {
    // start the Instant to calculate the elapsed time
    _ = helpers::get_elapsed_time_in_millisec();

    if let Err(e) = windows::run_service(args) {
        logger::write_error(format!("{e}"));
    }
}
