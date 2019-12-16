pub struct DirtyTag {}
pub struct DirtyFilter(pub bool);

impl Default for DirtyFilter {
    fn default() -> Self {
        Self(false)
    }
}