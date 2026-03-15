use crate::pattern::bit_mask::{BitMaskArena, BitMaskRef};

// Remark: Don't allow Clone for MultisetMatchState
//
#[derive(Debug)]
pub struct MultisetMatchState {
    instrs_mask: BitMaskRef,
    subjects_mask: BitMaskRef,
}

impl MultisetMatchState {
    pub fn new(arena: &mut BitMaskArena, capacity_instrs: usize, capacity_subjs: usize) -> Self {
        Self {
            instrs_mask: arena.alloc(capacity_instrs),
            subjects_mask: arena.alloc(capacity_subjs),
        }
    }

    pub fn set(&self, arena: &mut BitMaskArena, instr_index: usize, subject_index: usize) {
        arena.set(&self.instrs_mask, instr_index);
        arena.set(&self.subjects_mask, subject_index);
    }

    pub fn is_subject_set(&self, arena: &BitMaskArena, i: usize) -> bool {
        arena.is_set(&self.subjects_mask, i)
    }

    pub fn is_instruction_set(&self, arena: &BitMaskArena, i: usize) -> bool {
        arena.is_set(&self.instrs_mask, i)
    }

    pub fn is_instructions_mask_full(&self, arena: &BitMaskArena) -> bool {
        arena.is_full(&self.instrs_mask)
    }

    pub fn is_subjects_mask_full(&self, arena: &BitMaskArena) -> bool {
        arena.is_full(&self.subjects_mask)
    }

    pub fn count_unmatched_instructions(&self, arena: &BitMaskArena) -> usize {
        arena.count_unmatched(&self.instrs_mask)
    }

    pub fn count_unmatched_subjects(&self, arena: &BitMaskArena) -> usize {
        arena.count_unmatched(&self.subjects_mask)
    }

    pub fn subject_index_iter<'a>(
        &self,
        arena: &'a BitMaskArena,
        skip_set: bool,
    ) -> MultisetMatchStateIter<'a> {
        MultisetMatchStateIter {
            mask: self.subjects_mask.clone(),
            arena,
            current_pos: 0,
            skip_set,
        }
    }

    pub fn instructions_index_iter<'a>(
        &self,
        arena: &'a BitMaskArena,
        skip_set: bool,
    ) -> MultisetMatchStateIter<'a> {
        MultisetMatchStateIter {
            mask: self.instrs_mask.clone(),
            arena,
            current_pos: 0,
            skip_set,
        }
    }

    pub fn deep_clone(&self, arena: &mut BitMaskArena) -> Self {
        // Cloning just clones the references. Here we want to actually
        // allocate a new bitmask.
        Self {
            instrs_mask: arena.clone_mask(&self.instrs_mask),
            subjects_mask: arena.clone_mask(&self.subjects_mask),
        }
    }
}

pub struct MultisetMatchStateIter<'a> {
    mask: BitMaskRef,
    arena: &'a BitMaskArena,
    current_pos: usize,
    skip_set: bool,
}

impl<'a> MultisetMatchStateIter<'a> {
    fn next_filtered<F>(&mut self, pred: F) -> Option<usize>
    where
        F: Fn(&Self) -> bool,
    {
        while self.current_pos < self.mask.capacity() && !pred(self) {
            self.current_pos += 1;
        }

        if self.current_pos >= self.mask.capacity() {
            return None;
        }

        let res = self.current_pos;
        self.current_pos += 1;
        Some(res)
    }
}

impl<'a> Iterator for MultisetMatchStateIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos >= self.mask.capacity() {
            return None;
        }

        if self.skip_set {
            self.next_filtered(|s| !s.arena.is_set(&s.mask, s.current_pos))
        } else {
            let ret = self.current_pos;
            self.current_pos += 1;
            Some(ret)
        }
    }
}
