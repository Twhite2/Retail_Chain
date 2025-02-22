use anchor_lang::prelude::*;

declare_id!("7JYPt6XXmADUzAG12ZM3763PuF7XhJmfr7oWV9g2VrcM");

#[program]
pub mod retailchain {
    use super::*;

    pub fn initialize_store(
        ctx: Context<InitializeStore>,
        name: String,
        location: String
    ) -> Result<()> {
        let store = &mut ctx.accounts.store;
        store.owner = ctx.accounts.owner.key();
        store.name = name;
        store.location = location;
        store.total_products = 0;
        store.is_active = true;
        Ok(())
    }

    pub fn add_product(
        ctx: Context<AddProduct>,
        name: String,
        description: String,
        price: u64,
        quantity: u64
    ) -> Result<()> {
        let product = &mut ctx.accounts.product;
        let store = &mut ctx.accounts.store;

        product.store = store.key();
        product.name = name;
        product.description = description;
        product.price = price;
        product.quantity = quantity;
        product.created_at = Clock::get()?.unix_timestamp;
        
        store.total_products = store.total_products.checked_add(1).unwrap();
        
        Ok(())
    }

    pub fn update_product(
        ctx: Context<UpdateProduct>,
        price: Option<u64>,
        quantity: Option<u64>
    ) -> Result<()> {
        let product = &mut ctx.accounts.product;
        
        if let Some(new_price) = price {
            product.price = new_price;
        }
        
        if let Some(new_quantity) = quantity {
            product.quantity = new_quantity;
        }
        
        Ok(())
    }
}

#[account]
pub struct Store {
    pub owner: Pubkey,
    pub name: String,
    pub location: String,
    pub total_products: u64,
    pub is_active: bool,
}

#[account]
pub struct Product {
    pub store: Pubkey,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub quantity: u64,
    pub created_at: i64,
}

#[derive(Accounts)]
pub struct InitializeStore<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 32 + 8 + 1
    )]
    pub store: Account<'info, Store>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddProduct<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        constraint = store.owner == owner.key(),
        constraint = store.is_active
    )]
    pub store: Account<'info, Store>,
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 32 + 8 + 8 + 8
    )]
    pub product: Account<'info, Product>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProduct<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        constraint = store.owner == owner.key(),
        constraint = store.is_active
    )]
    pub store: Account<'info, Store>,
    #[account(
        mut,
        constraint = product.store == store.key()
    )]
    pub product: Account<'info, Product>,
}