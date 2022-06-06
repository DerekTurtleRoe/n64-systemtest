use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::tests::Test;

fn append_stress_tests(_target: &mut Vec<Box<dyn Test>>) {
    #[cfg(feature = "vmulf_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMULF {}));
    #[cfg(feature = "vmulu_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMULU {}));
    #[cfg(feature = "vmulq_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMULQ {}));
    #[cfg(feature = "vmudl_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMUDL {}));
    #[cfg(feature = "vmudh_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMUDH {}));
    #[cfg(feature = "vmudm_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMUDM {}));
    #[cfg(feature = "vmudn_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMUDN {}));
    #[cfg(feature = "vmacf_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMACF {}));
    #[cfg(feature = "vmacu_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMACU {}));
    #[cfg(feature = "vmadl_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMADL {}));
    #[cfg(feature = "vmadh_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMADH {}));
    #[cfg(feature = "vmadm_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMADM {}));
    #[cfg(feature = "vmadn_stress_test")]
    _target.push(Box::new(super::rsp::stresstests::VMADN {}));
    #[cfg(feature = "vrcp32_stress_test")]
    _target.push(Box::new(super::rsp::stresstests_div::VRCP32 {}));
    #[cfg(feature = "vrsq32_stress_test")]
    _target.push(Box::new(super::rsp::stresstests_div::VRSQ32 {}));
    #[cfg(feature = "rcp_rsq_dump")]
    _target.push(Box::new(super::rsp::op_vmov_vrcp::GenerateDump {}));
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
        Box::new(super::arithmetic::shifts::SRA {}),
        Box::new(super::arithmetic::shifts::SRL {}),
        Box::new(super::arithmetic::shifts::SLL {}),
        Box::new(super::arithmetic::shifts::ShiftsIntoR0 {}),
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
        Box::new(super::cart_memory::write::WriteAndCheckPIFlag {}),
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
        Box::new(super::rsp::op_break::BREAKWithinDelay {}),
        Box::new(super::rsp::op_break::BREAKWithinDelay2 {}),
        // This should be RSP test #3
        Box::new(super::rsp::wrap_around::WrapAround {}),
        // Non-vector instructions
        Box::new(super::rsp::op_add_addu::ADD {}),
        Box::new(super::rsp::op_add_addu::ADDU {}),
        Box::new(super::rsp::op_sub_subu::SUB {}),
        Box::new(super::rsp::op_sub_subu::SUBU {}),
        Box::new(super::rsp::op_addi::ADDI {}),
        Box::new(super::rsp::op_addiu::ADDIU {}),
        Box::new(super::rsp::op_andi::ANDI {}),
        Box::new(super::rsp::op_ori::ORI {}),
        Box::new(super::rsp::op_xori::XORI {}),
        Box::new(super::rsp::op_and::AND {}),
        Box::new(super::rsp::op_or::OR {}),
        Box::new(super::rsp::op_nor::NOR {}),
        Box::new(super::rsp::op_xor::XOR {}),
        Box::new(super::rsp::op_slt::SLT {}),
        Box::new(super::rsp::op_sltu::SLTU {}),
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
        Box::new(super::rsp::op_jr_jalr::JR {}),
        Box::new(super::rsp::op_jr_jalr::JRWithRegisterChangeInDelaySlot {}),
        Box::new(super::rsp::op_jr_jalr::JALR {}),
        Box::new(super::rsp::op_jr_jalr::JALRWithRegisterChangeInDelaySlot {}),
        Box::new(super::rsp::op_jr_jalr::JALRWithReturnAddressChangeInDelaySlot {}),
        Box::new(super::rsp::op_jr_jalr::JALRWithReturnAddressEqualToTargetAddress {}),

        Box::new(super::rsp::op_branches::BEQ {}),
        Box::new(super::rsp::op_branches::BNE {}),
        Box::new(super::rsp::op_branches::BLEZ {}),
        Box::new(super::rsp::op_branches::BGTZ {}),
        Box::new(super::rsp::op_branches::BLTZ {}),
        Box::new(super::rsp::op_branches::BGEZ {}),
        Box::new(super::rsp::op_branches::BLTZAL {}),
        Box::new(super::rsp::op_branches::BLTZALTestRA {}),
        Box::new(super::rsp::op_branches::BGEZAL {}),
        Box::new(super::rsp::op_branches::BGEZALTestRA {}),

        Box::new(super::rsp::op_shifts::SLL {}),
        Box::new(super::rsp::op_shifts::SRL {}),
        Box::new(super::rsp::op_shifts::SRA {}),
        Box::new(super::rsp::op_shifts::SLLV {}),
        Box::new(super::rsp::op_shifts::SLLVWithShiftAmountOverwrite {}),
        Box::new(super::rsp::op_shifts::SRLV {}),
        Box::new(super::rsp::op_shifts::SRLVWithShiftAmountOverwrite {}),
        Box::new(super::rsp::op_shifts::SRAV {}),
        Box::new(super::rsp::op_shifts::SRAVWithShiftAmountOverwrite {}),

        // RSP Vector Store instructions
        Box::new(super::rsp::op_vector_stores::SBV {}),
        Box::new(super::rsp::op_vector_stores::SSV {}),
        Box::new(super::rsp::op_vector_stores::SLV {}),
        Box::new(super::rsp::op_vector_stores::SDV {}),
        Box::new(super::rsp::op_vector_stores::SQV {}),
        Box::new(super::rsp::op_vector_stores::SRV {}),
        Box::new(super::rsp::op_vector_stores::SPV {}),
        Box::new(super::rsp::op_vector_stores::SUV {}),
        Box::new(super::rsp::op_vector_stores::SHV {}),
        Box::new(super::rsp::op_vector_stores::SFV {}),
        Box::new(super::rsp::op_vector_stores::SWV {}),
        Box::new(super::rsp::op_vector_stores::STV {}),

        // RSP Vector Load instructions
        Box::new(super::rsp::op_cfc2_ctc2::CTC2CFC2 {}),
        Box::new(super::rsp::op_cfc2_ctc2::CFC2WeirdIndexes {}),
        Box::new(super::rsp::op_cfc2_ctc2::CTC2WeirdIndexes {}),
        Box::new(super::rsp::op_mfc2_mtc2::MTC2 {}),
        Box::new(super::rsp::op_mfc2_mtc2::MFC2 {}),
        Box::new(super::rsp::op_vector_loads::LBV {}),
        Box::new(super::rsp::op_vector_loads::LSV {}),
        Box::new(super::rsp::op_vector_loads::LSVOverflow {}),
        Box::new(super::rsp::op_vector_loads::LLV {}),
        Box::new(super::rsp::op_vector_loads::LLVOverflow {}),
        Box::new(super::rsp::op_vector_loads::LDV {}),
        Box::new(super::rsp::op_vector_loads::LDVOverflow {}),
        Box::new(super::rsp::op_vector_loads::LQV {}),
        Box::new(super::rsp::op_vector_loads::LQVEndOfDMEM {}),
        Box::new(super::rsp::op_vector_loads::LRV {}),
        Box::new(super::rsp::op_vector_loads::LRVStartOfDMEM {}),
        Box::new(super::rsp::op_vector_loads::LPV {}),
        Box::new(super::rsp::op_vector_loads::LPVEndOfDMEM {}),
        Box::new(super::rsp::op_vector_loads::LUV {}),
        Box::new(super::rsp::op_vector_loads::LUVEndOfDMEM {}),
        Box::new(super::rsp::op_vector_loads::LHV {}),
        Box::new(super::rsp::op_vector_loads::LHVEndOfDMEM {}),
        Box::new(super::rsp::op_vector_loads::LFV {}),
        Box::new(super::rsp::op_vector_loads::LWV {}),
        Box::new(super::rsp::op_vector_loads::LTV {}),

        // Other vector instructions
        Box::new(super::rsp::op_vector_arithmetic::VADD {}),
        Box::new(super::rsp::op_vector_arithmetic::VSUB {}),
        Box::new(super::rsp::op_vector_arithmetic::VSUT {}),
        Box::new(super::rsp::op_vector_arithmetic::VABS {}),
        Box::new(super::rsp::op_vector_arithmetic::VADDC {}),
        Box::new(super::rsp::op_vector_arithmetic::VSUBC {}),
        Box::new(super::rsp::op_vector_arithmetic::VADDB {}),
        Box::new(super::rsp::op_vector_arithmetic::VSUBB {}),
        Box::new(super::rsp::op_vector_arithmetic::VACCB {}),
        Box::new(super::rsp::op_vector_arithmetic::VSUCB {}),
        Box::new(super::rsp::op_vector_arithmetic::VSAD {}),
        Box::new(super::rsp::op_vector_arithmetic::VSAC {}),
        Box::new(super::rsp::op_vector_arithmetic::VSUM {}),
        Box::new(super::rsp::op_vector_arithmetic::VSUMNoNops {}),
        Box::new(super::rsp::op_vector_arithmetic::VLT {}),
        Box::new(super::rsp::op_vector_arithmetic::VEQ {}),
        Box::new(super::rsp::op_vector_arithmetic::VNE {}),
        Box::new(super::rsp::op_vector_arithmetic::VGE {}),
        Box::new(super::rsp::op_vector_arithmetic::VMRG {}),
        Box::new(super::rsp::op_vector_arithmetic::VAND {}),
        Box::new(super::rsp::op_vector_arithmetic::VNAND {}),
        Box::new(super::rsp::op_vector_arithmetic::VOR {}),
        Box::new(super::rsp::op_vector_arithmetic::VNOR {}),
        Box::new(super::rsp::op_vector_arithmetic::VXOR {}),
        Box::new(super::rsp::op_vector_arithmetic::VNXOR {}),
        Box::new(super::rsp::op_vector_arithmetic::VNOP {}),
        Box::new(super::rsp::op_vector_arithmetic::VEXTT {}),
        Box::new(super::rsp::op_vector_arithmetic::VEXTQ {}),
        Box::new(super::rsp::op_vector_arithmetic::VEXTN {}),
        Box::new(super::rsp::op_vector_arithmetic::VINST {}),
        Box::new(super::rsp::op_vector_arithmetic::VINSQ {}),
        Box::new(super::rsp::op_vector_arithmetic::VINSN {}),
        Box::new(super::rsp::op_vector_arithmetic::VNULL {}),

        Box::new(super::rsp::op_vector_arithmetic::V30 {}),
        Box::new(super::rsp::op_vector_arithmetic::V31 {}),
        Box::new(super::rsp::op_vector_arithmetic::V46 {}),
        Box::new(super::rsp::op_vector_arithmetic::V47 {}),
        Box::new(super::rsp::op_vector_arithmetic::V59 {}),

        Box::new(super::rsp::op_vector_arithmetic::VCL {}),
        Box::new(super::rsp::op_vector_arithmetic::VCH {}),
        Box::new(super::rsp::op_vector_arithmetic::VCR {}),

        Box::new(super::rsp::op_vsar::VSAR {}),
        Box::new(super::rsp::op_vmacf::VMACFAll {}),
        Box::new(super::rsp::op_vmacf::VMACFH0 {}),
        Box::new(super::rsp::op_vmacf::VMACF5 {}),
        Box::new(super::rsp::op_vmacf::VMACFAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmacq::VMACQ {}),
        Box::new(super::rsp::op_vmacu::VMACUAll {}),
        Box::new(super::rsp::op_vmacu::VMACUH0 {}),
        Box::new(super::rsp::op_vmacu::VMACU0 {}),
        Box::new(super::rsp::op_vmacu::VMACUAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmadl::VMADLAll {}),
        Box::new(super::rsp::op_vmadl::VMADL4 {}),
        Box::new(super::rsp::op_vmadl::VMADLAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmadm::VMADMAll {}),
        Box::new(super::rsp::op_vmadm::VMADM4 {}),
        Box::new(super::rsp::op_vmadm::VMADMAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmadn::VMADNAll {}),
        Box::new(super::rsp::op_vmadn::VMADNH1 {}),
        Box::new(super::rsp::op_vmadn::VMADNH3 {}),
        Box::new(super::rsp::op_vmadn::VMADN6 {}),
        Box::new(super::rsp::op_vmadn::VMADNAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmadh::VMADHAll {}),
        Box::new(super::rsp::op_vmadh::VMADHQ1 {}),
        Box::new(super::rsp::op_vmadh::VMADHH1 {}),
        Box::new(super::rsp::op_vmadh::VMADH7 {}),
        Box::new(super::rsp::op_vmadh::VMADHAccumulatorOverflowed {}),
        Box::new(super::rsp::op_vmudh::VMUDHAll {}),
        Box::new(super::rsp::op_vmudh::VMUDHH0 {}),
        Box::new(super::rsp::op_vmudh::VMUDHQ1 {}),
        Box::new(super::rsp::op_vmudh::VMUDH7 {}),
        Box::new(super::rsp::op_vmudl::VMUDLAll {}),
        Box::new(super::rsp::op_vmudl::VMUDLH1 {}),
        Box::new(super::rsp::op_vmudl::VMUDL5 {}),
        Box::new(super::rsp::op_vmudm::VMUDMAll {}),
        Box::new(super::rsp::op_vmudm::VMUDMH1 {}),
        Box::new(super::rsp::op_vmudm::VMUDM7 {}),
        Box::new(super::rsp::op_vmudn::VMUDNAll {}),
        Box::new(super::rsp::op_vmudn::VMUDNH2 {}),
        Box::new(super::rsp::op_vmudn::VMUDN7 {}),
        Box::new(super::rsp::op_vmulf::VMULFAll {}),
        Box::new(super::rsp::op_vmulf::VMULFAll1 {}),
        Box::new(super::rsp::op_vmulf::VMULFH0 {}),
        Box::new(super::rsp::op_vmulf::VMULFH1 {}),
        Box::new(super::rsp::op_vmulu::VMULUAll {}),
        Box::new(super::rsp::op_vmulu::VMULUH1 {}),
        Box::new(super::rsp::op_vmulq::VMULQAll {}),
        Box::new(super::rsp::op_vmulq::VMULQH1 {}),

        Box::new(super::rsp::op_vmov_vrcp::VMOV {}),
        Box::new(super::rsp::op_vmov_vrcp::RCPTable {}),
        Box::new(super::rsp::op_vmov_vrcp::RSQTable {}),
        Box::new(super::rsp::op_vmov_vrcp::VRCPRegisterCombinations {}),
        Box::new(super::rsp::op_vmov_vrcp::VRCPHRegisterCombinations {}),
        Box::new(super::rsp::op_vmov_vrcp::VRCPLRegisterCombinations {}),
        Box::new(super::rsp::op_vmov_vrcp::VRSQRegisterCombinations {}),
        Box::new(super::rsp::op_vmov_vrcp::VRSQLRegisterCombinations {}),
        Box::new(super::rsp::op_vmov_vrcp::VRCPValues {}),
        Box::new(super::rsp::op_vmov_vrcp::VRSQValues {}),
        Box::new(super::rsp::op_vmov_vrcp::VRCP32Bit {}),
        Box::new(super::rsp::op_vmov_vrcp::VRSQ32Bit {}),
        Box::new(super::rsp::op_vmov_vrcp::HighUsesOutputVRCPTest {}),
        Box::new(super::rsp::op_vmov_vrcp::HighUsesOutputVRSQLTest {}),
        Box::new(super::rsp::op_vmov_vrcp::HighUsesOutputVRSQTest {}),
        Box::new(super::rsp::op_vmov_vrcp::HighUsesOutputVRSQLTest {}),
        Box::new(super::rsp::op_vmov_vrcp::VRCPHSetsInputForVRCPL {}),
        Box::new(super::rsp::op_vmov_vrcp::VRCPLHiddenRegisterFlagExists {}),

        Box::new(super::rsp::op_vrndp::VRNDPWithEvenVS {}),
        Box::new(super::rsp::op_vrndp::VRNDPWithOddVS {}),
        Box::new(super::rsp::op_vrndp::VRNDPOverwriteItselfWithElement {}),
        Box::new(super::rsp::op_vrndp::VRNDPAccumulatorOverflowed {}),

        Box::new(super::rsp::op_vrndn::VRNDNWithEvenVS {}),
        Box::new(super::rsp::op_vrndn::VRNDNWithOddVS {}),
        Box::new(super::rsp::op_vrndn::VRNDNOverwriteItselfWithElement {}),
        Box::new(super::rsp::op_vrndn::VRNDNAccumulatorOverflowed {}),

        Box::new(super::rsp::registers::SetClearInterrupt {}),
        Box::new(super::rsp::registers::SetClearHalt {}),
        Box::new(super::rsp::registers::SetClearSignal {}),
        Box::new(super::rsp::registers::SetClearInterruptOnBreak {}),
        Box::new(super::rsp::registers::SPRegisterReadAccessOnRSP {}),
        Box::new(super::rsp::registers::SemaphoreRegisterCPUOnly {}),
        Box::new(super::rsp::registers::SemaphoreRegisterRSPOnly {}),
        Box::new(super::rsp::registers::SemaphoreRegisterMixed {}),
        Box::new(super::rsp::registers::RSPHaltItselfWithoutBreak {}),

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
        Box::new(super::sp_memory::dma::SPDMAIntoDMEMUntilEnd {}),
        Box::new(super::sp_memory::dma::SPDMAIntoDMEMWithOverflow {}),
        Box::new(super::sp_memory::dma::SPDMAIntoIMEMUntilEnd {}),
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

/// Returns a list of tests to be performed.
pub fn tests() -> Vec<Box<dyn Test>> {
    let mut result = default_tests();
    append_stress_tests(&mut result);
    result
}
