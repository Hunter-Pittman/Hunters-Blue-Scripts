// Name: windows_utf16_convert.rs
// Module Description: Alot of windows programs output data in utf_16 byte arrays dealing with big endian/little endian, this is the function to convert
pub fn parse_utf16_bytes(bytes: &[u8]) -> Option<String> {
    let mut chunks = bytes.chunks_exact(2);
    let is_big_endian = match chunks.next() {
        Some(&[254, 255]) => true,
        Some(&[255, 254]) => false,
        _ => return None,
    };
    let utf16: Vec<_> = chunks
        .map(|x| {
            let arr2 = x.try_into().expect("convert .chunks_exact() to [u8; 2]");
            if is_big_endian {
                u16::from_be_bytes(arr2)
            } else {
                u16::from_le_bytes(arr2)
            }
        })
        .collect();
    String::from_utf16(&utf16).ok()
}