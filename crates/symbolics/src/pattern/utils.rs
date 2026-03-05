use crate::pattern::bit_mask::BitMask;

#[derive(Debug, Clone)]
pub struct MultisetMatchState {
    instrs_mask: BitMask,
    subjects_mask: BitMask,
}

impl MultisetMatchState {
    pub fn new(capacity_instrs: usize, capacity_subjs: usize) -> Self {
        Self {
            instrs_mask: BitMask::new(capacity_instrs),
            subjects_mask: BitMask::new(capacity_subjs),
        }
    }

    pub fn set(&mut self, instr_index: usize, subject_index: usize) {
        debug_assert!(instr_index < self.instrs_mask.len());
        debug_assert!(subject_index < self.subjects_mask.len());

        self.instrs_mask.set(instr_index);
        self.subjects_mask.set(subject_index);
    }

    pub fn is_subject_set(&self, i: usize) -> bool {
        self.subjects_mask.is_set(i)
    }

    pub fn is_instruction_set(&self, i: usize) -> bool {
        self.instrs_mask.is_set(i)
    }

    pub fn is_instructions_mask_full(&self) -> bool {
        self.instrs_mask.is_full()
    }

    pub fn is_subjects_mask_full(&self) -> bool {
        self.subjects_mask.is_full()
    }

    pub fn count_unmatched_instructions(&self) -> usize {
        self.instrs_mask.count_unmatched()
    }

    pub fn count_unmatched_subjects(&self) -> usize {
        self.subjects_mask.count_unmatched()
    }

    pub fn subject_index_iter<'a>(&'a self, skip_set: bool) -> MultisetMatchStateIter<'a> {
        MultisetMatchStateIter {
            mask: &self.subjects_mask,
            current_pos: 0,
            skip_set,
        }
    }

    pub fn instructions_index_iter<'a>(&'a self, skip_set: bool) -> MultisetMatchStateIter<'a> {
        MultisetMatchStateIter {
            mask: &self.instrs_mask,
            current_pos: 0,
            skip_set,
        }
    }
}

pub struct MultisetMatchStateIter<'a> {
    mask: &'a BitMask,
    current_pos: usize,
    skip_set: bool,
}

impl<'a> MultisetMatchStateIter<'a> {
    fn next_filtered<F>(&mut self, pred: F) -> Option<usize>
    where
        F: Fn(&Self) -> bool,
    {
        while self.current_pos < self.mask.len() && !pred(self) {
            self.current_pos += 1;
        }

        if self.current_pos >= self.mask.len() {
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
        if self.current_pos >= self.mask.len() {
            return None;
        }

        if self.skip_set {
            self.next_filtered(|s| !s.mask.is_set(s.current_pos))
        } else {
            let ret = self.current_pos;
            self.current_pos += 1;
            Some(ret)
        }
    }
}
