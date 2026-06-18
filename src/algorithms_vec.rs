use std::borrow::Borrow;

pub trait RemoveIndices {
    fn remove_indices<I>(&mut self, indices: I)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>;
}

impl<T> RemoveIndices for Vec<T> {
    fn remove_indices<I>(&mut self, indices: I)
    where
        I: IntoIterator,
        I::Item: Borrow<usize>,
    {
        let mut indices = indices
            .into_iter()
            .map(|index| *index.borrow())
            .collect::<Vec<_>>();
        if indices.is_empty() {
            return;
        }

        indices.sort_unstable();
        indices.dedup();

        let mut remove_indices = indices.into_iter().peekable();
        let mut index = 0;
        self.retain(|_| {
            let remove = remove_indices
                .peek()
                .is_some_and(|remove_index| *remove_index == index);
            if remove {
                remove_indices.next();
            }
            index += 1;
            !remove
        });
    }
}

#[cfg(test)]
mod tests {
    use super::RemoveIndices as _;

    #[test]
    fn removes_indices_in_one_pass() {
        let mut values = vec![0, 1, 2, 3, 4, 5];

        values.remove_indices([1, 4]);

        assert_eq!(values, vec![0, 2, 3, 5]);
    }

    #[test]
    fn sorts_and_deduplicates_indices() {
        let mut values = vec![0, 1, 2, 3, 4, 5];

        values.remove_indices([4, 1, 4, 0]);

        assert_eq!(values, vec![2, 3, 5]);
    }

    #[test]
    fn ignores_out_of_bounds_indices() {
        let mut values = vec![0, 1, 2];

        values.remove_indices([8, 1]);

        assert_eq!(values, vec![0, 2]);
    }

    #[test]
    fn accepts_borrowed_indices() {
        let mut values = vec![0, 1, 2, 3, 4];
        let indices = vec![1, 3];

        values.remove_indices(&indices);

        assert_eq!(values, vec![0, 2, 4]);
    }
}
