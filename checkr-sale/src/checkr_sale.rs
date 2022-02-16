#![no_std]

elrond_wasm::imports!();

const EGLD_DECIMALS_VALUE: u64 = 1_000_000_000_000_000_000;

/// A contract that allows anyone to by a CHECKR token in a public sale.
#[elrond_wasm::contract]
pub trait CHECKRSale {
    /// Necessary configuration when deploying:
    /// `max_amount` - max amount of EGLD that can be used to buy $CHECKR.  
    /// `min_amount` - min amount of EGLD that can be used to buy $CHECKR.  
    /// `initial_price` - price for $CHECKR token in EGLD (how much $CHECKR for 1 EGLD) 
    /// `token_id` - $CHECKR token ID.
    #[init]
    fn init(
        &self,
        max_amount: Self::BigUint,
        min_amount: Self::BigUint,
        initial_price: Self::BigUint,
        #[var_args] opt_token_id: OptionalArg<TokenIdentifier>,
    ) -> SCResult<()> {
        require!(max_amount > 0, "Max amount cannot be set to zero");
        require!(min_amount > 0, "Min amount cannot be set to zero");
        require!(initial_price > 0, "Initial price cannot be set to zero");

        let token_id = opt_token_id
            .into_option()
            .unwrap_or_else(TokenIdentifier::egld);
        require!(
            token_id.is_egld() || token_id.is_valid_esdt_identifier(),
            "Invalid token provided"
        );
        let caller: Address = self.blockchain().get_caller();
        self.set_owner(&caller);

        self.price().set(&initial_price);

        self.max_amount().set(&max_amount);

        self.min_amount().set(&min_amount);

        self.sale_token_id().set(&token_id);

        Ok(())
    }

    // endpoints

    /// User sends some tokens to the contract in order to exchange it for $CHECKR
    /// Optional `_data` argument is ignored.
    #[payable("EGLD")]
    #[endpoint]
    fn purchase_checker(
        &self,
        #[payment_amount] payment_amount: Self::BigUint,
    ) -> SCResult<()> {
        require!(
            payment_amount <= self.max_amount().get(),
            "The payment is too high"
        );
        require!(
            payment_amount >= self.min_amount().get(),
            "The payment is too low"
        );

        let balance = self.blockchain().get_sc_balance(&self.sale_token_id().get(), 0);
        require!(
            balance > 0,
            "No more token to sale."
        );
        let current_price = self.price().get();
        let one_egld = Self::BigUint::from(EGLD_DECIMALS_VALUE);
        let result_edst_token_amount = ( &current_price * &payment_amount ) / one_egld;
        require!(
            balance > result_edst_token_amount,
            "Not enough tokens for sale."
        );

        //send the ESDT token amount to the user
        let caller = self.blockchain().get_caller();
        let token_id = self.sale_token_id().get();
        self.send()
        .direct(&caller, &token_id, 0, &result_edst_token_amount, b"CHECKR sale successful :).");

        Ok(())
    }

    /// Optional `_data` argument is ignored.
    #[payable("*")]
    #[endpoint]
    fn deposit(
        &self,
        #[payment_amount] _payment_amount: Self::BigUint,
    ) -> SCResult<()> {

        Ok(())
    }

    /// Claims the funds in the SmartContract.
    #[endpoint]
    #[only_owner]
    fn claim_egld(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();

        let amount = self.blockchain().get_balance(&self.blockchain().get_sc_address());
    
        self.send().direct_egld(&caller, &amount, b"Withdrawn Successful.");

        Ok(())
    }

    #[endpoint]
    #[only_owner]
    fn claim_checkr(&self) -> SCResult<()> {

        let balance = self.blockchain().get_sc_balance(&self.sale_token_id().get(), 0);
   
        let caller = self.blockchain().get_caller();
        let token_id = self.sale_token_id().get();
        self.send()
        .direct(&caller, &token_id, 0, &balance, b"CHECKR withdraw successful :).");

        Ok(())
    }

    // storage
    #[storage_set("owner")]
    fn set_owner(&self, address: &Address);

    #[view]
    #[storage_get("owner")]
    fn get_owner(&self) -> Address;

    #[view(getSaleToken)]
    #[storage_mapper("saleTokenId")]
    fn sale_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

    #[view(getMaxAmount)]
    #[storage_mapper("maxAmount")]
    fn max_amount(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getMinAmount)]
    #[storage_mapper("minAmount")]
    fn min_amount(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[view(getPrice)]
    #[storage_mapper("price")]
    fn price(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

}
