
pub fn error_code(code: u32) -> String {
    match code {
        0 => "No error has occurred.",
        100 => "Pointer is unexpectedly NULL.",
        110 => "The function attempted to allocate memory and no more memory was available for allocation.",
        111 => "The desired functionality has not been enabled in this virtual machine.",
        112 => "The desired functionality is not available in the current phase. Always returned if the virtual machine has completed running.",
        113 => "An unexpected internal error has occurred.",
        115 => "The thread being used to call this function is not attached to the virtual machine. Calls must be made from attached threads.",
        116 => "The JVM TI environment provided is no longer connected or is not an environment.",
        _ => "Unknown error"
    }.to_string()
}
