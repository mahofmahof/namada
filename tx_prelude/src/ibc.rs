//! IBC lower-level functions for transactions.

use std::cell::RefCell;
use std::rc::Rc;

pub use namada_core::ledger::ibc::{
    Error, IbcActions, IbcCommonContext, IbcStorageContext, ProofSpec,
    TransferModule,
};
use namada_core::ledger::storage_api::{StorageRead, StorageWrite};
use namada_core::ledger::tx_env::TxEnv;
use namada_core::types::address::{Address, InternalAddress};
pub use namada_core::types::ibc::IbcEvent;
use namada_core::types::storage::{BlockHeight, Header, Key};
use namada_core::types::token::DenominatedAmount;

use crate::token::{burn, mint, transfer};
use crate::{Ctx, KeyValIterator};

/// IBC actions to handle an IBC message
pub fn ibc_actions(ctx: &mut Ctx) -> IbcActions<Ctx> {
    let ctx = Rc::new(RefCell::new(ctx.clone()));
    let mut actions = IbcActions::new(ctx.clone());
    let module = TransferModule::new(ctx);
    actions.add_transfer_route(module.module_id(), module);
    actions
}

impl IbcStorageContext for Ctx {
    type Error = crate::Error;
    type PrefixIter<'iter> = KeyValIterator<(String, Vec<u8>)>;

    fn read(
        &self,
        key: &Key,
    ) -> std::result::Result<Option<Vec<u8>>, Self::Error> {
        self.read_bytes(key)
    }

    fn has_key(&self, key: &Key) -> Result<bool, Self::Error> {
        <Ctx as StorageRead>::has_key(self, key)
    }

    fn write(
        &mut self,
        key: &Key,
        data: Vec<u8>,
    ) -> std::result::Result<(), Self::Error> {
        self.write_bytes(key, data)?;
        Ok(())
    }

    fn iter_prefix<'iter>(
        &'iter self,
        prefix: &Key,
    ) -> Result<Self::PrefixIter<'iter>, Self::Error> {
        StorageRead::iter_prefix(self, prefix)
    }

    fn iter_next<'iter>(
        &'iter self,
        iter: &mut Self::PrefixIter<'iter>,
    ) -> Result<Option<(String, Vec<u8>)>, Self::Error> {
        StorageRead::iter_next(self, iter)
    }

    fn delete(&mut self, key: &Key) -> std::result::Result<(), Self::Error> {
        StorageWrite::delete(self, key)
    }

    fn emit_ibc_event(
        &mut self,
        event: IbcEvent,
    ) -> std::result::Result<(), Self::Error> {
        <Ctx as TxEnv>::emit_ibc_event(self, &event)
    }

    fn get_ibc_event(
        &self,
        event_type: impl AsRef<str>,
    ) -> Result<Option<IbcEvent>, Self::Error> {
        <Ctx as TxEnv>::get_ibc_event(self, &event_type)
    }

    fn transfer_token(
        &mut self,
        src: &Address,
        dest: &Address,
        token: &Address,
        amount: DenominatedAmount,
    ) -> std::result::Result<(), Self::Error> {
        transfer(self, src, dest, token, amount, &None, &None, &None)
    }

    fn mint_token(
        &mut self,
        target: &Address,
        token: &Address,
        amount: DenominatedAmount,
    ) -> Result<(), Self::Error> {
        mint(
            self,
            &Address::Internal(InternalAddress::Ibc),
            target,
            token,
            amount.amount,
        )
    }

    fn burn_token(
        &mut self,
        target: &Address,
        token: &Address,
        amount: DenominatedAmount,
    ) -> Result<(), Self::Error> {
        burn(self, target, token, amount.amount)
    }

    fn get_height(&self) -> std::result::Result<BlockHeight, Self::Error> {
        self.get_block_height()
    }

    fn get_header(
        &self,
        height: BlockHeight,
    ) -> std::result::Result<Option<Header>, Self::Error> {
        self.get_block_header(height)
    }

    fn log_string(&self, message: String) {
        super::log_string(message);
    }
}

impl IbcCommonContext for Ctx {}
