use core::marker::PhantomData;
use alloc::vec::Vec;
use axerrno::AxResult;

use super::AxArchVCpuImpl;
use crate::arch::aarch64::vcpu::VCpu;
use crate::AxVMHal;
use arm_vgic;
use alloc::sync::Arc;
use axvcpu::AxVCpu;
use emu_dev::EmuContext;

pub struct AxArchDeviceList<H: AxVMHal> {
    vgic: arm_vgic::Vgic<AxArchVCpuImpl<H>>,
    _marker: core::marker::PhantomData<H>,
}

impl<H: AxVMHal> AxArchDeviceList<H> {
    pub fn new(list : Vec<Arc<AxVCpu<AxArchVCpuImpl<H>>>>) -> Self {
        let mut arch_vcpu_list = Vec::new();

        for arch_vcpu_item in list {
            let unsafe_cell_vcpu = unsafe { &*arch_vcpu_item.get_arch_vcpu() };
            let vcpu = unsafe_cell_vcpu.clone().into();
            arch_vcpu_list.push(vcpu);
        }

        arm_vgic::vgic_init();
        use crate::arch::aarch64::vgic_impl::emu_intc_init;
        Self {
            vgic: emu_intc_init(0x800_0000, 0x4_0000, &arch_vcpu_list),
            _marker: PhantomData,
        }
    }

    pub fn vmexit_handler(
        &self,
        vcpu: &mut VCpu<H>,
        exit_reason: & axvcpu::AxVCpuExitReason,
    ) -> AxResult {
        match exit_reason {
            axvcpu::AxVCpuExitReason::MmioRead { addr, width , data} => {
                let emu_ctx = EmuContext{
                    address: (*addr).into(),
                    width: (*width).into(),
                    write: false,
                    sign_ext: false, // not use 
                    reg: *data as usize,
                    reg_width: 0
                };
                self.vgic.handler(&emu_ctx, vcpu);
            },
            axvcpu::AxVCpuExitReason::MmioWrite { addr, width, data } => {
                let emu_ctx = EmuContext{
                    address: (*addr).into(),
                    width: (*width).into(),
                    write: true,
                    sign_ext: false, // not use 
                    reg: *data as usize,
                    reg_width: 0
                };
                self.vgic.handler(&emu_ctx, vcpu);
            },
            _ => (),
        }
        
        Ok(())
    }
}
