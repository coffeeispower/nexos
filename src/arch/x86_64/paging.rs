/// Calculates the correct indices in the page table for a given virtual memory address
/// For example if you want to map 0x1300-0x2300 to 0x4000-5000, you would need to put the address 0x1300 into the following indices: 3 - 0 - 0 - 0
/// So this function calculates those indices in the page table
pub fn get_page_index<T>(virtual_address: *mut T) -> [usize; 4] {
    let virtual_address = virtual_address as usize;
    [
        (virtual_address >> 12) & 0x1ff,
        (virtual_address >> (12 + 9)) & 0x1ff,
        (virtual_address >> (12 + 9 + 9)) & 0x1ff,
        (virtual_address >> (12 + 9 + 9 + 9)) & 0x1ff,
    ]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_index_test() {
        assert_eq!(get_page_index(0x4000 as *mut ()), [4, 0, 0, 0]);
        assert_eq!(
            get_page_index((0x1000 * 52 + 0x50000 * 7) as *mut ()),
            [100, 1, 0, 0]
        );
    }
}
