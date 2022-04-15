use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::tests::Test;

fn append_stress_tests(_target: &mut Vec<Box<dyn Test>>) {
    #[cfg(feature = "vmulf_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMULF {}));
    #[cfg(feature = "vmudh_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMUDH {}));
    #[cfg(feature = "vmudm_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMUDM {}));
    #[cfg(feature = "vmudn_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMUDN {}));
    #[cfg(feature = "vmacf_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMACF {}));
    #[cfg(feature = "vmadh_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMADH {}));
    #[cfg(feature = "vmadm_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMADM {}));
    #[cfg(feature = "vmadn_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMADN {}));
}

#[cfg(feature = "default_tests")]
fn default_tests() -> Vec<Box<dyn Test>> {
    vec! {
        // This should be the overall first test
        Box::new(super::startup::StartupTest {}),
        Box::new(super::address_error_exception::UnalignedLW {}),
        Box::new(super::address_error_exception::UnalignedLW2 {}),
        Box::new(super::address_error_exception::UnalignedLWDelay {}),
        Box::new(super::address_error_exception::UnalignedSW {}),
        Box::new(super::address_error_exception::LWAddressNotSignExtended {}),
        Box::new(super::address_error_exception::SWAddressNotSignExtended {}),
        Box::new(super::cart_memory::LW {}),
        Box::new(super::cart_memory::LH {}),
        Box::new(super::cart_memory::LB {}),
        Box::new(super::cart_memory::write::WriteAndReadback {}),
        Box::new(super::cart_memory::write::WriteAndReadback2 {}),
        Box::new(super::cart_memory::write::WriteAndReadback3 {}),
        Box::new(super::cart_memory::write::WriteAndReadback4 {}),
        Box::new(super::cart_memory::write::WriteAndReadback5 {}),
        Box::new(super::cart_memory::write::DecayAfterSomeClockCycles {}),
        Box::new(super::cart_memory::write::Write32AndReadback8 {}),
        Box::new(super::cart_memory::write::Write32AndReadback16 {}),
        Box::new(super::cart_memory::write::Write8AndReadback32 {}),
        Box::new(super::cart_memory::write::Write8WithOffsetAndReadback32 {}),
        Box::new(super::cart_memory::write::Write16AndReadback32 {}),
        Box::new(super::cart_memory::write::Write64AndReadback32 {}),
        Box::new(super::cop0::ContextMasking {}),
        Box::new(super::cop0::ContextMixedBitWriting {}),
        Box::new(super::cop0::XContextMasking {}),
        Box::new(super::cop0::XContextMaskingMixed {}),
        Box::new(super::cop0::BadVAddrReadOnly {}),
        Box::new(super::cop0::ExceptPCNoMasking {}),
        Box::new(super::cop0::ErrorEPCNoMasking {}),
        Box::new(super::cop0::LLAddrIs32Bit {}),
        Box::new(super::cop0::StatusIs32Bit {}),
        Box::new(super::exception_instructions::Break {}),
        Box::new(super::exception_instructions::BreakDelay {}),
        Box::new(super::exception_instructions::Syscall {}),
        Box::new(super::exception_instructions::SyscallDelay {}),
        Box::new(super::jumps::conditionals::BEQWithinDelay {}),
        Box::new(super::jumps::conditionals::BEQNotTakenWithinDelay {}),
        Box::new(super::jumps::conditionals::BEQWithinDelayOfJR {}),
        Box::new(super::jumps::conditionals::BGEZALNotTaken {}),
        Box::new(super::jumps::conditionals::BGEZALTaken {}),
        Box::new(super::jumps::conditionals::BGEZALThatChangesItsOwnCondition {}),
        Box::new(super::jumps::conditionals::BGEZALLNotTaken {}),
        Box::new(super::jumps::conditionals::BGEZALLTaken {}),
        Box::new(super::jumps::conditionals::BGEZALWithinDelay {}),
        Box::new(super::jumps::conditionals::BGEZALWithinDelayOfBEQ {}),
        Box::new(super::jumps::conditionals::BGEZALNotTakenWithinDelay {}),
        Box::new(super::jumps::j_and_jal::JWithinDelay {}),
        Box::new(super::jumps::j_and_jal::JALWithinDelay {}),
        Box::new(super::jumps::j_and_jal::JALDelayRAVisibility {}),
        Box::new(super::jumps::j_and_jal::JALWithinDelayOfJALR {}),
        Box::new(super::jumps::jr_and_jalr::JALRSimple {}),
        Box::new(super::jumps::jr_and_jalr::JALRWithSameRegister {}),
        Box::new(super::jumps::jr_and_jalr::JALRDelayRAVisibility {}),
        Box::new(super::jumps::jr_and_jalr::JRWithRegisterChangeInDelaySlot {}),
        Box::new(super::jumps::jr_and_jalr::JALRWithRegisterChangeInDelaySlot {}),
        Box::new(super::jumps::jr_and_jalr::JRWithinDelayOfJALR {}),
        Box::new(super::jumps::jr_and_jalr::JALRWithinDelayOfJALR {}),
        Box::new(super::overflow_exception::AddOverflowPositive {}),
        Box::new(super::overflow_exception::AddOverflowNegative {}),
        Box::new(super::overflow_exception::AddOverflowIntoR0 {}),
        Box::new(super::overflow_exception::AddOverflowDelaySlot1 {}),
        Box::new(super::overflow_exception::AddOverflowDelaySlot2 {}),
        Box::new(super::overflow_exception::DoubleAddOverflow {}),
        Box::new(super::overflow_exception::DoubleAddOverflowIntoR0 {}),
        Box::new(super::overflow_exception::SubOverflow {}),
        Box::new(super::overflow_exception::SubOverflowIntoR0 {}),
        Box::new(super::overflow_exception::DoubleSubOverflow {}),
        Box::new(super::overflow_exception::DoubleSubOverflowIntoR0 {}),
        Box::new(super::overflow_exception::AddImmediateOverflow {}),
        Box::new(super::overflow_exception::AddImmediateOverflowIntoR0 {}),
        Box::new(super::overflow_exception::DoubleAddImmediateOverflow {}),
        Box::new(super::overflow_exception::DoubleAddImmediateOverflowIntoR0 {}),
        Box::new(super::pif_memory::SW {}),
        Box::new(super::pif_memory::SH0 {}),
        Box::new(super::pif_memory::SH2 {}),
        Box::new(super::pif_memory::SB {}),
        Box::new(super::pif_memory::LB {}),
        Box::new(super::pif_memory::LH {}),
        // This should be RSP test #1
        Box::new(super::rsp::PCRegMasking {}),
        // This should be RSP test #2
        Box::new(super::rsp::op_break::BREAK {}),
        // This should be RSP test #3
        Box::new(super::rsp::wrap_around::WrapAround {}),
        // Non-vector instructions
        Box::new(super::rsp::op_addi::ADDI {}),
        Box::new(super::rsp::op_addiu::ADDIU {}),
        Box::new(super::rsp::op_andi::ANDI {}),
        Box::new(super::rsp::op_ori::ORI {}),
        Box::new(super::rsp::op_xori::XORI {}),
        Box::new(super::rsp::op_slti::SLTI {}),
        Box::new(super::rsp::op_sltiu::SLTIU {}),
        Box::new(super::rsp::op_lb::LB {}),
        Box::new(super::rsp::op_lh::LH {}),
        Box::new(super::rsp::op_lw::LW {}),
        Box::new(super::rsp::op_lbu::LBU {}),
        Box::new(super::rsp::op_lhu::LHU {}),
        Box::new(super::rsp::op_lwu::LWU {}),
        Box::new(super::rsp::op_sb::SB {}),
        Box::new(super::rsp::op_sh::SHAligned {}),
        Box::new(super::rsp::op_sh::SHUnaligned {}),
        Box::new(super::rsp::op_sw::SWAligned {}),
        Box::new(super::rsp::op_sw::SWUnaligned {}),
        Box::new(super::rsp::op_j::J {}),
        Box::new(super::rsp::op_jal::JAL {}),

        Box::new(super::rsp::op_branches::BEQ {}),
        Box::new(super::rsp::op_branches::BNE {}),
        Box::new(super::rsp::op_branches::BLEZ {}),
        Box::new(super::rsp::op_branches::BGTZ {}),
        Box::new(super::rsp::op_branches::BLTZ {}),
        Box::new(super::rsp::op_branches::BGEZ {}),
        Box::new(super::rsp::op_branches::BLTZAL {}),
        Box::new(super::rsp::op_branches::BGEZAL {}),

        // RSP Vector instructions
        Box::new(super::rsp::op_lqv_sqv::LQVSQV {}),
        Box::new(super::rsp::op_vsar::VSAR {}),
        Box::new(super::rsp::op_vmacf::VMACFAll {}),
        Box::new(super::rsp::op_vmacf::VMACFH0 {}),
        Box::new(super::rsp::op_vmacf::VMACF5 {}),
        Box::new(super::rsp::op_vmacf::VMACFAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmadm::VMADMAll {}),
        Box::new(super::rsp::op_vmadm::VMADM4 {}),
        Box::new(super::rsp::op_vmadm::VMADMAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmadn::VMADNAll {}),
        Box::new(super::rsp::op_vmadn::VMADNH3 {}),
        Box::new(super::rsp::op_vmadn::VMADN6 {}),
        Box::new(super::rsp::op_vmadn::VMADNAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmadh::VMADHAll {}),
        Box::new(super::rsp::op_vmadh::VMADHQ1 {}),
        Box::new(super::rsp::op_vmadh::VMADH7 {}),
        Box::new(super::rsp::op_vmadh::VMADHAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmudh::VMUDHAll {}),
        Box::new(super::rsp::op_vmudh::VMUDHQ1 {}),
        Box::new(super::rsp::op_vmudh::VMUDH7 {}),
        Box::new(super::rsp::op_vmudm::VMUDMAll {}),
        Box::new(super::rsp::op_vmudm::VMUDM6 {}),
        Box::new(super::rsp::op_vmudn::VMUDNAll {}),
        Box::new(super::rsp::op_vmudn::VMUDNH2 {}),
        Box::new(super::rsp::op_vmudn::VMUDN7 {}),
        Box::new(super::rsp::op_vmulf::VMULFAll {}),
        Box::new(super::rsp::op_vmulf::VMULFAll1 {}),
        Box::new(super::rsp::op_vmulf::VMULFH0 {}),
        Box::new(super::rsp::op_vmulf::VMULFH1 {}),
        Box::new(super::sp_memory::SW {}),
        Box::new(super::sp_memory::SWOutOfBounds {}),
        Box::new(super::sp_memory::SH {}),
        Box::new(super::sp_memory::SB {}),
        Box::new(super::sp_memory::SD {}),
        Box::new(super::sp_memory::LB {}),
        Box::new(super::sp_memory::LH {}),
        Box::new(super::sp_memory::dma::SPDMA0_8_7 {}),
        Box::new(super::sp_memory::dma::SPDMA0_12_7D {}),
        Box::new(super::sp_memory::dma::SPDMA0_12_7I {}),
        Box::new(super::sp_memory::dma::SPDMA4_8_7 {}),
        Box::new(super::sp_memory::dma::SPDMA0_8_11D {}),
        Box::new(super::sp_memory::dma::SPDMA0_8_11I {}),
        Box::new(super::sp_memory::dma::SPDMAIntoDMEMWithOverflow {}),
        Box::new(super::sp_memory::dma::SPDMAIntoIMEMWithOverflow {}),
        Box::new(super::tlb::WiredRandom {}),
        Box::new(super::tlb::WiredOutOfBoundsRandom {}),
        Box::new(super::tlb::WriteRandomExpectIgnored {}),
        Box::new(super::tlb::IndexMasking {}),
        Box::new(super::tlb::EntryLo0Masking {}),
        Box::new(super::tlb::EntryLo0Masking64 {}),
        Box::new(super::tlb::EntryLo1Masking {}),
        Box::new(super::tlb::EntryLo1Masking64 {}),
        Box::new(super::tlb::EntryHiMasking {}),
        Box::new(super::tlb::PageMaskMasking {}),
        Box::new(super::tlb::ConfigReadWrite {}),
        Box::new(super::tlb::TLBWriteReadPageMask {}),
        Box::new(super::tlb::TLBWriteReadBackEntry {}),
        Box::new(super::tlb::TLBUseTestRead0 {}),
        Box::new(super::tlb::TLBUseTestRead1 {}),
        Box::new(super::tlb::TLBUseTestReadMatchViaASID {}),
        Box::new(super::tlb::exceptions::ReadMiss4k {}),
        Box::new(super::tlb::exceptions::ReadMiss16k {}),
        Box::new(super::tlb::exceptions::ReadMiss64k {}),
        Box::new(super::tlb::exceptions::ReadMiss256k {}),
        Box::new(super::tlb::exceptions::ReadMiss1M {}),
        Box::new(super::tlb::exceptions::ReadMiss4M {}),
        Box::new(super::tlb::exceptions::ReadMiss16M {}),
        Box::new(super::tlb::exceptions::StoreMiss4k {}),
        Box::new(super::tlb::exceptions::ReadNonValid4k {}),
        Box::new(super::tlb::exceptions::StoreNonValid4k {}),
        Box::new(super::tlb::exceptions::StoreNonDirty4k {}),
        Box::new(super::tlb::exceptions::StoreNonDirtyAndNonValid4k {}),
        Box::new(super::tlb::exceptions::LWTLBMissTest32 {}),
        Box::new(super::tlb::exceptions::LWTLBMissTest64 {}),
        Box::new(super::tlb::exceptions::LWAddressNotSignExtended64 {}),
        Box::new(super::tlb::exceptions::SWAddressNotSignExtended64 {}),
        Box::new(super::traps::TLT {}),
        Box::new(super::traps::TLTU {}),
        Box::new(super::traps::TGE {}),
        Box::new(super::traps::TGEU {}),
        Box::new(super::traps::TEQ {}),
        Box::new(super::traps::TNE {}),
        Box::new(super::traps::TEQI {}),
        Box::new(super::traps::TNEI {}),
        Box::new(super::traps::TGEI {}),
        Box::new(super::traps::TGEIU {}),
        Box::new(super::traps::TLTI {}),
        Box::new(super::traps::TLTIU {}),
        Box::new(super::traps::delay::TNEDelay1 {}),
        Box::new(super::traps::delay::TNEDelay2 {}),
    }
}

#[cfg(not(feature = "default_tests"))]
fn default_tests() -> Vec<Box<dyn Test>> {
    vec! {}
}

pub fn tests() -> Vec<Box<dyn Test>> {
    let mut result = default_tests();
    append_stress_tests(&mut result);
    result
}
