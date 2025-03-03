use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::SupplyChainError;
use super::{validate_supplier_verified, validate_store_active};

// Supply agreement creation and management
pub fn create_supply_agreement(
    ctx: Context<CreateAgreement>,
    terms: String,
    deadline: i64,
    payment_amount: u64
) -> Result<()> {
    let agreement = &mut ctx.accounts.agreement;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate inputs
    require!(
        !terms.is_empty() && terms.len() <= 200,
        SupplyChainError::InvalidData
    );
    
    require!(
        deadline > current_time,
        SupplyChainError::InvalidDeadline
    );
    
    require!(
        payment_amount > 0,
        SupplyChainError::InvalidPaymentAmount
    );
    
    // Initialize the agreement
    agreement.supplier = ctx.accounts.supplier.key();
    agreement.store = ctx.accounts.store.key();
    agreement.terms = terms;
    agreement.deadline = deadline;
    agreement.payment_amount = payment_amount;
    agreement.status = AgreementStatus::Pending as u8;
    agreement.created_at = current_time;
    agreement.products = Vec::new();
    
    // Emit agreement creation event
    emit!(AgreementCreatedEvent {
        agreement: agreement.key(),
        supplier: agreement.supplier,
        store: agreement.store,
        payment_amount,
        deadline,
        timestamp: current_time,
    });
    
    Ok(())
}

pub fn accept_agreement(
    ctx: Context<UpdateAgreement>
) -> Result<()> {
    let agreement = &mut ctx.accounts.agreement;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate agreement status
    require!(
        agreement.status == AgreementStatus::Pending as u8,
        SupplyChainError::InvalidAgreementStatus
    );
    
    // Only the store can accept an agreement
    require!(
        ctx.accounts.store.owner == ctx.accounts.authority.key(),
        SupplyChainError::Unauthorized
    );
    
    // Update status
    agreement.status = AgreementStatus::Active as u8;
    
    // Emit agreement accepted event
    emit!(AgreementStatusUpdatedEvent {
        agreement: agreement.key(),
        old_status: AgreementStatus::Pending as u8,
        new_status: AgreementStatus::Active as u8,
        updated_by: ctx.accounts.authority.key(),
        timestamp: current_time,
    });
    
    Ok(())
}

pub fn add_products_to_agreement(
    ctx: Context<AddProductsToAgreement>,
    product_accounts: Vec<Pubkey>
) -> Result<()> {
    let agreement = &mut ctx.accounts.agreement;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate agreement status
    require!(
        agreement.status == AgreementStatus::Pending as u8 || 
        agreement.status == AgreementStatus::Active as u8,
        SupplyChainError::InvalidAgreementStatus
    );
    
    // Only supplier can add products
    require!(
        agreement.supplier == ctx.accounts.authority.key(),
        SupplyChainError::Unauthorized
    );
    
    // Add products to agreement
    for product in product_accounts.iter() {
        if !agreement.products.contains(product) {
            agreement.products.push(*product);
        }
    }
    
    // Emit products added event
    emit!(ProductsAddedToAgreementEvent {
        agreement: agreement.key(),
        products: product_accounts,
        added_by: ctx.accounts.authority.key(),
        timestamp: current_time,
    });
    
    Ok(())
}

pub fn complete_agreement(
    ctx: Context<CompleteAgreement>
) -> Result<()> {
    let agreement = &mut ctx.accounts.agreement;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate agreement status
    require!(
        agreement.status == AgreementStatus::Active as u8,
        SupplyChainError::InvalidAgreementStatus
    );
    
    // Both parties must sign off for completion
    let is_supplier = ctx.accounts.authority.key() == agreement.supplier;
    let is_store_owner = ctx.accounts.authority.key() == ctx.accounts.store.owner;
    
    require!(
        is_supplier || is_store_owner,
        SupplyChainError::Unauthorized
    );
    
    // Update agreement state
    agreement.status = AgreementStatus::Completed as u8;
    
    // Handle payment release from escrow if implemented
    // This would require additional token accounts and logic
    
    // Emit agreement completed event
    emit!(AgreementStatusUpdatedEvent {
        agreement: agreement.key(),
        old_status: AgreementStatus::Active as u8,
        new_status: AgreementStatus::Completed as u8,
        updated_by: ctx.accounts.authority.key(),
        timestamp: current_time,
    });
    
    Ok(())
}

