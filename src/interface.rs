use soroban_sdk::{Address, Env, Vec};


pub trait NFTokenTrait {
    // --------------------------------------------------------------------------------
    // Admin interface
    // --------------------------------------------------------------------------------

    /// Returns the current administrator
    fn admin(env: Env) -> Address;

    /// If "admin" is the administrator, set the administrator to "new_admin".
    /// Emit event with topics = ["set_admin", admin: Address], data = [new_admin: Address]
    fn set_admin(env: Env, new_admin: Address);

    // --------------------------------------------------------------------------------
    // Token interface
    // --------------------------------------------------------------------------------

    /// Allows "operator" to manage token "id" if "owner" is the current owner of token "id".
    /// Emit event with topics = ["appr", operator: Address], data = [id: i128]
    fn appr(env: Env, owner: Address, operator: Address, id: i128);

    /// If "approved", allows "operator" to manage all tokens of "owner"
    /// Emit event with topics = ["appr_all", operator: Address], data = [owner: Address]
    fn appr_all(env: Env, owner: Address, operator: Address, approved: bool);

    /// Returns the identifier approved for token "id".
    fn get_appr(env: Env, id: i128) -> Address;

    /// If "operator" is allowed to manage assets of "owner", return true.
    fn is_appr(env: Env, owner: Address, operator: Address) -> bool;

    /// Get the owner of "id" token.
    fn owner(env: Env, id: i128) -> Address;

    /// Get all NFT ids owned by address
    fn get_all_owned(env: Env, address: Address) -> Vec<i128>;

    /// Transfer token "id" from "from" to "to.
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer(env: Env, from: Address, to: Address, id: i128);

    /// Transfer token "id" from "from" to "to", consuming the allowance of "spender".
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [id: i128]
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, id: i128);

    /// Mint an NFT
    fn mint_new(env: Env, to: Address);

    /// If "admin" is the administrator or the token owner, burn token "id" from "from".
    /// Emit event with topics = ["burn", from: Address], data = [id: i128]
    fn burn(env: Env, id: i128);


    // --------------------------------------------------------------------------------
    // Implementation Interface
    // --------------------------------------------------------------------------------

    /// Initialize the contract.
    /// "admin" is the contract administrator.
    fn initialize(e: Env, admin: Address);
}
