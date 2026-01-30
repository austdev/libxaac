extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    static ixheaacd_factorial_7: [WORD32; 8];
    static ixheaacd_iso_code_index_table: [WORD32; 37];
    static ixheaacd_iso_code_data_table: [UWORD8; 226];
    static ixheaacd_signed_leader_is: [UWORD32; 226];
    static ixheaacd_iso_code_num_table: [WORD32; 0];
    static ixheaacd_pos_abs_leaders_a3: [WORD32; 0];
    static ixheaacd_pos_abs_leaders_a4: [WORD32; 0];
    static ixheaacd_absolute_leader_tab_da: [[UWORD8; 8]; 0];
    static ixheaacd_cardinality_offset_table_i3: [UWORD32; 0];
    static ixheaacd_cardinality_offset_tab_i4: [UWORD32; 0];
}
pub type size_t = usize;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type VOID = ();
pub type WORD = core::ffi::c_int;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
#[inline]
unsafe extern "C" fn ixheaac_shl32_sat(mut a: WORD32, mut b: WORD) -> WORD32 {
    let mut out_val: WORD32 = 0;
    if a > MAX_32 >> b {
        out_val = MAX_32;
    } else if a < MIN_32 >> b {
        out_val = MIN_32;
    } else {
        out_val = a << b;
    }
    return out_val;
}
#[inline]
unsafe extern "C" fn ixheaac_add32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut sum: WORD64 = 0;
    sum = a as WORD64 + b as WORD64;
    if sum >= MAX_32 as WORD64 {
        return MAX_32;
    }
    if sum <= MIN_32 as WORD64 {
        return MIN_32;
    }
    return sum as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaac_sub32_sat(mut a: WORD32, mut b: WORD32) -> WORD32 {
    let mut diff: WORD64 = 0;
    diff = a as WORD64 - b as WORD64;
    if diff >= MAX_32 as WORD64 {
        return MAX_32;
    }
    if diff <= MIN_32 as WORD64 {
        return MIN_32;
    }
    return diff as WORD32;
}
#[inline]
unsafe extern "C" fn ixheaac_negate32_sat(mut a: WORD32) -> WORD32 {
    let mut neg_val: WORD32 = 0;
    if a == MIN_32 {
        neg_val = MAX_32;
    } else {
        neg_val = -a;
    }
    return neg_val;
}
#[inline]
unsafe extern "C" fn ixheaac_sat64_32(mut a: WORD64) -> WORD32 {
    let mut result: WORD32 = 0;
    if a >= MAX_32 as WORD64 {
        result = MAX_32;
    } else if a <= MIN_32 as WORD64 {
        result = MIN_32;
    } else {
        result = a as WORD32;
    }
    return result;
}
pub const LEN_I3: core::ffi::c_int = 9 as core::ffi::c_int;
pub const LEN_I4: core::ffi::c_int = 28 as core::ffi::c_int;
unsafe extern "C" fn ixheaacd_nearest_neighbor_2d(
    mut x: *mut WORD32,
    mut y: *mut WORD32,
    mut count: WORD32,
    mut rem: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut sum: WORD32 = 0;
    let mut s: WORD32 = 0;
    let mut e: [WORD32; 8] = [0; 8];
    let mut em: WORD32 = 0;
    let mut rem_temp: [WORD32; 8] = [0; 8];
    memcpy(
        rem_temp.as_mut_ptr() as *mut core::ffi::c_void,
        rem as *const core::ffi::c_void,
        (8 as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    sum = 0 as core::ffi::c_int as WORD32;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 8 as core::ffi::c_int {
        if *x.offset(i as isize) < 0 as core::ffi::c_int {
            *y.offset(i as isize) = ixheaac_negate32_sat(
                ixheaac_shl32_sat(
                    ixheaac_sub32_sat(1 as WORD32, *x.offset(i as isize))
                        >> 1 as core::ffi::c_int,
                    1 as WORD,
                ),
            );
        } else {
            *y.offset(i as isize) = ixheaac_shl32_sat(
                ixheaac_add32_sat(1 as WORD32, *x.offset(i as isize))
                    >> 1 as core::ffi::c_int,
                1 as WORD,
            );
        }
        sum = ixheaac_add32_sat(sum, *y.offset(i as isize));
        if *x.offset(i as isize) % 2 as core::ffi::c_int != 0 as core::ffi::c_int {
            if *x.offset(i as isize) < 0 as core::ffi::c_int {
                rem_temp[i as usize] = ixheaac_negate32_sat(
                    ixheaac_sub32_sat(rem_temp[i as usize], (1 as WORD32) << count),
                );
            } else {
                rem_temp[i as usize] = ixheaac_sub32_sat(
                    rem_temp[i as usize],
                    (1 as WORD32) << count,
                );
            }
        }
        i += 1;
    }
    if sum as core::ffi::c_int % 4 as core::ffi::c_int != 0 {
        em = 0 as core::ffi::c_int as WORD32;
        j = 0 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            e[i as usize] = rem_temp[i as usize];
            i += 1;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            if e[i as usize] < 0 as core::ffi::c_int {
                s = -e[i as usize];
            } else {
                s = e[i as usize];
            }
            if em < s {
                em = s;
                j = i;
            }
            i += 1;
        }
        if e[j as usize] < 0 as core::ffi::c_int {
            let ref mut fresh1 = *y.offset(j as isize);
            *fresh1 -= 2 as core::ffi::c_int;
            rem_temp[j as usize] = ixheaac_add32_sat(
                rem_temp[j as usize],
                (2 as WORD32) << count,
            );
        } else {
            *y.offset(j as isize) = ixheaac_add32_sat(
                *y.offset(j as isize),
                2 as WORD32,
            );
            rem_temp[j as usize] = ixheaac_sub32_sat(
                rem_temp[j as usize],
                (2 as WORD32) << count,
            );
        }
    }
    memcpy(
        rem as *mut core::ffi::c_void,
        rem_temp.as_mut_ptr() as *const core::ffi::c_void,
        (8 as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_voronoi_search(
    mut x: *mut WORD32,
    mut y: *mut WORD32,
    mut count: WORD32,
    mut rem1: *mut WORD32,
    mut rem2: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut y0: [WORD32; 8] = [0; 8];
    let mut y1: [WORD32; 8] = [0; 8];
    let mut x1: [WORD32; 8] = [0; 8];
    let mut tmp: WORD32 = 0;
    let mut e0: WORD32 = 0;
    let mut e1: WORD32 = 0;
    ixheaacd_nearest_neighbor_2d(x, y0.as_mut_ptr(), count, rem1);
    i = 0 as core::ffi::c_int as WORD32;
    while i < 8 as core::ffi::c_int {
        if *x.offset(i as isize) == 0 as core::ffi::c_int {
            if *rem2.offset(i as isize) == 0 as core::ffi::c_int {
                x1[i as usize] = (*x.offset(i as isize) - 1 as core::ffi::c_int)
                    as WORD32;
            } else {
                x1[i as usize] = 0 as core::ffi::c_int as WORD32;
                *rem2.offset(i as isize) = ixheaac_sub32_sat(
                    *rem2.offset(i as isize),
                    (1 as WORD32) << count,
                );
            }
        } else {
            x1[i as usize] = ixheaac_sub32_sat(*x.offset(i as isize), 1 as WORD32);
        }
        i += 1;
    }
    ixheaacd_nearest_neighbor_2d(x1.as_mut_ptr(), y1.as_mut_ptr(), count, rem2);
    i = 0 as core::ffi::c_int as WORD32;
    while i < 8 as core::ffi::c_int {
        y1[i as usize] = ixheaac_add32_sat(y1[i as usize], 1 as WORD32);
        i += 1;
    }
    e1 = 0 as core::ffi::c_int as WORD32;
    e0 = e1;
    i = 0 as core::ffi::c_int as WORD32;
    while i < 8 as core::ffi::c_int {
        tmp = *rem1.offset(i as isize);
        e0 = ixheaac_add32_sat(ixheaac_sat64_32(tmp as WORD64 * tmp as WORD64), e0);
        tmp = *rem2.offset(i as isize);
        e1 = ixheaac_add32_sat(ixheaac_sat64_32(tmp as WORD64 * tmp as WORD64), e1);
        i += 1;
    }
    if e0 < e1 {
        i = 0 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            *y.offset(i as isize) = y0[i as usize];
            i += 1;
        }
    } else {
        i = 0 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            *y.offset(i as isize) = y1[i as usize];
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_voronoi_idx_dec(
    mut kv: *mut WORD32,
    mut m: WORD32,
    mut y: *mut WORD32,
    mut count: WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut v: [WORD32; 8] = [0; 8];
    let mut tmp: WORD32 = 0;
    let mut sum: WORD32 = 0;
    let mut ptr1: *mut WORD32 = 0 as *mut WORD32;
    let mut ptr2: *mut WORD32 = 0 as *mut WORD32;
    let mut z: [WORD32; 8] = [0; 8];
    let mut rem1: [WORD32; 8] = [0; 8];
    let mut rem2: [WORD32; 8] = [0; 8];
    i = 0 as core::ffi::c_int as WORD32;
    while i < 8 as core::ffi::c_int {
        *y.offset(i as isize) = *kv.offset(7 as core::ffi::c_int as isize);
        i += 1;
    }
    z[7 as core::ffi::c_int as usize] = *y.offset(7 as core::ffi::c_int as isize)
        >> count;
    rem1[7 as core::ffi::c_int as usize] = (*y.offset(7 as core::ffi::c_int as isize)
        & m as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
    sum = 0 as core::ffi::c_int as WORD32;
    i = 6 as core::ffi::c_int as WORD32;
    while i >= 1 as core::ffi::c_int {
        tmp = ixheaac_shl32_sat(*kv.offset(i as isize), 1 as WORD);
        sum = ixheaac_add32_sat(sum, tmp);
        *y.offset(i as isize) = ixheaac_add32_sat(*y.offset(i as isize), tmp);
        z[i as usize] = *y.offset(i as isize) >> count;
        rem1[i as usize] = (*y.offset(i as isize)
            & m as core::ffi::c_int - 1 as core::ffi::c_int) as WORD32;
        i -= 1;
    }
    *y.offset(0 as core::ffi::c_int as isize) = ixheaac_add32_sat(
        *y.offset(0 as core::ffi::c_int as isize),
        ixheaac_add32_sat(
            ixheaac_sat64_32(
                4 as core::ffi::c_int as WORD64
                    * *kv.offset(0 as core::ffi::c_int as isize) as WORD64,
            ),
            sum,
        ),
    );
    z[0 as core::ffi::c_int as usize] = ixheaac_sub32_sat(
        *y.offset(0 as core::ffi::c_int as isize),
        2 as WORD32,
    ) >> count;
    if m != 0 as core::ffi::c_int {
        rem1[0 as core::ffi::c_int as usize] = ixheaac_sub32_sat(
            *y.offset(0 as core::ffi::c_int as isize),
            2 as WORD32,
        ) % m;
    } else {
        rem1[0 as core::ffi::c_int as usize] = ixheaac_sub32_sat(
            *y.offset(0 as core::ffi::c_int as isize),
            2 as WORD32,
        );
    }
    memcpy(
        rem2.as_mut_ptr() as *mut core::ffi::c_void,
        rem1.as_mut_ptr() as *const core::ffi::c_void,
        (8 as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    ixheaacd_voronoi_search(
        z.as_mut_ptr(),
        v.as_mut_ptr(),
        count,
        rem1.as_mut_ptr(),
        rem2.as_mut_ptr(),
    );
    ptr1 = y;
    ptr2 = v.as_mut_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < 8 as core::ffi::c_int {
        let fresh0 = ptr2;
        ptr2 = ptr2.offset(1);
        *ptr1 = ixheaac_sub32_sat(
            *ptr1,
            ixheaac_sat64_32(m as WORD64 * *fresh0 as WORD64),
        );
        ptr1 = ptr1.offset(1);
        i += 1;
    }
}
unsafe extern "C" fn ixheaacd_gosset_rank_of_permutation(
    mut rank: WORD32,
    mut xs: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut a: [WORD32; 8] = [0; 8];
    let mut w: [WORD32; 8] = [0; 8];
    let mut base: WORD32 = 0;
    let mut fac: WORD32 = 0;
    let mut fac_b: WORD32 = 0;
    let mut target: WORD32 = 0;
    j = 0 as core::ffi::c_int as WORD32;
    w[j as usize] = 1 as core::ffi::c_int as WORD32;
    a[j as usize] = *xs.offset(0 as core::ffi::c_int as isize);
    base = 1 as core::ffi::c_int as WORD32;
    i = 1 as core::ffi::c_int as WORD32;
    while i < 8 as core::ffi::c_int {
        if *xs.offset(i as isize)
            != *xs.offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
        {
            j += 1;
            w[j as usize] = 1 as core::ffi::c_int as WORD32;
            a[j as usize] = *xs.offset(i as isize);
        } else {
            w[j as usize] += 1;
            base *= w[j as usize];
        }
        i += 1;
    }
    if w[0 as core::ffi::c_int as usize] == 8 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            *xs.offset(i as isize) = a[0 as core::ffi::c_int as usize];
            i += 1;
        }
    } else {
        target = rank * base;
        fac_b = 1 as core::ffi::c_int as WORD32;
        i = 0 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            fac = fac_b * ixheaacd_factorial_7[i as usize];
            j = -(1 as core::ffi::c_int) as WORD32;
            loop {
                j += 1;
                target -= w[j as usize] * fac;
                if !(target >= 0 as core::ffi::c_int) {
                    break;
                }
            }
            *xs.offset(i as isize) = a[j as usize];
            target += w[j as usize] * fac;
            fac_b *= w[j as usize];
            w[j as usize] -= 1;
            i += 1;
        }
    };
}
unsafe extern "C" fn ixheaacd_get_abs_leader_tbl(
    mut table: *const UWORD32,
    mut code_book_ind: UWORD32,
    mut size: WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    i = 4 as core::ffi::c_int as WORD32;
    while i < size {
        if code_book_ind < *table.offset(i as isize) {
            break;
        }
        i += 4 as core::ffi::c_int;
    }
    if i > size {
        i = size;
    }
    if code_book_ind
        < *table.offset((i as core::ffi::c_int - 2 as core::ffi::c_int) as isize)
    {
        i -= 2 as core::ffi::c_int;
    }
    if code_book_ind
        < *table.offset((i as core::ffi::c_int - 1 as core::ffi::c_int) as isize)
    {
        i -= 1;
    }
    i -= 1;
    return i;
}
unsafe extern "C" fn ixheaacd_gosset_decode_base_index(
    mut n: WORD32,
    mut code_book_ind: UWORD32,
    mut ya: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut im: WORD32 = 0;
    let mut t: WORD32 = 0;
    let mut sign_code: WORD32 = 0;
    let mut idx: WORD32 = 0 as WORD32;
    let mut ks: WORD32 = 0;
    let mut rank: WORD32 = 0;
    if n < 2 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            *ya.offset(i as isize) = 0 as core::ffi::c_int as WORD32;
            i += 1;
        }
    } else {
        match n {
            2 | 3 => {
                i = ixheaacd_get_abs_leader_tbl(
                    ixheaacd_cardinality_offset_table_i3.as_ptr(),
                    code_book_ind,
                    LEN_I3,
                );
                idx = *ixheaacd_pos_abs_leaders_a3.as_ptr().offset(i as isize);
            }
            4 => {
                i = ixheaacd_get_abs_leader_tbl(
                    ixheaacd_cardinality_offset_tab_i4.as_ptr(),
                    code_book_ind,
                    LEN_I4,
                );
                idx = *ixheaacd_pos_abs_leaders_a4.as_ptr().offset(i as isize);
            }
            _ => {}
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            *ya.offset(i as isize) = (*ixheaacd_absolute_leader_tab_da
                .as_ptr()
                .offset(idx as isize))[i as usize] as WORD32;
            i += 1;
        }
        t = ixheaacd_iso_code_index_table[idx as usize];
        im = *ixheaacd_iso_code_num_table.as_ptr().offset(idx as isize);
        ks = ixheaacd_get_abs_leader_tbl(
            ixheaacd_signed_leader_is.as_ptr().offset(t as isize),
            code_book_ind,
            im,
        );
        sign_code = (2 as core::ffi::c_int
            * ixheaacd_iso_code_data_table[(t + ks) as usize] as core::ffi::c_int)
            as WORD32;
        i = 7 as core::ffi::c_int as WORD32;
        while i >= 0 as core::ffi::c_int {
            let ref mut fresh2 = *ya.offset(i as isize);
            *fresh2
                *= 1 as core::ffi::c_int
                    - (sign_code as core::ffi::c_int & 2 as core::ffi::c_int);
            sign_code >>= 1 as core::ffi::c_int;
            i -= 1;
        }
        rank = code_book_ind.wrapping_sub(ixheaacd_signed_leader_is[(t + ks) as usize])
            as WORD32;
        ixheaacd_gosset_rank_of_permutation(rank, ya);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_rotated_gosset_mtx_dec(
    mut qn: WORD32,
    mut code_book_idx: WORD32,
    mut kv: *mut WORD32,
    mut b: *mut WORD32,
) -> VOID {
    if qn <= 4 as core::ffi::c_int {
        ixheaacd_gosset_decode_base_index(qn, code_book_idx as UWORD32, b);
    } else {
        let mut i: WORD32 = 0;
        let mut m: WORD32 = 0;
        let mut c: [WORD32; 8] = [0; 8];
        let mut count: WORD32 = 0 as WORD32;
        while qn > 4 as core::ffi::c_int {
            count += 1;
            qn -= 2 as core::ffi::c_int;
        }
        if count >= 31 as core::ffi::c_int {
            m = MAX_32;
        } else {
            m = ((1 as core::ffi::c_int) << count) as WORD32;
        }
        ixheaacd_gosset_decode_base_index(qn, code_book_idx as UWORD32, b);
        ixheaacd_voronoi_idx_dec(kv, m, c.as_mut_ptr(), count);
        i = 0 as core::ffi::c_int as WORD32;
        while i < 8 as core::ffi::c_int {
            *b.offset(i as isize) = ixheaac_add32_sat(
                ixheaac_sat64_32(m as WORD64 * *b.offset(i as isize) as WORD64),
                c[i as usize],
            );
            i += 1;
        }
    };
}
