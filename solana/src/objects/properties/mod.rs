pub mod create;
pub mod table;
pub mod tokens;

pub trait NautilusAccountInfo<'a>:
    solana_program::account_info::IntoAccountInfo<'a> + Sized + Clone
{
    fn key(&self) -> &'a solana_program::pubkey::Pubkey;
    fn is_signer(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn lamports(&self) -> u64;
    fn mut_lamports(
        &self,
    ) -> Result<std::cell::RefMut<'_, &'a mut u64>, solana_program::program_error::ProgramError>;
    fn owner(&self) -> &'a solana_program::pubkey::Pubkey;
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
    fn transfer_lamports<T: NautilusAccountInfo<'a> + 'a>(
        self,
        to: T,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult;
}