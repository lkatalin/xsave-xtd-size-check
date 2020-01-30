// For more on CPUID and Processor Extended State Enumeration Sub-leaves, see Table 1-5 (pg. 28) in
// https://software.intel.com/sites/default/files/managed/c5/15/architecture-instruction-set-extensions-programming-reference.pdf

fn main() {
    // read which bits are enabled in xcr0
    let xcr0 = unsafe { core::arch::x86_64::_xgetbv(0) };
    println!("xcr0: {:b}", xcr0);

    // For supported state component enumeration, see 13.4.3 in
    // https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-1-manual.pdf
    // as well as https://en.wikipedia.org/wiki/Control_register
    let state_comps = [2, 5, 6, 7, 9];
    
    // set the bits for relevant state components (2, 5, 6, 7, 9)
    // == 0b1011100100
    unsafe { core::arch::x86_64::_xsetbv(0, 740) };
 
    // re-read xcr0
    let xcr0 = unsafe { core::arch::x86_64::_xgetbv(0) };
    println!("xcr0: {:b}", xcr0);
    
    for comp in state_comps.iter() {
        // find state component size and offset
        let res = unsafe { core::arch::x86_64::__cpuid_count(0x0d, *comp) };
        println!("State component {} size: {}, offset: {}", comp, res.eax, res.ebx);
    }
}
