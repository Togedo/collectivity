/// Groups types used to indicate whether a trait implementation may panic
pub trait SafetyMarker {}

/// Indicates a trait implementation may not panic
pub struct Safe {}
impl SafetyMarker for Safe {}

/// Indicates a trait implementation may panic
pub struct Unsafe {}
impl SafetyMarker for Unsafe {}
