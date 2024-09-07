const MAIN_PIC_DATA_PORT: u16 = 0x00A0;
const SECONDARY_PIC_DATA_PORT: u16 = 0x00A1;
pub fn disable() {
    unsafe {
        super::ports::write(MAIN_PIC_DATA_PORT, 0xFF);
        super::ports::write(SECONDARY_PIC_DATA_PORT, 0xFF);
    }
}
