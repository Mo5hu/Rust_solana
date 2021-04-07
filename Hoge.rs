pub enum TokenInstruction {
    InitializeMint {
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: COption<Pubkey>,
    },
    InitializeAccount,
    InitializeMultisig {
        m: u8,
    },
    Transfer {
        amount: u64,
    },
    Approve {
        amount: u64,
    },
    Revoke,
    SetAuthority {
        authority_type: AuthorityType,
        new_authority: COption<Pubkey>,
    },
    MintTo {
        amount: u64,
    },
    Burn {
        amount: u64,
    },
    CloseAccount,
    FreezeAccount,
    ThawAccount,
    Transfer2 {
        amount: u64,
        decimals: u8,
    },
    Approve2 {
        amount: u64,
        decimals: u8,
    },
    MintTo2 {
        amount: u64,
        decimals: u8,
    },
    Burn2 {
        amount: u64,
        decimals: u8,
    },
}