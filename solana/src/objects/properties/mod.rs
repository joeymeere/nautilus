use solana_program::{
    account_info::IntoAccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod create;
pub mod signer;
pub mod table;

pub trait NautilusAccountInfo<'a>: IntoAccountInfo<'a> + Clone + 'a {
    fn key(&self) -> &'a Pubkey;
    fn is_signer(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn lamports(&self) -> u64;
    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError>;
    fn owner(&self) -> &'a Pubkey;
    fn span(&self) -> usize;
    fn size(&self) -> u64 {
        self.span().try_into().unwrap()
    }
    fn required_rent(&self) -> Result<u64, solana_program::program_error::ProgramError> {
        use solana_program::sysvar::Sysvar;
        Ok((solana_program::sysvar::rent::Rent::get().unwrap()).minimum_balance(self.span()))
    }
}

pub trait NautilusTransferLamports<'a>: NautilusAccountInfo<'a> + 'a {
    fn transfer_lamports(self, to: impl NautilusAccountInfo<'a>, amount: u64) -> ProgramResult;
}
