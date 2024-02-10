pub struct VecList<V> {
    free: Option<usize>,
    slots: Vec<Slot<V>>,
}

pub enum Slot<V> {
    Free(Option<usize>),
    Used(V),
}

impl<V> VecList<V> {
    pub fn new() -> Self {
        Self {
            free: None,
            slots: Vec::new(),
        }
    }

    fn grow(&mut self) {
        let last_len = self.slots.len();
        let next_free_idx = last_len;
        let free_slots = std::iter::successors(Some(Slot::Free(Some(next_free_idx + 1))), |n| {
            let Slot::Free(Some(n)) = n else {
                unreachable!()
            };
            Some(Slot::Free(Some(n + 1)))
        })
        .take(last_len)
        .chain(std::iter::once(Slot::Free(None)));

        self.free = Some(next_free_idx);
        self.slots.extend(free_slots);
    }

    pub fn append(&mut self, item: V) -> usize {
        match self.free {
            Some(idx) => {
                let next = std::mem::replace(&mut self.slots[idx], Slot::Used(item));
                let Slot::Free(next) = next else {
                    unreachable!()
                };
                self.free = next;
                idx
            }
            None => {
                self.grow();
                self.append(item)
            }
        }
    }

    pub fn remove(&mut self, idx: usize) -> Result<V, ()> {
        match &self.slots[idx] {
            Slot::Free(_) => Err(()),
            Slot::Used(_) => {
                let next = self.free.replace(idx);
                let last_slot = std::mem::replace(&mut self.slots[idx], Slot::Free(next));

                let Slot::Used(item) = last_slot else {
                    unreachable!()
                };

                Ok(item)
            }
        }
    }

    pub fn get(&self, idx: usize) -> Option<&V> {
        match &self.slots[idx] {
            Slot::Free(_) => None,
            Slot::Used(item) => Some(item),
        }
    }
}
