use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    cpi, error::NautilusError, Create, NautilusAccountInfo, NautilusCreateRecord, NautilusData,
    NautilusIndex, NautilusRecord, NautilusSigner, NautilusTransferLamports, Signer, Wallet,
};

pub mod index;

#[derive(Clone)]
pub struct Record<'a, T>
where
    T: NautilusData,
{
    pub program_id: &'a Pubkey,
    pub account_info: Box<AccountInfo<'a>>,
    pub index: NautilusIndex<'a>,
    pub data: T,
}

impl<'a, T> Record<'a, T>
where
    T: NautilusData,
{
    pub fn new(
        program_id: &'a Pubkey,
        account_info: Box<AccountInfo<'a>>,
        index: NautilusIndex<'a>,
    ) -> Self {
        Self {
            program_id,
            index,
            account_info,
            data: T::default(),
        }
    }

    pub fn load(
        program_id: &'a Pubkey,
        account_info: Box<AccountInfo<'a>>,
        index: NautilusIndex<'a>,
    ) -> Result<Self, ProgramError> {
        let data = match T::try_from_slice(match &account_info.try_borrow_data() {
            Ok(acct_data) => acct_data,
            Err(_) => {
                return Err(NautilusError::LoadDataFailed(
                    T::TABLE_NAME.to_string(),
                    account_info.key.to_string(),
                )
                .into())
            }
        }) {
            Ok(state_data) => state_data,
            Err(_) => {
                return Err(NautilusError::DeserializeDataFailed(
                    T::TABLE_NAME.to_string(),
                    account_info.key.to_string(),
                )
                .into())
            }
        };
        Ok(Self {
            program_id,
            index,
            account_info,
            data,
        })
    }
}

impl<'a, T> NautilusAccountInfo<'a> for Record<'a, T>
where
    T: NautilusData,
{
    fn account_info(&self) -> Box<AccountInfo<'a>> {
        self.account_info.clone()
    }

    fn key(&self) -> &'a Pubkey {
        self.account_info.key
    }

    fn is_signer(&self) -> bool {
        self.account_info.is_signer
    }

    fn is_writable(&self) -> bool {
        self.account_info.is_writable
    }

    fn lamports(&self) -> u64 {
        self.account_info.lamports()
    }

    fn mut_lamports(&self) -> Result<std::cell::RefMut<'_, &'a mut u64>, ProgramError> {
        self.account_info.try_borrow_mut_lamports()
    }

    fn owner(&self) -> &'a Pubkey {
        self.account_info.owner
    }

    fn span(&self) -> usize {
        self.account_info.data_len()
    }
}

impl<'a, T> NautilusRecord<'a> for Record<'a, T>
where
    T: NautilusData,
{
    fn primary_key(&self) -> &'a [u8] {
        self.data.primary_key()
    }

    fn seeds(&self) -> [&'a [u8]; 2] {
        self.data.seeds()
    }

    fn pda(&self) -> (Pubkey, u8) {
        self.data.pda(self.program_id)
    }

    fn check_authorities(&self, accounts: Vec<AccountInfo>) -> Result<(), ProgramError> {
        self.data.check_authorities(accounts)
    }

    fn count_authorities(&self) -> u8 {
        self.data.count_authorities()
    }
}

impl<'a, T> NautilusTransferLamports<'a> for Record<'a, T>
where
    T: NautilusData,
{
    fn transfer_lamports(
        self,
        to: impl NautilusAccountInfo<'a>,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult {
        let from = self.account_info;
        **from.try_borrow_mut_lamports()? -= amount;
        **to.mut_lamports()? += amount;
        Ok(())
    }
}

impl<'a, T> NautilusCreateRecord<'a, T> for Create<'a, Record<'a, T>>
where
    T: NautilusData,
{
    fn create_record(&mut self, data: T) -> ProgramResult {
        let payer = Signer::new(Wallet {
            account_info: self.fee_payer.to_owned(),
            system_program: self.system_program.to_owned(),
        });
        let data_pointer = Box::new(data);
        cpi::create::create_record(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.system_program.to_owned(),
            data_pointer.clone(),
        )?;
        self.self_account.data = *data_pointer;
        Ok(())
    }

    fn create_record_with_payer(
        &mut self,
        data: T,
        payer: impl NautilusSigner<'a>,
    ) -> ProgramResult {
        let data_pointer = Box::new(data);
        cpi::create::create_record(
            self.self_account.clone(),
            self.self_account.program_id,
            payer,
            self.system_program.to_owned(),
            data_pointer.clone(),
        )?;
        self.self_account.data = *data_pointer;
        Ok(())
    }
}