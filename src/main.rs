// For more on CPUID and Processor Extended State Enumeration Sub-leaves, see Table 1-5 (pg. 28) in
// https://software.intel.com/sites/default/files/managed/c5/15/architecture-instruction-set-extensions-programming-reference.pdf

fn main() {    
    // For supported state component enumeration, see 13.4.3 in
    // https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-vol-1-manual.pdf
    let state_comps = [2, 5, 6, 7, 9];
    
    for comp in state_comps.iter() {
        // find state component size and offset
        let res = unsafe { core::arch::x86_64::__cpuid_count(0x0d, *comp) };
        println!("State component {} size: {}, offset: {}", comp, res.eax, res.ebx);
    }
}
