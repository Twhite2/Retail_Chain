use anchor_lang::prelude::*;

#[error_code]
pub enum SupplyChainError {
    // Authentication/Authorization Errors
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    
    #[msg("Only verified entities can perform this action")]
    VerificationRequired,
    
    #[msg("Only authorized verifiers can perform this verification")]
    UnauthorizedVerifier,
    
    // Supply Chain Status Errors
    #[msg("Invalid status transition")]
    InvalidStatusTransition,
    
    #[msg("The item is already in this status")]
    AlreadyInStatus,
    
    #[msg("This action cannot be performed on a completed shipment")]
    ShipmentAlreadyCompleted,
    
    #[msg("This store is currently inactive")]
    StoreInactive,
    
    // Agreement Related Errors
    #[msg("Agreement is in an invalid status for this operation")]
    InvalidAgreementStatus,
    
    #[msg("Agreement deadline must be in the future")]
    InvalidDeadline,
    
    #[msg("Payment amount must be greater than zero")]
    InvalidPaymentAmount,
    
    #[msg("This agreement has already been accepted")]
    AgreementAlreadyAccepted,
    
    #[msg("This agreement has already been completed")]
    AgreementAlreadyCompleted,
    
    #[msg("This agreement has been canceled")]
    AgreementCanceled,
    
    #[msg("This agreement is in dispute and requires resolution")]
    AgreementInDispute,
    
    #[msg("The dispute is already resolved")]
    DisputeAlreadyResolved,
    
    // Relationship Errors
    #[msg("Invalid relationship between entities")]
    InvalidRelationship,
    
    #[msg("This supplier is not related to this store")]
    SupplierNotRelated,
    
    #[msg("This shipment is not related to this agreement")]
    ShipmentNotRelated,
    
    // Data Validation Errors
    #[msg("The provided data is invalid")]
    InvalidData,
    
    #[msg("Required field is missing")]
    MissingRequiredField,
    
    #[msg("String length exceeds maximum allowed")]
    StringTooLong,
    
    #[msg("Product quantity must be greater than zero")]
    InvalidQuantity,
    
    #[msg("Price must be greater than zero")]
    InvalidPrice,
    
    #[msg("Rating must be between 0 and 5")]
    InvalidRating,
    
    // IoT Data Errors
    #[msg("Invalid IoT data type")]
    InvalidIoTDataType,
    
    #[msg("IoT data is out of acceptable range")]
    IoTDataOutOfRange,
    
    #[msg("IoT data has already been verified")]
    IoTDataAlreadyVerified,
    
    #[msg("Timestamp is invalid or in the future")]
    InvalidTimestamp,
    
    // Technical Errors
    #[msg("Arithmetic operation failed")]
    ArithmeticError,
    
    #[msg("Failed to serialize or deserialize data")]
    SerializationError,
    
    #[msg("Account ownership is invalid")]
    InvalidOwner,
    
    #[msg("Program is not authorized to perform this operation")]
    ProgramUnauthorized,
    
    // General Operational Errors
    #[msg("Operation timeout exceeded")]
    OperationTimeout,
    
    #[msg("Resource limit exceeded")]
    ResourceLimitExceeded,
    
    #[msg("Duplicate entry found")]
    DuplicateEntry,
    
    #[msg("Referenced entity was not found")]
    EntityNotFound,
    
    // Compliance Errors
    #[msg("This operation would violate compliance requirements")]
    ComplianceViolation,
    
    #[msg("Required verification is missing")]
    MissingVerification,
    
    #[msg("Required certification is missing or expired")]
    CertificationRequired,
    
    // Catch-all for unexpected errors
    #[msg("An unexpected error occurred")]
    UnexpectedError,
}
