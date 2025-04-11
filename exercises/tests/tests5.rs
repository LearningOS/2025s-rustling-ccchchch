/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
/// The pointer passed as the address should not be null, and it should point to a unique, valid location
/// where a `u32` value can be safely written to.
unsafe fn modify_by_address(address: usize) {
    let ptr = address as *mut u32;  // Convert usize back to a mutable pointer to u32
    // SAFETY: We assume the pointer is valid and properly aligned to point to a u32.
    // Since we are given the contract in the function's documentation, the pointer is valid.
    *ptr = 0xAABBCCDD;  // Modify the value at the given memory address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
