pub type WORD16 = core::ffi::c_short;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_word_struct {
    pub index: WORD32,
    pub len: WORD32,
    pub code_word: UWORD32,
}
#[no_mangle]
pub static mut ixheaacd_huff_book_scl: [ia_huff_code_word_struct; 121] = [
    {
        let mut init = ia_huff_code_word_struct {
            index: 60 as WORD32,
            len: 1 as WORD32,
            code_word: 0 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 59 as WORD32,
            len: 3 as WORD32,
            code_word: 4 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 61 as WORD32,
            len: 4 as WORD32,
            code_word: 10 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 58 as WORD32,
            len: 4 as WORD32,
            code_word: 11 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 62 as WORD32,
            len: 4 as WORD32,
            code_word: 12 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 57 as WORD32,
            len: 5 as WORD32,
            code_word: 26 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 63 as WORD32,
            len: 5 as WORD32,
            code_word: 27 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 56 as WORD32,
            len: 6 as WORD32,
            code_word: 56 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 64 as WORD32,
            len: 6 as WORD32,
            code_word: 57 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 55 as WORD32,
            len: 6 as WORD32,
            code_word: 58 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 65 as WORD32,
            len: 6 as WORD32,
            code_word: 59 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 66 as WORD32,
            len: 7 as WORD32,
            code_word: 120 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 54 as WORD32,
            len: 7 as WORD32,
            code_word: 121 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 67 as WORD32,
            len: 7 as WORD32,
            code_word: 122 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 53 as WORD32,
            len: 8 as WORD32,
            code_word: 246 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 68 as WORD32,
            len: 8 as WORD32,
            code_word: 247 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 52 as WORD32,
            len: 8 as WORD32,
            code_word: 248 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 69 as WORD32,
            len: 8 as WORD32,
            code_word: 249 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 51 as WORD32,
            len: 8 as WORD32,
            code_word: 250 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 70 as WORD32,
            len: 9 as WORD32,
            code_word: 502 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 50 as WORD32,
            len: 9 as WORD32,
            code_word: 503 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 49 as WORD32,
            len: 9 as WORD32,
            code_word: 504 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 71 as WORD32,
            len: 9 as WORD32,
            code_word: 505 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 72 as WORD32,
            len: 10 as WORD32,
            code_word: 1012 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 48 as WORD32,
            len: 10 as WORD32,
            code_word: 1013 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 73 as WORD32,
            len: 10 as WORD32,
            code_word: 1014 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 47 as WORD32,
            len: 10 as WORD32,
            code_word: 1015 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 74 as WORD32,
            len: 10 as WORD32,
            code_word: 1016 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 46 as WORD32,
            len: 10 as WORD32,
            code_word: 1017 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 76 as WORD32,
            len: 11 as WORD32,
            code_word: 2036 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 75 as WORD32,
            len: 11 as WORD32,
            code_word: 2037 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 77 as WORD32,
            len: 11 as WORD32,
            code_word: 2038 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 78 as WORD32,
            len: 11 as WORD32,
            code_word: 2039 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 45 as WORD32,
            len: 11 as WORD32,
            code_word: 2040 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 43 as WORD32,
            len: 11 as WORD32,
            code_word: 2041 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 44 as WORD32,
            len: 12 as WORD32,
            code_word: 4084 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 79 as WORD32,
            len: 12 as WORD32,
            code_word: 4085 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 42 as WORD32,
            len: 12 as WORD32,
            code_word: 4086 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 41 as WORD32,
            len: 12 as WORD32,
            code_word: 4087 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 80 as WORD32,
            len: 12 as WORD32,
            code_word: 4088 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 40 as WORD32,
            len: 12 as WORD32,
            code_word: 4089 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 81 as WORD32,
            len: 13 as WORD32,
            code_word: 8180 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 39 as WORD32,
            len: 13 as WORD32,
            code_word: 8181 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 82 as WORD32,
            len: 13 as WORD32,
            code_word: 8182 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 38 as WORD32,
            len: 13 as WORD32,
            code_word: 8183 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 83 as WORD32,
            len: 13 as WORD32,
            code_word: 8184 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 37 as WORD32,
            len: 14 as WORD32,
            code_word: 16370 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 35 as WORD32,
            len: 14 as WORD32,
            code_word: 16371 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 85 as WORD32,
            len: 14 as WORD32,
            code_word: 16372 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 33 as WORD32,
            len: 14 as WORD32,
            code_word: 16373 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 36 as WORD32,
            len: 14 as WORD32,
            code_word: 16374 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 34 as WORD32,
            len: 14 as WORD32,
            code_word: 16375 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 84 as WORD32,
            len: 14 as WORD32,
            code_word: 16376 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 32 as WORD32,
            len: 14 as WORD32,
            code_word: 16377 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 87 as WORD32,
            len: 15 as WORD32,
            code_word: 32756 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 89 as WORD32,
            len: 15 as WORD32,
            code_word: 32757 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 30 as WORD32,
            len: 15 as WORD32,
            code_word: 32758 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 31 as WORD32,
            len: 15 as WORD32,
            code_word: 32759 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 86 as WORD32,
            len: 16 as WORD32,
            code_word: 65520 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 29 as WORD32,
            len: 16 as WORD32,
            code_word: 65521 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 26 as WORD32,
            len: 16 as WORD32,
            code_word: 65522 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 27 as WORD32,
            len: 16 as WORD32,
            code_word: 65523 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 28 as WORD32,
            len: 16 as WORD32,
            code_word: 65524 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 24 as WORD32,
            len: 16 as WORD32,
            code_word: 65525 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 88 as WORD32,
            len: 16 as WORD32,
            code_word: 65526 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 25 as WORD32,
            len: 17 as WORD32,
            code_word: 131054 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 22 as WORD32,
            len: 17 as WORD32,
            code_word: 131055 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 23 as WORD32,
            len: 17 as WORD32,
            code_word: 131056 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 90 as WORD32,
            len: 18 as WORD32,
            code_word: 262114 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 21 as WORD32,
            len: 18 as WORD32,
            code_word: 262115 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 19 as WORD32,
            len: 18 as WORD32,
            code_word: 262116 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 3 as WORD32,
            len: 18 as WORD32,
            code_word: 262117 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 1 as WORD32,
            len: 18 as WORD32,
            code_word: 262118 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 2 as WORD32,
            len: 18 as WORD32,
            code_word: 262119 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 0 as WORD32,
            len: 18 as WORD32,
            code_word: 262120 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 98 as WORD32,
            len: 19 as WORD32,
            code_word: 524242 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 99 as WORD32,
            len: 19 as WORD32,
            code_word: 524243 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 100 as WORD32,
            len: 19 as WORD32,
            code_word: 524244 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 101 as WORD32,
            len: 19 as WORD32,
            code_word: 524245 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 102 as WORD32,
            len: 19 as WORD32,
            code_word: 524246 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 117 as WORD32,
            len: 19 as WORD32,
            code_word: 524247 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 97 as WORD32,
            len: 19 as WORD32,
            code_word: 524248 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 91 as WORD32,
            len: 19 as WORD32,
            code_word: 524249 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 92 as WORD32,
            len: 19 as WORD32,
            code_word: 524250 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 93 as WORD32,
            len: 19 as WORD32,
            code_word: 524251 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 94 as WORD32,
            len: 19 as WORD32,
            code_word: 524252 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 95 as WORD32,
            len: 19 as WORD32,
            code_word: 524253 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 96 as WORD32,
            len: 19 as WORD32,
            code_word: 524254 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 104 as WORD32,
            len: 19 as WORD32,
            code_word: 524255 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 111 as WORD32,
            len: 19 as WORD32,
            code_word: 524256 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 112 as WORD32,
            len: 19 as WORD32,
            code_word: 524257 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 113 as WORD32,
            len: 19 as WORD32,
            code_word: 524258 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 114 as WORD32,
            len: 19 as WORD32,
            code_word: 524259 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 115 as WORD32,
            len: 19 as WORD32,
            code_word: 524260 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 116 as WORD32,
            len: 19 as WORD32,
            code_word: 524261 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 110 as WORD32,
            len: 19 as WORD32,
            code_word: 524262 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 105 as WORD32,
            len: 19 as WORD32,
            code_word: 524263 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 106 as WORD32,
            len: 19 as WORD32,
            code_word: 524264 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 107 as WORD32,
            len: 19 as WORD32,
            code_word: 524265 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 108 as WORD32,
            len: 19 as WORD32,
            code_word: 524266 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 109 as WORD32,
            len: 19 as WORD32,
            code_word: 524267 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 118 as WORD32,
            len: 19 as WORD32,
            code_word: 524268 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 6 as WORD32,
            len: 19 as WORD32,
            code_word: 524269 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 8 as WORD32,
            len: 19 as WORD32,
            code_word: 524270 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 9 as WORD32,
            len: 19 as WORD32,
            code_word: 524271 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 10 as WORD32,
            len: 19 as WORD32,
            code_word: 524272 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 5 as WORD32,
            len: 19 as WORD32,
            code_word: 524273 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 103 as WORD32,
            len: 19 as WORD32,
            code_word: 524274 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 120 as WORD32,
            len: 19 as WORD32,
            code_word: 524275 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 119 as WORD32,
            len: 19 as WORD32,
            code_word: 524276 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 4 as WORD32,
            len: 19 as WORD32,
            code_word: 524277 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 7 as WORD32,
            len: 19 as WORD32,
            code_word: 524278 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 15 as WORD32,
            len: 19 as WORD32,
            code_word: 524279 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 16 as WORD32,
            len: 19 as WORD32,
            code_word: 524280 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 18 as WORD32,
            len: 19 as WORD32,
            code_word: 524281 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 20 as WORD32,
            len: 19 as WORD32,
            code_word: 524282 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 17 as WORD32,
            len: 19 as WORD32,
            code_word: 524283 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 11 as WORD32,
            len: 19 as WORD32,
            code_word: 524284 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 12 as WORD32,
            len: 19 as WORD32,
            code_word: 524285 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 14 as WORD32,
            len: 19 as WORD32,
            code_word: 524286 as UWORD32,
        };
        init
    },
    {
        let mut init = ia_huff_code_word_struct {
            index: 13 as WORD32,
            len: 19 as WORD32,
            code_word: 524287 as UWORD32,
        };
        init
    },
];
#[no_mangle]
pub static mut ixheaacd_book_scl_index: [WORD32; 33] = [
    0 as core::ffi::c_int,
    0x10100004 as core::ffi::c_int,
    0x2040000c as core::ffi::c_int,
    0xa0003b as core::ffi::c_int,
    0x30d0007a as core::ffi::c_int,
    0x412000fa as core::ffi::c_int,
    0x516001f9 as core::ffi::c_int,
    0x61c003f9 as core::ffi::c_int,
    0x722007f9 as core::ffi::c_int,
    0x82800ff9 as core::ffi::c_uint as WORD32,
    0x92d01ff8 as core::ffi::c_uint as WORD32,
    0xa3503ff9 as core::ffi::c_uint as WORD32,
    0xb400fff6 as core::ffi::c_uint as WORD32,
    0xc431fff0 as core::ffi::c_uint as WORD32,
    0x687ffef as core::ffi::c_int,
    0x707fff7 as core::ffi::c_int,
    0x747fffb as core::ffi::c_int,
    0x767fffd as core::ffi::c_int,
    0x777fffe as core::ffi::c_int,
    0x787ffff as core::ffi::c_int,
    0x30000b as core::ffi::c_int,
    0x60001b as core::ffi::c_int,
    0xf000f7 as core::ffi::c_int,
    0x14001f7 as core::ffi::c_int,
    0x1a003f7 as core::ffi::c_int,
    0x20007f7 as core::ffi::c_int,
    0x2600ff7 as core::ffi::c_int,
    0x2c01ff7 as core::ffi::c_int,
    0x3303ff7 as core::ffi::c_int,
    0x3907ff7 as core::ffi::c_int,
    0x421ffef as core::ffi::c_int,
    0x14a3ffe8 as core::ffi::c_int,
    0x587ffdf as core::ffi::c_int,
];
#[no_mangle]
pub static mut ixheaacd_book_scl_code_book: [WORD16; 122] = [
    0x13 as core::ffi::c_int as WORD16,
    0x781 as core::ffi::c_int as WORD16,
    0x763 as core::ffi::c_int as WORD16,
    0x7a4 as core::ffi::c_int as WORD16,
    0x744 as core::ffi::c_int as WORD16,
    0x7c4 as core::ffi::c_int as WORD16,
    0x725 as core::ffi::c_int as WORD16,
    0x7e5 as core::ffi::c_int as WORD16,
    0x706 as core::ffi::c_int as WORD16,
    0x806 as core::ffi::c_int as WORD16,
    0x6e6 as core::ffi::c_int as WORD16,
    0x826 as core::ffi::c_int as WORD16,
    0x847 as core::ffi::c_int as WORD16,
    0x6c7 as core::ffi::c_int as WORD16,
    0x867 as core::ffi::c_int as WORD16,
    0x6a8 as core::ffi::c_int as WORD16,
    0x888 as core::ffi::c_int as WORD16,
    0x688 as core::ffi::c_int as WORD16,
    0x8a8 as core::ffi::c_int as WORD16,
    0x668 as core::ffi::c_int as WORD16,
    0x8c9 as core::ffi::c_int as WORD16,
    0x649 as core::ffi::c_int as WORD16,
    0x629 as core::ffi::c_int as WORD16,
    0x8e9 as core::ffi::c_int as WORD16,
    0x90a as core::ffi::c_int as WORD16,
    0x60a as core::ffi::c_int as WORD16,
    0x92a as core::ffi::c_int as WORD16,
    0x5ea as core::ffi::c_int as WORD16,
    0x94a as core::ffi::c_int as WORD16,
    0x5ca as core::ffi::c_int as WORD16,
    0x98b as core::ffi::c_int as WORD16,
    0x96b as core::ffi::c_int as WORD16,
    0x9ab as core::ffi::c_int as WORD16,
    0x9cb as core::ffi::c_int as WORD16,
    0x5ab as core::ffi::c_int as WORD16,
    0x56b as core::ffi::c_int as WORD16,
    0x58c as core::ffi::c_int as WORD16,
    0x9ec as core::ffi::c_int as WORD16,
    0x54c as core::ffi::c_int as WORD16,
    0x52c as core::ffi::c_int as WORD16,
    0xa0c as core::ffi::c_int as WORD16,
    0x50c as core::ffi::c_int as WORD16,
    0xa2d as core::ffi::c_int as WORD16,
    0x4ed as core::ffi::c_int as WORD16,
    0xa4d as core::ffi::c_int as WORD16,
    0x4cd as core::ffi::c_int as WORD16,
    0xa6d as core::ffi::c_int as WORD16,
    0x4ae as core::ffi::c_int as WORD16,
    0x46e as core::ffi::c_int as WORD16,
    0xaae as core::ffi::c_int as WORD16,
    0x42e as core::ffi::c_int as WORD16,
    0x48e as core::ffi::c_int as WORD16,
    0x44e as core::ffi::c_int as WORD16,
    0xa8e as core::ffi::c_int as WORD16,
    0x40e as core::ffi::c_int as WORD16,
    0xaef as core::ffi::c_int as WORD16,
    0xb2f as core::ffi::c_int as WORD16,
    0x3cf as core::ffi::c_int as WORD16,
    0x3ef as core::ffi::c_int as WORD16,
    0xad0 as core::ffi::c_int as WORD16,
    0x3b0 as core::ffi::c_int as WORD16,
    0x350 as core::ffi::c_int as WORD16,
    0x370 as core::ffi::c_int as WORD16,
    0x390 as core::ffi::c_int as WORD16,
    0x310 as core::ffi::c_int as WORD16,
    0xb10 as core::ffi::c_int as WORD16,
    0x331 as core::ffi::c_int as WORD16,
    0x2d1 as core::ffi::c_int as WORD16,
    0x2f1 as core::ffi::c_int as WORD16,
    0xb52 as core::ffi::c_int as WORD16,
    0x2b2 as core::ffi::c_int as WORD16,
    0x272 as core::ffi::c_int as WORD16,
    0x72 as core::ffi::c_int as WORD16,
    0x32 as core::ffi::c_int as WORD16,
    0x52 as core::ffi::c_int as WORD16,
    0x12 as core::ffi::c_int as WORD16,
    0xc53 as core::ffi::c_int as WORD16,
    0xc73 as core::ffi::c_int as WORD16,
    0xc93 as core::ffi::c_int as WORD16,
    0xcb3 as core::ffi::c_int as WORD16,
    0xcd3 as core::ffi::c_int as WORD16,
    0xeb3 as core::ffi::c_int as WORD16,
    0xc33 as core::ffi::c_int as WORD16,
    0xb73 as core::ffi::c_int as WORD16,
    0xb93 as core::ffi::c_int as WORD16,
    0xbb3 as core::ffi::c_int as WORD16,
    0xbd3 as core::ffi::c_int as WORD16,
    0xbf3 as core::ffi::c_int as WORD16,
    0xc13 as core::ffi::c_int as WORD16,
    0xd13 as core::ffi::c_int as WORD16,
    0xdf3 as core::ffi::c_int as WORD16,
    0xe13 as core::ffi::c_int as WORD16,
    0xe33 as core::ffi::c_int as WORD16,
    0xe53 as core::ffi::c_int as WORD16,
    0xe73 as core::ffi::c_int as WORD16,
    0xe93 as core::ffi::c_int as WORD16,
    0xdd3 as core::ffi::c_int as WORD16,
    0xd33 as core::ffi::c_int as WORD16,
    0xd53 as core::ffi::c_int as WORD16,
    0xd73 as core::ffi::c_int as WORD16,
    0xd93 as core::ffi::c_int as WORD16,
    0xdb3 as core::ffi::c_int as WORD16,
    0xed3 as core::ffi::c_int as WORD16,
    0xd3 as core::ffi::c_int as WORD16,
    0x113 as core::ffi::c_int as WORD16,
    0x133 as core::ffi::c_int as WORD16,
    0x153 as core::ffi::c_int as WORD16,
    0xb3 as core::ffi::c_int as WORD16,
    0xcf3 as core::ffi::c_int as WORD16,
    0xf13 as core::ffi::c_int as WORD16,
    0xef3 as core::ffi::c_int as WORD16,
    0x93 as core::ffi::c_int as WORD16,
    0xf3 as core::ffi::c_int as WORD16,
    0x1f3 as core::ffi::c_int as WORD16,
    0x213 as core::ffi::c_int as WORD16,
    0x253 as core::ffi::c_int as WORD16,
    0x293 as core::ffi::c_int as WORD16,
    0x233 as core::ffi::c_int as WORD16,
    0x173 as core::ffi::c_int as WORD16,
    0x193 as core::ffi::c_int as WORD16,
    0x1d3 as core::ffi::c_int as WORD16,
    0x1b3 as core::ffi::c_int as WORD16,
];
