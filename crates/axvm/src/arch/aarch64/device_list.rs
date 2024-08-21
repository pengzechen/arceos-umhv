use core::marker::PhantomData;
use alloc::vec::Vec;
use axerrno::AxResult;

use super::AxArchVCpuImpl;
use crate::AxVMHal;

pub struct AxArchDeviceList<H: AxVMHal> {
    _marker: core::marker::PhantomData<H>,
}

impl<H: AxVMHal> AxArchDeviceList<H> {
    pub fn new() -> Self {
        let mut arch_vcpu_list = Vec::new();

        for arch_vcpu in list {
            let unsafe_cell_vcpu = unsafe { &*arch_vcpu.arch_vcpu.get() };
            let vcpu = unsafe_cell_vcpu.clone().into();
            arch_vcpu_list.push(vcpu);
        }

        Self {
            vgic: emu_intc_init(0x800_0000, 0x4_0000, &arch_vcpu_list),
            _marker: PhantomData,
        }
    }

    pub fn vmexit_handler(
        &self,
        _arch_vcpu: &mut AxArchVCpuImpl<H>,
        _exit_reason: axvcpu::AxVCpuExitReason,
    ) -> AxResult {
        Ok(())
    }
}
