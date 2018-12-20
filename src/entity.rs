use generational_arena::Index;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity(Index);

impl From<Index> for Entity {
    fn from(index: Index) -> Self {
        Entity(index)
    }
}
