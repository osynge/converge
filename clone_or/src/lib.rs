pub trait CloneOr<Rhs = Self> {
    fn clone_or(&self, default: &Rhs) -> Self;
}