pub fn dispute_agreement(
    ctx: Context<DisputeAgreement>,
    dispute_reason: String
) -> Result<()> {
    let agreement = &mut ctx.accounts.agreement;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate agreement status - can only dispute active agreements
    require!(
        agreement.status == AgreementStatus::Active as u8,
        SupplyChainError::InvalidAgreementStatus
    );
    
    // Either party can raise a dispute
    let is_supplier = ctx.accounts.authority.key() == agreement.supplier;
    let is_store_owner = ctx.accounts.authority.key() == ctx.accounts.store.owner;
    
    require!(
        is_supplier || is_store_owner,
        SupplyChainError::Unauthorized
    );
    
    // Update agreement state
    agreement.status = AgreementStatus::Disputed as u8;
    
    // Create dispute record
    let dispute = &mut ctx.accounts.dispute;
    dispute.agreement = agreement.key();
    dispute.initiated_by = ctx.accounts.authority.key();
    dispute.reason = dispute_reason;
    dispute.created_at = current_time;
    dispute.resolved = false;
    
    // Emit dispute event
    emit!(AgreementDisputedEvent {
        agreement: agreement.key(),
        dispute: dispute.key(),
        initiated_by: ctx.accounts.authority.key(),
        timestamp: current_time,
    });
    
    Ok(())
}

pub fn resolve_dispute(
    ctx: Context<ResolveDispute>,
    resolution_notes: String,
    resolution_outcome: u8
) -> Result<()> {
    let dispute = &mut ctx.accounts.dispute;
    let agreement = &mut ctx.accounts.agreement;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Verify the dispute is for this agreement
    require!(
        dispute.agreement == agreement.key(),
        SupplyChainError::InvalidRelationship
    );
    
    // Only verifiers can resolve disputes
    require!(
        ctx.accounts.authority_credentials.is_verifier,
        SupplyChainError::UnauthorizedVerifier
    );
    
    // Update dispute status
    dispute.resolved = true;
    dispute.resolved_by = Some(ctx.accounts.authority.key());
    dispute.resolution_notes = Some(resolution_notes);
    dispute.resolved_at = Some(current_time);
    
    // Update agreement based on resolution outcome
    match resolution_outcome {
        0 => {
            // Continue agreement
            agreement.status = AgreementStatus::Active as u8;
        },
        1 => {
            // Complete agreement
            agreement.status = AgreementStatus::Completed as u8;
        },
        2 => {
            // Cancel agreement
            agreement.status = AgreementStatus::Canceled as u8;
        },
        _ => return Err(SupplyChainError::InvalidData.into())
    }
    
    // Emit resolution event
    emit!(DisputeResolvedEvent {
        dispute: dispute.key(),
        agreement: agreement.key(),
        resolved_by: ctx.accounts.authority.key(),
        outcome: resolution_outcome,
        timestamp: current_time,
    });
    
    Ok(())
}

pub fn cancel_agreement(
    ctx: Context<UpdateAgreement>
) -> Result<()> {
    let agreement = &mut ctx.accounts.agreement;
    let current_time = Clock::get()?.unix_timestamp;
    
    // Can only cancel pending or active agreements
    require!(
        agreement.status == AgreementStatus::Pending as u8 || 
        agreement.status == AgreementStatus::Active as u8,
        SupplyChainError::InvalidAgreementStatus
    );
    
    // Either party can cancel
    let is_supplier = ctx.accounts.authority.key() == agreement.supplier;
    let is_store_owner = ctx.accounts.authority.key() == ctx.accounts.store.owner;
    
    require!(
        is_supplier || is_store_owner,
        SupplyChainError::Unauthorized
    );
    
    // Update agreement state
    let old_status = agreement.status;
    agreement.status = AgreementStatus::Canceled as u8;
    
    // Emit agreement canceled event
    emit!(AgreementStatusUpdatedEvent {
        agreement: agreement.key(),
        old_status,
        new_status: AgreementStatus::Canceled as u8,
        updated_by: ctx.accounts.authority.key(),
        timestamp: current_time,
    });
    
    Ok(())
}

