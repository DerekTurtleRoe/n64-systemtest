use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::tests::Test;

pub fn tests() -> Vec<Box<dyn Test>> {
    vec! {
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
        Box::new(super::sp_memory::SW {}),
        Box::new(super::sp_memory::SWOutOfBounds {}),
        Box::new(super::sp_memory::SH {}),
        Box::new(super::sp_memory::SB {}),
        Box::new(super::sp_memory::SD {}),
        Box::new(super::sp_memory::LB {}),
        Box::new(super::sp_memory::LH {}),
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
