use std::os::windows::ffi::OsStrExt;

#[allow(non_camel_case_types)]
type BCRYPT_ALG_HANDLE = usize;
#[allow(non_camel_case_types)]
type wchar_t = u16;
type WCHAR = wchar_t;
type LPCWSTR = *const WCHAR;
#[allow(non_camel_case_types)]
type c_uchar = u8;
type UCHAR = c_uchar;
type PUCHAR = *mut UCHAR;
#[allow(non_camel_case_types)]
type c_ulong = u32;
type ULONG = c_ulong;
#[allow(non_camel_case_types)]
type c_long = i32;
type LONG = c_long;
type NTSTATUS = LONG;
#[link(name = "Bcrypt")]
extern "C" {
    fn BCryptOpenAlgorithmProvider(
        _: *mut BCRYPT_ALG_HANDLE,
        _: LPCWSTR,
        _: LPCWSTR,
        _: ULONG,
    ) -> NTSTATUS;
    fn BCryptGenRandom(_: BCRYPT_ALG_HANDLE, _: PUCHAR, _: ULONG, _: ULONG) -> NTSTATUS;
    fn BCryptCloseAlgorithmProvider(_: BCRYPT_ALG_HANDLE, _: ULONG) -> NTSTATUS;
}
const STATUS_SUCCESS: NTSTATUS = 0;

const RAND_BUF_SIZE: usize = 1024 * 1024;
pub struct RandState {
    handle: BCRYPT_ALG_HANDLE,
    buf: Box<[u8; RAND_BUF_SIZE]>,
    pos: usize,
}
impl RandState {
    fn reload_buf(&mut self) {
        let ret =
            unsafe { BCryptGenRandom(self.handle, self.buf.as_mut_ptr(), self.buf.len() as _, 0) };
        assert!(ret == STATUS_SUCCESS);
        self.pos = 0;
    }
    pub fn new() -> Self {
        let alg_name = std::ffi::OsStr::new("RNG")
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        let mut alg = std::mem::MaybeUninit::<BCRYPT_ALG_HANDLE>::uninit();
        let ret: NTSTATUS = unsafe {
            BCryptOpenAlgorithmProvider(alg.as_mut_ptr(), alg_name.as_ptr(), std::ptr::null(), 0)
        };
        assert!(ret == STATUS_SUCCESS);
        let alg = unsafe { alg.assume_init() };

        use std::convert::TryInto;
        let buf: Box<[u8; RAND_BUF_SIZE]> = vec![0; RAND_BUF_SIZE]
            .into_boxed_slice()
            .try_into()
            .unwrap();
        let mut ret = Self {
            buf,
            pos: 0,
            handle: alg,
        };
        ret.reload_buf();
        ret
    }

    fn gen_random_bytes(&mut self, buf: &mut [u8]) {
        let to_copy = buf.len();
        let remaining = self.buf.len() - self.pos;
        if to_copy <= remaining {
            buf.copy_from_slice(&self.buf[self.pos..(self.pos + to_copy)]);
            self.pos += to_copy;
        } else {
            (&mut buf[..remaining]).copy_from_slice(&self.buf[self.pos..]);
            self.reload_buf();
            self.gen_random_bytes(&mut buf[remaining..]);
        }
    }

    pub fn random_double(&mut self) -> f64 {
        // Gen random bytes
        let mut random_bytes = [0u8; 8];
        self.gen_random_bytes(&mut random_bytes[..]);

        let float_size = std::mem::size_of::<f64>() as u32 * 8;
        const FRACTION_BITS: u32 = 52;
        const EXPONENT_BIAS: u32 = 1023;

        // Get the mantissa (52 bits)
        let value = u64::from_ne_bytes(random_bytes);
        let fraction = value >> (float_size - FRACTION_BITS);

        // Get a 0 exponent
        let exponent = 0;
        let exponent_bits = ((EXPONENT_BIAS + exponent) as u64) << FRACTION_BITS;

        // Combine them and floor so it's never 1
        f64::from_bits(fraction | exponent_bits) - (1.0 - f64::EPSILON / 2.0)
    }
    pub fn random_double_range(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.random_double()
    }
}

impl Drop for RandState {
    fn drop(&mut self) {
        let ret = unsafe { BCryptCloseAlgorithmProvider(self.handle, 0) };
        assert!(ret == STATUS_SUCCESS);
    }
}
