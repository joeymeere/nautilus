pub mod index;

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone)]
pub struct Table<'a, T: crate::objects::properties::table::NautilusData> {
    pub program_id: &'a solana_program::pubkey::Pubkey,
    pub account_info: solana_program::account_info::AccountInfo<'a>,
    pub data: T,
}

impl<'a, T: crate::objects::properties::table::NautilusData>
    solana_program::account_info::IntoAccountInfo<'a> for Table<'a, T>
{
    fn into_account_info(self) -> solana_program::account_info::AccountInfo<'a> {
        self.account_info
    }
}

impl<'a, T: crate::objects::properties::table::NautilusData>
    crate::objects::properties::NautilusAccountInfo<'a> for Table<'a, T>
{
    fn key(&self) -> &'a solana_program::pubkey::Pubkey {
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

    fn mut_lamports(
        &self,
    ) -> Result<std::cell::RefMut<'_, &'a mut u64>, solana_program::program_error::ProgramError>
    {
        self.account_info.try_borrow_mut_lamports()
    }

    fn owner(&self) -> &'a solana_program::pubkey::Pubkey {
        self.account_info.owner
    }

    fn span(&self) -> usize {
        self.account_info.data_len()
    }
}

impl<'a, T: crate::objects::properties::table::NautilusData>
    crate::objects::properties::table::NautilusTable<'a> for Table<'a, T>
{
    fn primary_key(&self) -> &'a [u8] {
        self.data.primary_key()
    }

    fn seeds(&self) -> [&'a [u8]; 2] {
        self.data.seeds()
    }

    fn pda(&self) -> (solana_program::pubkey::Pubkey, u8) {
        self.data.pda(self.program_id)
    }

    fn check_authorities(
        &self,
        accounts: Vec<solana_program::account_info::AccountInfo>,
    ) -> Result<(), solana_program::program_error::ProgramError> {
        self.data.check_authorities(accounts)
    }

    fn count_authorities(&self) -> u8 {
        self.data.count_authorities()
    }
}

impl<'a, T: crate::objects::properties::table::NautilusData + 'a>
    crate::objects::properties::NautilusTransferLamports<'a> for Table<'a, T>
{
    fn transfer_lamports<U: crate::objects::properties::NautilusAccountInfo<'a>>(
        self,
        to: U,
        amount: u64,
    ) -> solana_program::entrypoint::ProgramResult {
        let from = self.account_info;
        **from.try_borrow_mut_lamports()? -= amount;
        **to.mut_lamports()? += amount;
        Ok(())
    }
}

impl<'a, T: crate::objects::properties::table::NautilusData>
    crate::objects::properties::create::NautilusCreate<'a>
    for crate::objects::properties::create::Create<'a, Table<'a, T>>
{
    fn create(&self) -> solana_program::entrypoint::ProgramResult {
        use crate::objects::properties::{table::NautilusTable, NautilusAccountInfo};

        let payer = self.fee_payer.clone();
        let system_program = self.system_program.clone();
        let (_, bump) = self.self_account.pda();
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                payer.key,
                self.self_account.key(),
                self.self_account.required_rent()?,
                self.self_account.size(),
                system_program.key,
            ),
            &[
                payer,
                self.self_account.account_info.clone(),
                system_program,
            ],
            &[&self.self_account.data.seeds(), &[&[bump]]],
        )
    }

    fn create_with_payer<U: crate::objects::properties::NautilusAccountInfo<'a>>(
        &self,
        payer: U,
    ) -> solana_program::entrypoint::ProgramResult {
        use crate::objects::properties::{table::NautilusTable, NautilusAccountInfo};

        let system_program = self.system_program.clone();
        let (_, bump) = self.self_account.pda();
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                payer.key(),
                self.self_account.key(),
                self.self_account.required_rent()?,
                self.self_account.size(),
                system_program.key,
            ),
            &[
                payer.into(),
                self.self_account.account_info.clone(),
                system_program,
            ],
            &[&self.self_account.data.seeds(), &[&[bump]]],
        )
    }
}