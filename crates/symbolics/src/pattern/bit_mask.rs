type BitMaskBase = u128;

#[inline]
fn base_bit_count() -> usize {
    std::mem::size_of::<BitMaskBase>() * 8
}

#[inline]
fn bit_mask_location(i: usize) -> (usize, usize) {
    (i / base_bit_count(), i % base_bit_count())
}

#[inline]
fn chunk_count(capacity: usize) -> usize {
    capacity.div_ceil(base_bit_count())
}

pub struct BitMaskArena {
    chunks: Vec<BitMaskBase>,
}

#[derive(Debug, Clone)]
pub struct BitMaskRef {
    start: usize,
    capacity: usize,
}

impl BitMaskRef {
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl BitMaskArena {
    pub fn new() -> Self {
        BitMaskArena { chunks: Vec::new() }
    }

    pub fn alloc(&mut self, capacity: usize) -> BitMaskRef {
        let start = self.chunks.len();
        let n = chunk_count(capacity);

        self.chunks.extend(std::iter::repeat(0).take(n));
        BitMaskRef { start, capacity }
    }

    pub fn clone_mask(&mut self, mask: &BitMaskRef) -> BitMaskRef {
        let start = self.chunks.len();
        let n = chunk_count(mask.capacity);
        let src_start = mask.start;

        let cloned: Vec<BitMaskBase> = self.chunks[src_start..src_start + n].to_vec();
        self.chunks.extend_from_slice(&cloned);

        BitMaskRef {
            start,
            capacity: mask.capacity,
        }
    }

    pub fn set(&mut self, mask: &BitMaskRef, i: usize) {
        if i >= mask.capacity {
            return;
        }
        let (k, l) = bit_mask_location(i);
        self.chunks[mask.start + k] |= 1 << l;
    }

    pub fn is_set(&self, mask: &BitMaskRef, i: usize) -> bool {
        if i >= mask.capacity {
            return false;
        }
        let (k, l) = bit_mask_location(i);
        self.chunks[mask.start + k] & (1 << l) != 0
    }

    pub fn is_full(&self, mask: &BitMaskRef) -> bool {
        self.count_matched(mask) == mask.capacity
    }

    pub fn count_matched(&self, mask: &BitMaskRef) -> usize {
        let n = chunk_count(mask.capacity);
        self.chunks[mask.start..mask.start + n]
            .iter()
            .map(|m| m.count_ones() as usize)
            .sum()
    }

    pub fn count_unmatched(&self, mask: &BitMaskRef) -> usize {
        mask.capacity - self.count_matched(mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty_mask() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(10);

        assert_eq!(arena.count_matched(&mask), 0);
        assert_eq!(arena.count_unmatched(&mask), 10);
        assert_eq!(mask.capacity, 10);
        assert!(!arena.is_full(&mask));
    }

    #[test]
    fn test_set_and_is_set() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(10);

        arena.set(&mask, 0);
        arena.set(&mask, 5);
        arena.set(&mask, 9);
        assert!(arena.is_set(&mask, 0));
        assert!(arena.is_set(&mask, 5));
        assert!(arena.is_set(&mask, 9));
        assert!(!arena.is_set(&mask, 1));
        assert!(!arena.is_set(&mask, 4));
    }

    #[test]
    fn test_count_matched() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(10);

        assert_eq!(arena.count_matched(&mask), 0);
        arena.set(&mask, 0);
        assert_eq!(arena.count_matched(&mask), 1);
        arena.set(&mask, 5);
        assert_eq!(arena.count_matched(&mask), 2);
        arena.set(&mask, 9);
        assert_eq!(arena.count_matched(&mask), 3);
    }

    #[test]
    fn test_is_full() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(4);

        arena.set(&mask, 0);
        arena.set(&mask, 1);
        arena.set(&mask, 2);
        assert!(!arena.is_full(&mask));
        arena.set(&mask, 3);
        assert!(arena.is_full(&mask));
    }

    #[test]
    fn test_set_idempotent() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(10);

        arena.set(&mask, 3);
        arena.set(&mask, 3);
        arena.set(&mask, 3);
        assert_eq!(arena.count_matched(&mask), 1);
    }

    #[test]
    fn test_set_out_of_bounds_does_not_panic() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(10);

        arena.set(&mask, 10);
        arena.set(&mask, 99);
        assert_eq!(arena.count_matched(&mask), 0);
    }

    #[test]
    fn test_across_first_chunk_boundary() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(130);

        arena.set(&mask, 127);
        arena.set(&mask, 128);
        assert!(arena.is_set(&mask, 127));
        assert!(arena.is_set(&mask, 128));
        assert!(!arena.is_set(&mask, 126));
        assert!(!arena.is_set(&mask, 129));
        assert_eq!(arena.count_matched(&mask), 2);
    }
    #[test]
    fn test_across_second_chunk_boundary() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(260);

        arena.set(&mask, 255);
        arena.set(&mask, 256);
        assert!(arena.is_set(&mask, 255));
        assert!(arena.is_set(&mask, 256));
        assert_eq!(arena.count_matched(&mask), 2);
    }

    #[test]
    fn test_across_third_chunk_boundary() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(400);

        arena.set(&mask, 383);
        arena.set(&mask, 384);
        assert!(arena.is_set(&mask, 383));
        assert!(arena.is_set(&mask, 384));
        assert_eq!(arena.count_matched(&mask), 2);
    }

    #[test]
    fn test_expand_129_capacity() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(130);

        for i in 0..130 {
            arena.set(&mask, i);
        }
        assert!(arena.is_full(&mask));
        assert_eq!(arena.count_matched(&mask), 130);
        assert_eq!(arena.count_unmatched(&mask), 0);
    }

    #[test]
    fn test_clone_is_independent() {
        let mut arena = BitMaskArena::new();
        let mask = arena.alloc(10);
        arena.set(&mask, 3);

        let cloned = arena.clone_mask(&mask);
        arena.set(&cloned, 7);

        assert!(!arena.is_set(&mask, 7));
        assert!(arena.is_set(&cloned, 7));
        assert_eq!(arena.count_matched(&mask), 1);
        assert_eq!(arena.count_matched(&cloned), 2);
    }
}
