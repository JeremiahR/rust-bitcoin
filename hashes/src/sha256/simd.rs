// copied from chacha20_poly1305/src/chacha20.rs
// before committing we should find a better place for this code
#[repr(align(16))]
#[derive(Clone, Copy, PartialEq)]
struct U32x4([u32; 4]);

impl U32x4 {
    #[inline(always)]
    fn wrapping_add(self, rhs: Self) -> Self {
        let mut result = [0u32; 4];
        (0..4).for_each(|i| {
            result[i] = self.0[i].wrapping_add(rhs.0[i]);
        });
        U32x4(result)
    }

    #[inline(always)]
    fn rotate_left(self, n: u32) -> Self {
        let mut result = [0u32; 4];
        (0..4).for_each(|i| {
            result[i] = self.0[i].rotate_left(n);
        });
        U32x4(result)
    }

    #[inline(always)]
    fn rotate_elements_left<const N: u32>(self) -> Self {
        let mut result = [0u32; 4];
        (0..4).for_each(|i| {
            result[i] = self.0[(i + N as usize) % 4];
        });
        U32x4(result)
    }

    #[inline(always)]
    fn rotate_elements_right<const N: u32>(self) -> Self {
        let mut result = [0u32; 4];
        (0..4).for_each(|i| {
            result[i] = self.0[(i + 4 - N as usize) % 4];
        });
        U32x4(result)
    }

    #[inline(always)]
    fn to_le_bytes(self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        (0..4).for_each(|i| {
            bytes[i * 4..(i + 1) * 4].copy_from_slice(&self.0[i].to_le_bytes());
        });
        bytes
    }
}

impl BitXor for U32x4 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        let mut result = [0u32; 4];
        (0..4).for_each(|i| {
            result[i] = self.0[i] ^ rhs.0[i];
        });
        U32x4(result)
    }
}
