
use arm_vgic::VcpuTrait;
use super::vcpu::VCpu;
use crate::AxVMHal;

impl<H: AxVMHal> VcpuTrait for VCpu<H> {
    
    fn if_id(&self) -> usize { 
        self.vcpu_id 
    }
    fn if_vm_id(&self) ->usize { 
        0 
    }
    fn if_phys_id(&self) ->usize { 
        0
    }

    fn if_get_gpr(&self, idx: usize) -> usize {
        self.ctx.gpr(idx)
    }
    fn if_set_gpr(&mut self, idx: usize, val: usize) {
        self.ctx.set_gpr(idx, val)
    }
}

use alloc::vec::Vec;
use arm_vgic::Vgic;
use arm_vgic::vint::VgicInt;
const GIC_SPI_MAX:usize = 1024;
const GIC_PRIVINT_NUM:usize = 32;
const GIC_SGIS_NUM:usize = 16;



pub fn emu_intc_init<V: VcpuTrait + Clone>(base_ipa: usize, length: usize, vcpu_list: &Vec<V>) -> Vgic<V> {

    let vcpu_num = vcpu_list.len();
    info!("vcpu_num: {}, base_ipa: {:x}, length: {:x}", vcpu_num, base_ipa, length);
    let mut vgic: Vgic<V> = Vgic::new(base_ipa, length, vcpu_num);
    
    for i in 0..GIC_SPI_MAX {
        vgic.vgicd.interrupts.push(VgicInt::<V>::new(i));
    }

    for vcpu in vcpu_list {
        let mut cpu_priv = arm_vgic::vint_private::VgicCpuPriv::default();

        
        for int_idx in 0..GIC_PRIVINT_NUM {
            let phys_id: usize = vcpu.if_phys_id();
            cpu_priv.interrupts.push(VgicInt::<V>::priv_new(
                int_idx,
                vcpu.clone(),
                1 << phys_id,
                int_idx < GIC_SGIS_NUM,
            ));
        }

        vgic.cpu_priv.push(cpu_priv);
    }

    
    vgic
}