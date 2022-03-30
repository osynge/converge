pub trait CloneWithDefault<Rhs = Self> {
    fn clone_with_default(&self, default: &Rhs) -> Self;
}
