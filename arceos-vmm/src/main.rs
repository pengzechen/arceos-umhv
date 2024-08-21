#![no_std]
#![no_main]

#[macro_use]
extern crate log;
#[macro_use]
extern crate alloc;
extern crate axstd as std;

mod hal;
mod task;
mod vmm;

use axvm::AxVMPerCpu;

use crate::hal::AxVMHalImpl;

#[percpu::def_percpu]
pub static mut AXVM_PER_CPU: AxVMPerCpu<AxVMHalImpl> = AxVMPerCpu::new_uninit();

#[no_mangle]
fn main() {
    /* === temp init gic start ==== */
    use arm_gic::GicDistributor;
    use arm_gic::GicCpuInterface;
    GicDistributor::init_base(0x800_0000 as * mut u8);
    GicCpuInterface::init_base(0x801_0000 as * mut u8);
    /* === temp init gic end ==== */
    
    info!("Starting virtualization...");

    // TODO: remove this to somewhere else like `percpu.hardware_enable()`.
    info!("Hardware support: {:?}", axvm::has_hardware_support());

    // Init hardware virtualization support in each core.
    // Note: This is awkward because we need to find a proper place to call this on each core.
    let percpu = unsafe { AXVM_PER_CPU.current_ref_mut_raw() };
    percpu.init(0).expect("Failed to initialize percpu state");
    percpu
        .hardware_enable()
        .expect("Failed to enable virtualization");

    vmm::init();

    vmm::start();

    // Todo: move this to `vmm::start()`.
    axtask::WaitQueue::new().wait();

    unreachable!("VMM start failed")
}
