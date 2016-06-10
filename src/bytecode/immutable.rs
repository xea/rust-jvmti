
///
/// Offers immutable operations that instead of mutating the current object, return new copies of
/// the collection
pub trait ImmutableCollection {
    fn immutable_append(&mut self, other: &mut Self) -> Self;
}

impl<T> ImmutableCollection for Vec<T> {

    fn immutable_append(&mut self, other: &mut Self) -> Self {
        let mut new_vec: Vec<T> = Vec::with_capacity(self.len() + other.len());
        new_vec.append(self);
        new_vec.append(other);

        new_vec
    }
}

