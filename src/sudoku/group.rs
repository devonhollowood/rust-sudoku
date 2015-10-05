/// Struct representing a row, column or box
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Group<T> {
    elements: Vec<T>
}

impl<T: PartialEq> Group<T> {
    /// Returns whether the Group contains a given character.
    pub fn contains(&self, c: T) -> bool {
        self.elements.contains(&c)
    }
}

impl<'a, T: Clone> From<&'a [T]> for Group<T> {
    /// Creates a Group, which contains the elements in `elems`
    fn from(elems: &'a [T]) -> Group<T> {
        Group { elements: Vec::from(elems) }
    }
}

#[cfg(test)]
mod tests {
    use super::Group;

    #[test]
    fn contains() {
        let g = Group::from(&vec!['1', '4', '6', '8', '9'][..]);
        assert_eq!(g.contains('1'), true);
        assert_eq!(g.contains('2'), false);
    }
}
