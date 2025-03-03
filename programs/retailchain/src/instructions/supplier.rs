use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::SupplyChainError;

// Supplier registration and management
pub fn register_supplier(
    ctx: Context<RegisterSupplier>,
    name: String,
    certification: String,
    description: String
) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;
    supplier.key = ctx.accounts.authority.key();
    supplier.name = name;
    supplier.certification = certification;
    supplier.description = description;
    supplier.products_supplied = 0;
    supplier.is_verified = false;
    supplier.rating = 0;
    supplier.created_at = Clock::get()?.unix_timestamp;
    
    Ok(())
}

pub fn verify_supplier(
    ctx: Context<VerifySupplier>
) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;
    
    // Only allow verification by authorized verifiers
    require!(
        ctx.accounts.authority.key() == supplier.key || 
        ctx.accounts.authority_credentials.is_verifier, 
        SupplyChainError::UnauthorizedVerifier
    );
    
    supplier.is_verified = true;
    
    Ok(())
}

pub fn update_supplier(
    ctx: Context<UpdateSupplier>,
    certification: Option<String>,
    description: Option<String>
) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;
    
    // Only the supplier themselves can update their information
    require!(
        ctx.accounts.authority.key() == supplier.key,
        SupplyChainError::Unauthorized
    );
    
    if let Some(new_certification) = certification {
        supplier.certification = new_certification;
    }
    
    if let Some(new_description) = description {
        supplier.description = new_description;
    }
    
    Ok(())
}

pub fn rate_supplier(
    ctx: Context<RateSupplier>,
    rating: u8
) -> Result<()> {
    let supplier = &mut ctx.accounts.supplier;
    
    // Rating must be between 0 and 5
    require!(
        rating <= 5,
        SupplyChainError::InvalidData
    );
    
    // Only stores that have completed transactions with this supplier can rate
    require!(
        ctx.accounts.agreement.store == ctx.accounts.store.key() &&
        ctx.accounts.agreement.supplier == supplier.key &&
        ctx.accounts.agreement.status == AgreementStatus::Completed as u8,
        SupplyChainError::Unauthorized
    );
    
    supplier.rating = rating;
    
    Ok(())
}

// Account contexts for supplier operations
#[derive(Accounts)]
pub struct RegisterSupplier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = Supplier::space()
    )]
    pub supplier: Account<'info, Supplier>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifySupplier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = authority_credentials.authority == authority.key(),
        constraint = authority_credentials.is_verifier
    )]
    pub authority_credentials: Account<'info, VerifierCredential>,
    #[account(mut)]
    pub supplier: Account<'info, Supplier>,
}

#[derive(Accounts)]
pub struct UpdateSupplier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = supplier.key == authority.key()
    )]
    pub supplier: Account<'info, Supplier>,
}

#[derive(Accounts)]
pub struct RateSupplier<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = store.owner == authority.key()
    )]
    pub store: Account<'info, Store>,
    #[account(
        constraint = agreement.store == store.key(),
        constraint = agreement.supplier == supplier.key,
        constraint = agreement.status == AgreementStatus::Completed as u8
    )]
    pub agreement: Account<'info, SupplyAgreement>,
    #[account(mut)]
    pub supplier: Account<'info, Supplier>,
}

// Additional supplier-product relationship functionality
pub fn add_product_to_supplier_catalog(
    ctx: Context<AddProductToSupplierCatalog>,
    name: String,
    description: String,
    price: u64,
    available_quantity: u64
) -> Result<()> {
    let supplier_product = &mut ctx.accounts.supplier_product;
    let supplier = &mut ctx.accounts.supplier;
    
    // Verify the supplier is verified before they can add products
    require!(
        supplier.is_verified,
        SupplyChainError::VerificationRequired
    );
    
    // Set up the supplier product
    supplier_product.supplier = supplier.key;
    supplier_product.name = name;
    supplier_product.description = description;
    supplier_product.price = price;
    supplier_product.available_quantity = available_quantity;
    supplier_product.created_at = Clock::get()?.unix_timestamp;
    
    // Increment the supplier's product count
    supplier.products_supplied = supplier.products_supplied.checked_add(1)
        .ok_or(SupplyChainError::ArithmeticError)?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct AddProductToSupplierCatalog<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = supplier.key == authority.key(),
        constraint = supplier.is_verified
    )]
    pub supplier: Account<'info, Supplier>,
    #[account(
        init,
        payer = authority,
        space = SupplierProduct::space()
    )]
    pub supplier_product: Account<'info, SupplierProduct>,
    pub system_program: Program<'info, System>,
}

// Additional account structure for supplier catalog
#[account]
pub struct SupplierProduct {
    pub supplier: Pubkey,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub available_quantity: u64,
    pub created_at: i64,
}

impl SupplierProduct {
    pub fn space() -> usize {
        8 +   // discriminator
        32 +  // supplier: Pubkey
        32 +  // name: String (max assumed)
        128 + // description: String (max assumed)
        8 +   // price: u64
        8 +   // available_quantity: u64
        8     // created_at: i64
    }
}
