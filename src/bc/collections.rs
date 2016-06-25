///
/// Offers immutable operations that instead of mutating the current object, return new copies of
/// the collection
pub trait ImmutableCollection {

    /// Append the elements of `other` to the end of the current collection and return the result
    /// as a new collection, leaving the original ones unchanged.
    fn immutable_append(&mut self, other: &mut Self) -> Self;
}

impl<T: Clone> ImmutableCollection for Vec<T> {

    fn immutable_append(&mut self, other: &mut Self) -> Self {
        let mut new_vec: Vec<T> = Vec::with_capacity(self.len() + other.len());

        new_vec.append(&mut self.clone());
        new_vec.append(&mut other.clone());

        new_vec
    }
}
