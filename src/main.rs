use objc2_application_services::AXIsProcessTrusted;
use objc2_core_graphics::{CGPreflightScreenCaptureAccess, CGRequestScreenCaptureAccess};

fn check_permissions() {
    unsafe {
        if !CGPreflightScreenCaptureAccess() {
            if !CGRequestScreenCaptureAccess() {
                println!("failed to access accessability");
                std::process::exit(1);
            }
        }
        if !AXIsProcessTrusted() {
            println!("not trusted");
            std::process::exit(1);
        }
    }
}

fn main() {
    check_permissions();
}
