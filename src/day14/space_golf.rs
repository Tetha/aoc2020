
use super::MaskValueV2;

#[derive(Clone, Debug, Copy)]
enum FloatingBitState {
    Nothing,
    ZeroReturned,
    OneReturned,
}
struct AddressIterator<'a> {
    mask: &'a Vec<MaskValueV2>,
    address: &'a u64,
    state: Vec<FloatingBitState>,
    max_reached: bool,
    returned_once: bool,
}

impl<'a> AddressIterator<'a> {
    fn new(mask: &'a Vec<MaskValueV2>, address: &'a u64) -> AddressIterator<'a> {
        AddressIterator{
            mask, address,
            state: itertools::repeat_n(FloatingBitState::Nothing, 36).collect::<Vec<FloatingBitState>>(),
            max_reached: false,
            returned_once: false,
        }
    }
}
impl<'a> Iterator for AddressIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        println!("-----");
        println!("before update {:?}", self.state);
        if self.max_reached {
            return None;
        }
        let old_state = self.state.clone();
        let mut carry = false;
        for (idx, (state, mask)) in itertools::zip(&old_state, self.mask).enumerate() {
            match mask {
                // these are just static anyway
                MaskValueV2::PassThrough => {
                    self.max_reached = idx == self.mask.len() - 1;
                },
                MaskValueV2::SetToOne => {
                    self.max_reached = idx == self.mask.len() - 1;
                },
                MaskValueV2::Floating => {
                    match state {
                        FloatingBitState::Nothing => {
                            self.state[idx] = if carry { FloatingBitState::OneReturned } else { FloatingBitState::ZeroReturned };
                            break
                        }
                        FloatingBitState::ZeroReturned => {
                            //self.state[idx] = if carry { FloatingBitState::OneReturned } else { FloatingBitState::ZeroReturned };
                            self.state[idx] = FloatingBitState::OneReturned;
                            break;
                        }
                        FloatingBitState::OneReturned => {
                            self.state[idx] = FloatingBitState::ZeroReturned;
                            carry = true;
                            self.max_reached = idx == self.mask.len() - 1;
                            continue;
                        }
                    }
                }
            }
        }

        if self.max_reached && self.returned_once {
            return None;
        }

        println!("after update {:?}", self.state);
        // reconstruct current value
        let mut current = 0;
        for (idx, (state, mask)) in itertools::zip(&self.state, self.mask).enumerate() {
            match mask {
                MaskValueV2::PassThrough => {
                    let mask_value = self.address & (1 << idx);
                    if mask_value > 0 {
                        current += 1 << idx;
                    } else {
                        current = 2*current;
                    }
                }
                MaskValueV2::SetToOne => {
                    current += 1 << idx;
                }
                MaskValueV2::Floating => {
                    match state {
                        FloatingBitState::Nothing => break,
                        FloatingBitState::ZeroReturned => {
                        }
                        FloatingBitState::OneReturned => {
                            current += 1 << idx;
                        }
                    }
                }
            }
        }
        println!("output: {}", current);
        self.returned_once = true;
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::MaskValueV2;

    use super::AddressIterator;

    #[test]
    fn test_floating() {
        let mask = vec![MaskValueV2::Floating, MaskValueV2::Floating, MaskValueV2::Floating];
        let iterator = AddressIterator::new(&mask, &0);
        assert_eq!(vec![0, 1, 2, 3, 4, 5, 6, 7], iterator.collect::<Vec<u64>>());
    }

    #[test]
    fn test_set_to_one() {
        let mask = vec![MaskValueV2::SetToOne, MaskValueV2::Floating, MaskValueV2::Floating];
        let iterator = AddressIterator::new(&mask, &0);
        assert_eq!(vec![1, 3, 5, 7], iterator.collect::<Vec<u64>>());
    }
    #[test]
    fn test_passthrough() {
        let mask = vec![MaskValueV2::PassThrough, MaskValueV2::PassThrough, MaskValueV2::PassThrough];
        let iterator = AddressIterator::new(&mask, &6);
        assert_eq!(vec![6], iterator.collect::<Vec<u64>>());
    }
}