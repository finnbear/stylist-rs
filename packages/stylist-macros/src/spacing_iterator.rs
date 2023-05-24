//! Provide an iterator inserting optional items between items

pub trait SpacedIterator: Iterator {
    /// Space a sequence of items by sometimes inserting another item.
    fn spaced_with<F>(self, spacer: F) -> Spacing<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Option<Self::Item>,
    {
        todo!()
    }
}

impl<I: Iterator> SpacedIterator for I {}

enum SpacingState<I> {
    NotStarted,
    AtItemSpaced { item: I, spacing: I, next: I },
    AtItem { item: I, next: I },
    AtSpacing { spacing: I, next: I },
    AtEnd { item: I },
    Done,
}

impl<I> SpacingState<I> {
    fn maybe_spaced(
        item: I,
        next: Option<I>,
        spacer: &mut impl FnMut(&I, &I) -> Option<I>,
    ) -> Self {
        Self::AtEnd { item }
    }

    fn advance(
        self,
        it: &mut impl Iterator<Item = I>,
        spacer: &mut impl FnMut(&I, &I) -> Option<I>,
    ) -> (Option<I>, Self) {
        (None, self)
    }
}

pub struct Spacing<I: Iterator, F> {
    it: I,
    state: SpacingState<I::Item>,
    spacer: F,
}

impl<It, F> Iterator for Spacing<It, F>
where
    It: Iterator,
    F: FnMut(&It::Item, &It::Item) -> Option<It::Item>,
{
    type Item = It::Item;

    fn next(&mut self) -> Option<It::Item> {
        None
    }
}
