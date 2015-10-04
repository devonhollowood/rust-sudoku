/// Struct representing a row, column or box
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Group {
    elements: Vec<char>
}

impl Group {
    /// Returns whether the Group contains a given character.
    pub fn contains(&self, c: char) -> bool {
        self.elements.contains(&c)
    }
}

impl<'a> From<&'a [char]> for Group {
    /// Creates a Group, which contains the elements in `elems`
    fn from(elems: &'a [char]) -> Group {
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