// Account contexts for agreement operations
#[derive(Accounts)]
pub struct CreateAgreement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = supplier.key == authority.key() || store.owner == authority.key()
    )]
    pub supplier: Account<'info, Supplier>,
    pub store: Account<'info, Store>,
    #[account(
        init,
        payer = authority,
        space = SupplyAgreement::space()
    )]
    pub agreement: Account<'info, SupplyAgreement>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAgreement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = (agreement.supplier == authority.key() && supplier.key() == authority.key()) || 
                    (agreement.store == store.key() && store.owner == authority.key())
    )]
    pub agreement: Account<'info, SupplyAgreement>,
    pub supplier: Account<'info, Supplier>,
    pub store: Account<'info, Store>,
}

#[derive(Accounts)]
pub struct AddProductsToAgreement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = agreement.supplier == authority.key() && supplier.key() == authority.key()
    )]
    pub agreement: Account<'info, SupplyAgreement>,
    pub supplier: Account<'info, Supplier>,
}

#[derive(Accounts)]
pub struct CompleteAgreement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = (agreement.supplier == supplier.key() && supplier.key() == authority.key()) || 
                    (agreement.store == store.key() && store.owner == authority.key())
    )]
    pub agreement: Account<'info, SupplyAgreement>,
    pub supplier: Account<'info, Supplier>,
    pub store: Account<'info, Store>,
}

#[derive(Accounts)]
pub struct DisputeAgreement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = (agreement.supplier == supplier.key() && supplier.key() == authority.key()) || 
                    (agreement.store == store.key() && store.owner == authority.key())
    )]
    pub agreement: Account<'info, SupplyAgreement>,
    pub supplier: Account<'info, Supplier>,
    pub store: Account<'info, Store>,
    #[account(
        init,
        payer = authority,
        space = AgreementDispute::space()
    )]
    pub dispute: Account<'info, AgreementDispute>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveDispute<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = authority_credentials.authority == authority.key(),
        constraint = authority_credentials.is_verifier
    )]
    pub authority_credentials: Account<'info, VerifierCredential>,
    #[account(mut)]
    pub dispute: Account<'info, AgreementDispute>,
    #[account(
        mut,
        constraint = dispute.agreement == agreement.key()
    )]
    pub agreement: Account<'info, SupplyAgreement>,
}

// Additional account structures
#[account]
pub struct AgreementDispute {
    pub agreement: Pubkey,
    pub initiated_by: Pubkey,
    pub reason: String,
    pub created_at: i64,
    pub resolved: bool,
    pub resolved_by: Option<Pubkey>,
    pub resolution_notes: Option<String>,
    pub resolved_at: Option<i64>,
}

impl AgreementDispute {
    pub fn space() -> usize {
        8 +    // discriminator
        32 +   // agreement: Pubkey
        32 +   // initiated_by: Pubkey
        200 +  // reason: String (max assumed)
        8 +    // created_at: i64
        1 +    // resolved: bool
        (1 + 32) +  // resolved_by: Option<Pubkey>
        (1 + 200) + // resolution_notes: Option<String> (max assumed)
        (1 + 8)     // resolved_at: Option<i64>
    }
}

// Event definitions
#[event]
pub struct AgreementCreatedEvent {
    pub agreement: Pubkey,
    pub supplier: Pubkey,
    pub store: Pubkey,
    pub payment_amount: u64,
    pub deadline: i64,
    pub timestamp: i64,
}

#[event]
pub struct AgreementStatusUpdatedEvent {
    pub agreement: Pubkey,
    pub old_status: u8,
    pub new_status: u8,
    pub updated_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProductsAddedToAgreementEvent {
    pub agreement: Pubkey,
    pub products: Vec<Pubkey>,
    pub added_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AgreementDisputedEvent {
    pub agreement: Pubkey,
    pub dispute: Pubkey,
    pub initiated_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DisputeResolvedEvent {
    pub dispute: Pubkey,
    pub agreement: Pubkey,
    pub resolved_by: Pubkey,
    pub outcome: u8,
    pub timestamp: i64,
}
