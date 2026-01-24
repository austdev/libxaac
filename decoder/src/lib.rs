
pub mod ixheaacd;

// next mod is needed for migration process only 
mod gen_ixheaacd_ref;


#[cfg(all(feature = "fallback", feature = "integration"))]
compile_error!("features \"fallback\" and \"intergation\" cannot be enabled at the same time");

#[cfg(feature = "integration")]
mod integration_test {

    use crate::gen_ixheaacd_ref::*;
    use crate::ixheaacd::OffsetLengths;
    use crate::ixheaacd::WindowType;
    use crate::ixheaacd;
    use std::slice;

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_lpd_bpf_fix(
        usac_data: *mut ia_usac_data_struct,
        is_short: WORD32,
        out_buffer: *mut FLOAT32,
        st: ia_usac_lpd_decoder_handle,
    ) -> WORD32 {
        if usac_data.is_null() || out_buffer.is_null() || st.is_null() {
            return -1;
        }
        unsafe {
            let lpd_sbf_len = (NUM_FRAMES * (*usac_data).num_subfrm as u32) / 2;
            let synth_delay = (lpd_sbf_len - 1) * LEN_SUBFR;
            let buffer_slice = slice::from_raw_parts_mut(
                out_buffer, 
                (synth_delay + (*usac_data).ccfl as u32 + LEN_SUBFR) as usize,
            );
            let res = ixheaacd::lpd_bpf_fix(
                &(*usac_data),
                is_short != 0,
                buffer_slice,
                &mut (*st),
            );
            if res.is_ok() { 0 } else { -1 }
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_calc_window(
        win: *mut *mut WORD32,
        len: WORD32,
        wfun_select: WORD32,
        ec_flag: WORD32,
    ) -> WORD32 {
        if !win.is_null() {
            let win_type = match wfun_select {
                0 => WindowType::Sine,
                1 => WindowType::Kbd,
                _ => { return -1; }
            };
            #[allow(invalid_reference_casting)]
            unsafe {
                if let Some(result) = ixheaacd::calc_window(len, win_type, ec_flag != 0) {
                    *win = result.as_ptr() as *mut WORD32;
                    return 0;
                }
            }
        }
        -1
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_combine_fac(
            src1: *mut WORD32,
            src2: *mut WORD32,
            dest: *mut WORD32,
            len: WORD32,
            shift1: WORD8,
            shift2: WORD8,
        ) {
        if src1.is_null() || src2.is_null() || dest.is_null() || len < 0 {
            return;
        }
        let len = len as usize;
        unsafe {
            let src1_slice = slice::from_raw_parts(src1, len);
            let src2_slice = slice::from_raw_parts(src2, len);
            let dest_slice = slice::from_raw_parts_mut(dest, len);
            ixheaacd::combine_fac(src1_slice, src2_slice, dest_slice, shift1, shift2);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_scale_down(
            dest: *mut WORD32,
            src: *mut WORD32,
            len: WORD32,
            shift1: WORD8,
            shift2: WORD8,
        ) {
        if dest.is_null() || src.is_null() || len < 0 {
            return;
        }
        let len = len as usize;
        unsafe {
            let dest_slice = slice::from_raw_parts_mut(dest, len);
            let src_slice = slice::from_raw_parts(src, len);
            ixheaacd::scale_down(dest_slice, src_slice, shift1, shift2);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_scale_down_adj(
            dest: *mut WORD32,
            src: *mut WORD32,
            len: WORD32,
            shift1: WORD8,
            shift2: WORD8,
        ) {
        if dest.is_null() || src.is_null() || len < 0 {
            return;
        }
        let len = len as usize;
        unsafe {
            let dest_slice = slice::from_raw_parts_mut(dest, len);
            let src_slice = slice::from_raw_parts(src, len);
            ixheaacd::scale_down_adj(dest_slice, src_slice, shift1, shift2);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_windowing_long1(
            src1: *mut WORD32,
            src2: *mut WORD32,
            win_fwd: *const WORD32,
            win_rev: *const WORD32,  // C passes pointer to END of array
            dest: *mut WORD32,
            vlen: WORD32,
            shift1: WORD8,
            shift2: WORD8,
        ) -> WORD8 {
        if src1.is_null() || src2.is_null() || win_fwd.is_null() || win_rev.is_null() || dest.is_null() || vlen <= 0 {
            return 0;
        }
        let vlen = vlen as usize;
        unsafe {
            let src1_slice = slice::from_raw_parts(src1, vlen / 2);
            let src2_slice = slice::from_raw_parts(src2, vlen);
            let win_fwd_slice = slice::from_raw_parts(win_fwd, vlen);
            // C passes win_rev pointing to END, convert to beginning
            let win_rev_slice = slice::from_raw_parts(win_rev.sub(vlen - 1), vlen);
            let dest_slice = slice::from_raw_parts_mut(dest, vlen);
            ixheaacd::windowing_long1(src1_slice, src2_slice, win_fwd_slice, win_rev_slice, dest_slice, shift1, shift2)
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_windowing_long2(
            src1: *mut WORD32,
            win_fwd: *const WORD32,
            fac_data_out: *mut WORD32,
            over_lap: *mut WORD32,
            p_out_buffer: *mut WORD32,
            ixheaacd_drc_offset: *mut offset_lengths,
            shift1: WORD8,
            shift2: WORD8,
            shift3: WORD8,
        ) -> WORD8 {
        if src1.is_null() || win_fwd.is_null() || fac_data_out.is_null() ||
           over_lap.is_null() || p_out_buffer.is_null() || ixheaacd_drc_offset.is_null() {
            return 0;
        }
        unsafe {
            let offset = OffsetLengths::from_c_struct(ixheaacd_drc_offset);
            let src1_slice = slice::from_raw_parts(src1, offset.n_long);
            let win_fwd_slice = slice::from_raw_parts(win_fwd, offset.n_long);
            let fac_slice = slice::from_raw_parts(fac_data_out, offset.lfac * 2);
            let overlap_slice = slice::from_raw_parts(over_lap, offset.lfac + offset.n_flat_ls);
            let dest_slice = slice::from_raw_parts_mut(p_out_buffer, offset.n_long);
            ixheaacd::windowing_long2(src1_slice, win_fwd_slice, fac_slice, overlap_slice, dest_slice, &offset, shift1, shift2, shift3)
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_windowing_long3(
            src1: *mut WORD32,
            win_fwd: *const WORD32,
            over_lap: *mut WORD32,
            p_out_buffer: *mut WORD32,
            win_rev: *const WORD32,  // C passes pointer to END of array
            ixheaacd_drc_offset: *mut offset_lengths,
            shift1: WORD8,
            shift2: WORD8,
        ) -> WORD8 {
        if src1.is_null() || win_fwd.is_null() || over_lap.is_null() ||
           p_out_buffer.is_null() || win_rev.is_null() || ixheaacd_drc_offset.is_null() {
            return 0;
        }
        unsafe {
            let offset = OffsetLengths::from_c_struct(ixheaacd_drc_offset);
            let src1_slice = slice::from_raw_parts(src1, offset.n_long);
            let win_fwd_slice = slice::from_raw_parts(win_fwd, offset.n_trans_ls);
            let overlap_slice = slice::from_raw_parts(over_lap, offset.n_trans_ls + offset.n_flat_ls);
            let dest_slice = slice::from_raw_parts_mut(p_out_buffer, offset.n_long);
            // C passes win_rev pointing to END, convert to beginning
            let win_rev_slice = slice::from_raw_parts(win_rev.sub(offset.n_trans_ls - 1), offset.n_trans_ls);
            ixheaacd::windowing_long3(src1_slice, win_fwd_slice, overlap_slice, dest_slice, win_rev_slice, &offset, shift1, shift2)
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_windowing_short1(
            src1: *mut WORD32,
            src2: *mut WORD32,
            fp: *mut WORD32,
            ixheaacd_drc_offset: *mut offset_lengths,
            shiftp: WORD8,
            shift_olap: WORD8,
        ) {
        if src1.is_null() || src2.is_null() || fp.is_null() || ixheaacd_drc_offset.is_null() {
            return;
        }
        unsafe {
            let offset = OffsetLengths::from_c_struct(ixheaacd_drc_offset);
            let src1_slice = slice::from_raw_parts(src1, offset.n_short);
            let src2_slice = slice::from_raw_parts(src2, offset.n_short);
            let fp_slice = slice::from_raw_parts_mut(fp, offset.n_flat_ls +offset. lfac);
            ixheaacd::windowing_short1(src1_slice, src2_slice, fp_slice, &offset, shiftp, shift_olap);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_windowing_short2(
            src1: *mut WORD32,
            win_fwd: *mut WORD32,
            fp: *mut WORD32,
            ixheaacd_drc_offset: *mut offset_lengths,
            shiftp: WORD8,
            shift_olap: WORD8,
        ) {
        if src1.is_null() || win_fwd.is_null() || fp.is_null() || ixheaacd_drc_offset.is_null() {
            return;
        }
        unsafe {
            let offset = OffsetLengths::from_c_struct(ixheaacd_drc_offset);
            let src1_slice = slice::from_raw_parts(src1, offset.n_short / 2);
            let win_fwd_slice = slice::from_raw_parts(win_fwd, offset.n_short);
            let fp_slice = slice::from_raw_parts_mut(fp, offset.n_short + offset.n_flat_ls);
            ixheaacd::windowing_short2(src1_slice, win_fwd_slice, fp_slice, &offset, shiftp, shift_olap);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_windowing_short3(
            src1: *mut WORD32,
            win_rev: *mut WORD32,  // C passes pointer to END of array
            fp: *mut WORD32,
            nshort: WORD32,
            shiftp: WORD8,
            shift_olap: WORD8,
        ) -> WORD8 {
        if src1.is_null() || win_rev.is_null() || fp.is_null() || nshort <= 0 {
            return 0;
        }
        let n_short = nshort as usize;
        unsafe {
            let src1_slice = slice::from_raw_parts(src1, n_short);
            // C passes win_rev pointing to END, convert to beginning
            let win_fwd_slice = slice::from_raw_parts(win_rev.sub(n_short - 1), n_short);
            let fp_slice = slice::from_raw_parts_mut(fp, n_short);
            ixheaacd::windowing_short3(src1_slice, win_fwd_slice, fp_slice, shiftp, shift_olap)
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_windowing_short4(
            src1: *mut WORD32,
            win_fwd: *mut WORD32,
            fp: *mut WORD32,
            win_fwd1: *mut WORD32,  // C passes pointer to END of array
            nshort: WORD32,
            flag: WORD32,
            shiftp: WORD8,
            shift_olap: WORD8,
            output_q: WORD8,
        ) -> WORD8 {
        if src1.is_null() || win_fwd.is_null() || fp.is_null() || win_fwd1.is_null() || nshort <= 0 {
            return 0;
        }
        let n_short = nshort as usize;
        unsafe {
            let src1_slice = slice::from_raw_parts(src1, n_short);
            let win_fwd_slice = slice::from_raw_parts(win_fwd, n_short);
            let fp_slice = slice::from_raw_parts_mut(fp, n_short * 2);
            // C passes win_fwd1 pointing to END, convert to beginning
            let win_rev1_slice = slice::from_raw_parts(win_fwd1.sub(n_short - 1), n_short);
            let windowed_flag = flag != 0;
            ixheaacd::windowing_short4(src1_slice, win_fwd_slice, fp_slice, win_rev1_slice, windowed_flag, shiftp, shift_olap, output_q)
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_vec_cnst_mul(
            a: FLOAT32,
            x: *mut FLOAT32,
            z: *mut FLOAT32,
            n: WORD32,
        ) {
        if x.is_null() || z.is_null() || n <= 0 {
            return;
        }
        let len = n as usize;
        unsafe {
            let x_slice = slice::from_raw_parts(x, len);
            let z_slice = slice::from_raw_parts_mut(z, len);
            ixheaacd::vec_cnst_mul(a, x_slice, z_slice);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_memset(x: *mut FLOAT32, n: WORD32) {
        if x.is_null() || n <= 0 {
            return;
        }
        unsafe {
            let slice = slice::from_raw_parts_mut(x, n as usize);
            slice.fill(0.0);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_mem_cpy(x: *const FLOAT32, y: *mut FLOAT32, n: WORD32) {
        if x.is_null() || y.is_null() || n <= 0 {
            return;
        }
        unsafe {
            let src = slice::from_raw_parts(x, n as usize);
            let dst = slice::from_raw_parts_mut(y, n as usize);
            dst.copy_from_slice(src);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_acelp_imdct(
        imdct_in: *mut WORD32,
        npoints: WORD32,
        qshift: *mut WORD8,
        scratch: *mut WORD32,
    ) {
        if imdct_in.is_null() || qshift.is_null() || scratch.is_null() || npoints <= 0 {
            return;
        }
        unsafe {
            let imdct_len = (npoints / 2) as usize;
            let imdct_slice = slice::from_raw_parts_mut(imdct_in, imdct_len);
            let scratch_slice = slice::from_raw_parts_mut(scratch, 1024);
            ixheaacd::acelp_imdct(imdct_slice, npoints, &mut *qshift, scratch_slice);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_fd_imdct_long(
        usac_data: *mut ia_usac_data_struct,
        i_ch: WORD32,
        fac_idata: *mut WORD32,
        ixheaacd_drc_offset: *mut offset_lengths,
        fac_q: WORD8,
    ) -> IA_ERRORCODE {
        if usac_data.is_null() || fac_idata.is_null() || ixheaacd_drc_offset.is_null() {
            return -1;
        }
        unsafe {
            let offset = OffsetLengths::from_c_struct(ixheaacd_drc_offset);
            // FAC buffer size: 2*FAC_LENGTH+16 = 272
            let fac_slice = slice::from_raw_parts_mut(fac_idata, 272);
            let result = ixheaacd::fd_imdct_long(
                &mut *usac_data,
                i_ch,
                fac_slice,
                &offset,
                fac_q,
            );
            if result.is_ok() { 0 } else { -1 }
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ixheaacd_fd_imdct_short(
        usac_data: *mut ia_usac_data_struct,
        i_ch: WORD32,
        fac_data_out: *mut WORD32,
        ixheaacd_drc_offset: *mut offset_lengths,
        fac_q: WORD8,
    ) -> IA_ERRORCODE {
        if usac_data.is_null() || fac_data_out.is_null() || ixheaacd_drc_offset.is_null() {
            return -1;
        }
        unsafe {
            let offset = OffsetLengths::from_c_struct(ixheaacd_drc_offset);
            // FAC buffer size: 2*FAC_LENGTH+16 = 272
            let fac_slice = slice::from_raw_parts_mut(fac_data_out, 272);
            let result = ixheaacd::fd_imdct_short(
                &mut *usac_data,
                i_ch,
                fac_slice,
                &offset,
                fac_q,
            );
            if result.is_ok() { 0 } else { -1 }
        }
    }

}