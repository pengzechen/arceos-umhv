
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