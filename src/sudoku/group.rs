/// Struct representing a row, column or box

use std::slice::Iter;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Group<T> {
    elements: Vec<T>
}

impl<T: PartialEq> Group<T> {
    /// Returns whether the Group contains a given character.
    pub fn contains(&self, elem: T) -> bool {
        self.elements.contains(&elem)
    }

    pub fn iter<'a>(&'a self) -> GroupIter<'a, T> {
        GroupIter::new(self.elements.iter())
    }
}

impl<'a, T: Clone> From<&'a [T]> for Group<T> {
    /// Creates a Group, which contains the elements in `elems`
    fn from(elems: &'a [T]) -> Group<T> {
        Group { elements: Vec::from(elems) }
    }
}

pub struct GroupIter<'a, T: 'a> {
    iter: Iter<'a, T>,
}

impl<'a, T> GroupIter<'a, T> {
    fn new(iter: Iter<T>) -> GroupIter<T> {
        GroupIter { iter: iter }
    }
}

impl<'a, T> Iterator for GroupIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
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
