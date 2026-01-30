extern "C" {
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
}
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
pub type __jmp_buf = [core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [core::ffi::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_crc_bit_buf_struct {
    pub ptr_bit_buf_base: *mut UWORD8,
    pub ptr_bit_buf_end: *mut UWORD8,
    pub ptr_read_next: *mut UWORD8,
    pub bit_pos: WORD16,
    pub cnt_bits: WORD32,
    pub size: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_crc_reg_data_struct {
    pub active: UWORD8,
    pub buf_size: WORD32,
    pub max_bits: WORD32,
    pub bit_cnt: UWORD32,
    pub bit_buf_cnt: WORD32,
    pub str_bit_buf: ia_crc_bit_buf_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_adts_crc_info_struct {
    pub crc_active: UWORD8,
    pub no_reg: UWORD16,
    pub file_value: UWORD16,
    pub crc_lookup: [UWORD16; 256],
    pub str_crc_reg_data: [ia_crc_reg_data_struct; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_bit_buf_struct {
    pub ptr_bit_buf_base: *mut UWORD8,
    pub ptr_bit_buf_end: *mut UWORD8,
    pub ptr_read_next: *mut UWORD8,
    pub bit_pos: WORD32,
    pub cnt_bits: WORD32,
    pub size: WORD32,
    pub adts_header_present: WORD32,
    pub crc_check: WORD32,
    pub protection_absent: WORD8,
    pub no_raw_data_blocks: WORD8,
    pub str_adts_crc_info: ia_adts_crc_info_struct,
    pub pstr_adts_crc_info: *mut ia_adts_crc_info_struct,
    pub initial_cnt_bits: WORD32,
    pub audio_mux_align: WORD32,
    pub bit_count: WORD32,
    pub valid_bits: WORD32,
    pub byte: UWORD8,
    pub byte_ptr: *mut UWORD8,
    pub ptr_start: *mut UWORD8,
    pub write_bit_count: WORD32,
    pub max_size: WORD32,
    pub xaac_jmp_buf: *mut jmp_buf,
}
pub type UINT32 = UWORD32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_dec_mps_config_struct {
    pub bs_freq_res: UINT32,
    pub bs_fixed_gain_dmx: UINT32,
    pub bs_temp_shape_config: UINT32,
    pub bs_decorr_config: UINT32,
    pub bs_high_rate_mode: UINT32,
    pub bs_phase_coding: UINT32,
    pub bs_ott_bands_phase_present: UINT32,
    pub bs_ott_bands_phase: UINT32,
    pub bs_residual_bands: UINT32,
    pub bs_pseudo_lr: UINT32,
    pub bs_env_quant_mode: UINT32,
    pub ldmps_present_flag: UINT32,
    pub bs_sampling_freq_index: UINT32,
    pub bs_fampling_frequency: UINT32,
    pub bs_frame_length: UINT32,
    pub bs_tree_config: UINT32,
    pub bs_quant_mode: UINT32,
    pub bs_one_icc: UINT32,
    pub bs_arbitrary_downmix: UINT32,
    pub bs_residual_coding: UINT32,
    pub bs_fixed_gain_sur: UINT32,
    pub bs_fixed_gain_LFE: UINT32,
    pub bs_matrix_mode: UINT32,
    pub bs_3D_audio_mode: UINT32,
    pub bs_3D_audio_HRTF_set: UINT32,
    pub bs_HRTF_freq_res: UINT32,
    pub HRTF_num_band: UINT32,
    pub HRTF_num_phase: UINT32,
    pub bs_HRTF_num_chan: UINT32,
    pub bs_HRTF_asymmetric: UINT32,
    pub bs_HRTF_level_left: [[UINT32; 28]; 7],
    pub bs_HRTF_level_right: [[UINT32; 28]; 7],
    pub bs_HRTF_phase: [UINT32; 7],
    pub bs_HRTF_phase_LR: [[UINT32; 28]; 7],
    pub bs_HRTF_icc: [UINT32; 7],
    pub bs_HRTF_icc_LR: [[UINT32; 28]; 7],
    pub bs_ott_bands: [UINT32; 5],
    pub bs_ttt_dual_mode: [UINT32; 1],
    pub bs_ttt_mode_low: [UINT32; 1],
    pub bs_ttt_mode_high: [UINT32; 1],
    pub bs_ttt_bands_low: [UINT32; 1],
    pub bs_ttt_bands_high: [UINT32; 1],
    pub bs_sac_ext_type: [UINT32; 8],
    pub sac_ext_cnt: UINT32,
    pub bs_residual_present: [UINT32; 3],
    pub bs_residual_sampling_freq_index: UINT32,
    pub bs_residual_frames_per_spatial_frame: UINT32,
    pub bs_residual_bands_ld_mps: [UINT32; 3],
    pub bs_arbitrary_downmix_residual_sampling_freq_index: UINT32,
    pub bs_arbitrary_downmix_residual_frames_per_spatial_frame: UINT32,
    pub bs_arbitrary_downmix_residual_bands: WORD32,
    pub num_out_chan_AT: UINT32,
    pub num_ott_boxes_AT: UINT32,
    pub bs_output_channel_pos_AT: [UINT32; 28],
    pub bs_ott_box_present_AT: [[UINT32; 7]; 7],
    pub bs_ott_default_cld_AT: [UINT32; 49],
    pub bs_ott_mode_lfe_AT: [UINT32; 49],
    pub bs_ott_bands_AT: [UINT32; 49],
    pub num_ott_boxes: WORD32,
    pub num_ttt_boxes: WORD32,
    pub num_input_channels: WORD32,
    pub num_output_channels: WORD32,
    pub ott_mode_lfe: [WORD32; 5],
    pub no_ldsbr_present: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ld_mps_dec_tree_properties_struct {
    pub num_input_chan: WORD32,
    pub num_output_chan: WORD32,
    pub num_ott_boxes: WORD32,
    pub num_ttt_boxes: WORD32,
    pub ott_mode_lfe: [WORD32; 5],
}
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const MAX_PARAMETER_BANDS: core::ffi::c_int = 28 as core::ffi::c_int;
pub const MAX_NUM_OTT: core::ffi::c_int = 5 as core::ffi::c_int;
pub const MAX_RESIDUAL_CHANNELS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const MAX_OUTPUT_CHANNELS: core::ffi::c_int = 7 as core::ffi::c_int;
pub const MAX_NUM_EXT_TYPES: core::ffi::c_int = 8 as core::ffi::c_int;
pub const MAX_ARBITRARY_TREE_LEVELS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_ARBITRARY_TREE_INDEX: core::ffi::c_int = ((1 as core::ffi::c_int)
    << MAX_ARBITRARY_TREE_LEVELS + 1 as core::ffi::c_int) - 1 as core::ffi::c_int;
pub const MAX_RES_SAMP_FREQ_IDX: core::ffi::c_int = 11 as core::ffi::c_int;
static mut ixheaacd_freq_res_table: [WORD32; 8] = [
    0 as core::ffi::c_int,
    23 as core::ffi::c_int,
    15 as core::ffi::c_int,
    12 as core::ffi::c_int,
    9 as core::ffi::c_int,
    7 as core::ffi::c_int,
    5 as core::ffi::c_int,
    4 as core::ffi::c_int,
];
static mut ixheaacd_hrtf_freq_res_table: [[WORD32; 8]; 2] = [
    [
        0 as core::ffi::c_int,
        28 as core::ffi::c_int,
        20 as core::ffi::c_int,
        14 as core::ffi::c_int,
        10 as core::ffi::c_int,
        7 as core::ffi::c_int,
        5 as core::ffi::c_int,
        4 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int,
        13 as core::ffi::c_int,
        13 as core::ffi::c_int,
        8 as core::ffi::c_int,
        7 as core::ffi::c_int,
        4 as core::ffi::c_int,
        3 as core::ffi::c_int,
        3 as core::ffi::c_int,
    ],
];
static mut ixheaacd_tree_property_table: [ia_ld_mps_dec_tree_properties_struct; 8] = [
    {
        let mut init = ia_ld_mps_dec_tree_properties_struct {
            num_input_chan: 1 as WORD32,
            num_output_chan: 6 as WORD32,
            num_ott_boxes: 5 as WORD32,
            num_ttt_boxes: 0 as WORD32,
            ott_mode_lfe: [
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
            ],
        };
        init
    },
    {
        let mut init = ia_ld_mps_dec_tree_properties_struct {
            num_input_chan: 1 as WORD32,
            num_output_chan: 6 as WORD32,
            num_ott_boxes: 5 as WORD32,
            num_ttt_boxes: 0 as WORD32,
            ott_mode_lfe: [
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                1 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ],
        };
        init
    },
    {
        let mut init = ia_ld_mps_dec_tree_properties_struct {
            num_input_chan: 2 as WORD32,
            num_output_chan: 6 as WORD32,
            num_ott_boxes: 3 as WORD32,
            num_ttt_boxes: 1 as WORD32,
            ott_mode_lfe: [
                1 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ],
        };
        init
    },
    {
        let mut init = ia_ld_mps_dec_tree_properties_struct {
            num_input_chan: 2 as WORD32,
            num_output_chan: 8 as WORD32,
            num_ott_boxes: 5 as WORD32,
            num_ttt_boxes: 1 as WORD32,
            ott_mode_lfe: [
                1 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ],
        };
        init
    },
    {
        let mut init = ia_ld_mps_dec_tree_properties_struct {
            num_input_chan: 2 as WORD32,
            num_output_chan: 8 as WORD32,
            num_ott_boxes: 5 as WORD32,
            num_ttt_boxes: 1 as WORD32,
            ott_mode_lfe: [
                1 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ],
        };
        init
    },
    {
        let mut init = ia_ld_mps_dec_tree_properties_struct {
            num_input_chan: 6 as WORD32,
            num_output_chan: 8 as WORD32,
            num_ott_boxes: 2 as WORD32,
            num_ttt_boxes: 0 as WORD32,
            ott_mode_lfe: [
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ],
        };
        init
    },
    {
        let mut init = ia_ld_mps_dec_tree_properties_struct {
            num_input_chan: 6 as WORD32,
            num_output_chan: 8 as WORD32,
            num_ott_boxes: 2 as WORD32,
            num_ttt_boxes: 0 as WORD32,
            ott_mode_lfe: [
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ],
        };
        init
    },
    {
        let mut init = ia_ld_mps_dec_tree_properties_struct {
            num_input_chan: 1 as WORD32,
            num_output_chan: 2 as WORD32,
            num_ott_boxes: 1 as WORD32,
            num_ttt_boxes: 0 as WORD32,
            ott_mode_lfe: [
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ],
        };
        init
    },
];
unsafe extern "C" fn ixheaacd_ld_spatial_extension_config(
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut config: *mut ia_usac_dec_mps_config_struct,
    mut bits_available: WORD32,
) -> IA_ERRORCODE {
    let mut j: WORD32 = 0;
    let mut ch: WORD32 = 0;
    let mut idx: WORD32 = 0;
    let mut tmp: WORD32 = 0;
    let mut tmp_open: WORD32 = 0;
    let mut sac_ext_len: WORD32 = 0;
    let mut bits_read: WORD32 = 0;
    let mut n_fill_bits: WORD32 = 0;
    let mut i: UWORD32 = 0;
    let mut ba: WORD32 = bits_available;
    (*config).sac_ext_cnt = 0 as UINT32;
    tmp = (*it_bit_buff).cnt_bits;
    while ba >= 8 as core::ffi::c_int {
        if (*config).sac_ext_cnt >= MAX_NUM_EXT_TYPES as UINT32 {
            return IA_FATAL_ERROR as IA_ERRORCODE;
        }
        (*config).bs_sac_ext_type[(*config).sac_ext_cnt as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            4 as WORD,
        ) as UINT32;
        ba -= 4 as core::ffi::c_int;
        sac_ext_len = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD);
        ba -= 4 as core::ffi::c_int;
        if ba >= 6 as core::ffi::c_int && sac_ext_len > 0 as core::ffi::c_int {
            if sac_ext_len == 15 as core::ffi::c_int {
                sac_ext_len += ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
                ba -= 8 as core::ffi::c_int;
                if sac_ext_len == 15 as core::ffi::c_int + 255 as core::ffi::c_int {
                    sac_ext_len += ixheaacd_read_bits_buf(it_bit_buff, 16 as WORD);
                    ba -= 16 as core::ffi::c_int;
                }
            }
            match (*config).bs_sac_ext_type[(*config).sac_ext_cnt as usize] {
                0 => {
                    (*config).bs_residual_coding = 1 as UINT32;
                    (*config).bs_residual_sampling_freq_index = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        4 as WORD,
                    ) as UINT32;
                    if (*config).bs_residual_sampling_freq_index
                        > MAX_RES_SAMP_FREQ_IDX as UINT32
                    {
                        return IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                    (*config).bs_residual_frames_per_spatial_frame = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        2 as WORD,
                    ) as UINT32;
                    if (*config).num_ott_boxes + (*config).num_ttt_boxes
                        > MAX_RESIDUAL_CHANNELS
                    {
                        return IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                    j = 0 as core::ffi::c_int as WORD32;
                    while j < (*config).num_ott_boxes + (*config).num_ttt_boxes {
                        (*config).bs_residual_present[j as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        ) as UINT32;
                        if (*config).bs_residual_present[j as usize] != 0 {
                            (*config).bs_residual_bands_ld_mps[j as usize] = ixheaacd_read_bits_buf(
                                it_bit_buff,
                                5 as WORD,
                            ) as UINT32;
                            if (*config).bs_residual_bands_ld_mps[j as usize]
                                > MAX_PARAMETER_BANDS as UINT32
                            {
                                return IA_FATAL_ERROR as IA_ERRORCODE;
                            }
                        }
                        j += 1;
                    }
                }
                1 => {
                    (*config).bs_arbitrary_downmix = 2 as UINT32;
                    (*config).bs_arbitrary_downmix_residual_sampling_freq_index = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        4 as WORD,
                    ) as UINT32;
                    if (*config).bs_arbitrary_downmix_residual_sampling_freq_index
                        > MAX_RES_SAMP_FREQ_IDX as UINT32
                    {
                        return IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                    (*config).bs_arbitrary_downmix_residual_frames_per_spatial_frame = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        2 as WORD,
                    ) as UINT32;
                    (*config).bs_arbitrary_downmix_residual_bands = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        5 as WORD,
                    );
                    if (*config).bs_arbitrary_downmix_residual_bands
                        >= ixheaacd_freq_res_table[(*config).bs_freq_res as usize]
                    {
                        return IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                }
                2 => {
                    (*config).num_out_chan_AT = 0 as UINT32;
                    (*config).num_ott_boxes_AT = 0 as UINT32;
                    if (*config).num_output_channels > MAX_OUTPUT_CHANNELS {
                        return IA_FATAL_ERROR as IA_ERRORCODE;
                    }
                    ch = 0 as core::ffi::c_int as WORD32;
                    while ch < (*config).num_output_channels {
                        tmp_open = 1 as core::ffi::c_int as WORD32;
                        idx = 0 as core::ffi::c_int as WORD32;
                        while tmp_open > 0 as core::ffi::c_int
                            && idx < MAX_ARBITRARY_TREE_INDEX
                        {
                            (*config).bs_ott_box_present_AT[ch as usize][idx as usize] = ixheaacd_read_bits_buf(
                                it_bit_buff,
                                1 as WORD,
                            ) as UINT32;
                            if (*config).bs_ott_box_present_AT[ch as usize][idx as usize]
                                != 0
                            {
                                (*config).num_ott_boxes_AT = ((*config).num_ott_boxes_AT)
                                    .wrapping_add(1);
                                tmp_open += 1;
                            } else {
                                (*config).num_out_chan_AT = ((*config).num_out_chan_AT)
                                    .wrapping_add(1);
                                tmp_open -= 1;
                            }
                            idx += 1;
                        }
                        ch += 1;
                    }
                    i = 0 as UWORD32;
                    while i < (*config).num_ott_boxes_AT {
                        (*config).bs_ott_default_cld_AT[i as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        ) as UINT32;
                        (*config).bs_ott_mode_lfe_AT[i as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            1 as WORD,
                        ) as UINT32;
                        if (*config).bs_ott_mode_lfe_AT[i as usize] != 0 {
                            (*config).bs_ott_bands_AT[i as usize] = ixheaacd_read_bits_buf(
                                it_bit_buff,
                                5 as WORD,
                            ) as UINT32;
                        } else {
                            (*config).bs_ott_bands_AT[i as usize] = ixheaacd_freq_res_table[(*config)
                                .bs_freq_res as usize] as UINT32;
                        }
                        i = i.wrapping_add(1);
                    }
                    i = 0 as UWORD32;
                    while i < (*config).num_out_chan_AT {
                        (*config).bs_output_channel_pos_AT[i as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            5 as WORD,
                        ) as UINT32;
                        i = i.wrapping_add(1);
                    }
                }
                _ => {}
            }
        }
        bits_read = tmp - (*it_bit_buff).cnt_bits;
        n_fill_bits = 8 as WORD32 * sac_ext_len - bits_read;
        while n_fill_bits > 7 as core::ffi::c_int {
            ixheaacd_read_bits_buf(it_bit_buff, 8 as WORD);
            n_fill_bits -= 8 as core::ffi::c_int;
        }
        if n_fill_bits > 0 as core::ffi::c_int {
            ixheaacd_read_bits_buf(it_bit_buff, n_fill_bits as WORD);
        }
        ba -= (8 as WORD32 * sac_ext_len) as core::ffi::c_int;
        (*config).sac_ext_cnt = ((*config).sac_ext_cnt).wrapping_add(1);
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_ld_spatial_specific_config(
    mut config: *mut ia_usac_dec_mps_config_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> IA_ERRORCODE {
    let mut i: WORD32 = 0;
    let mut num_header_bits: WORD32 = 0;
    let mut hc: UWORD32 = 0;
    let mut hb: UWORD32 = 0;
    let mut sac_header_len: WORD32 = 0;
    let mut bits_available: WORD32 = 0;
    let mut tmp: WORD32 = (*it_bit_buff).cnt_bits;
    let mut err: WORD32 = 0 as WORD32;
    sac_header_len = tmp;
    bits_available = sac_header_len;
    (*config).bs_sampling_freq_index = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD)
        as UINT32;
    if (*config).bs_sampling_freq_index == 15 as UINT32 {
        (*config).bs_fampling_frequency = ixheaacd_read_bits_buf(it_bit_buff, 24 as WORD)
            as UINT32;
    }
    (*config).bs_frame_length = ixheaacd_read_bits_buf(it_bit_buff, 5 as WORD) as UINT32;
    (*config).bs_freq_res = ixheaacd_read_bits_buf(it_bit_buff, 3 as WORD) as UINT32;
    (*config).bs_tree_config = ixheaacd_read_bits_buf(it_bit_buff, 4 as WORD) as UINT32;
    if (*config).bs_tree_config > 7 as UINT32 {
        return IA_FATAL_ERROR as IA_ERRORCODE;
    }
    if (*config).bs_tree_config != 15 as UINT32 {
        (*config).num_ott_boxes = ixheaacd_tree_property_table[(*config).bs_tree_config
                as usize]
            .num_ott_boxes;
        (*config).num_ttt_boxes = ixheaacd_tree_property_table[(*config).bs_tree_config
                as usize]
            .num_ttt_boxes;
        (*config).num_input_channels = ixheaacd_tree_property_table[(*config)
                .bs_tree_config as usize]
            .num_input_chan;
        (*config).num_output_channels = ixheaacd_tree_property_table[(*config)
                .bs_tree_config as usize]
            .num_output_chan;
        i = 0 as core::ffi::c_int as WORD32;
        while i < MAX_NUM_OTT {
            (*config).ott_mode_lfe[i as usize] = ixheaacd_tree_property_table[(*config)
                    .bs_tree_config as usize]
                .ott_mode_lfe[i as usize];
            i += 1;
        }
    }
    (*config).bs_quant_mode = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD) as UINT32;
    if (*config).bs_tree_config != 7 as UINT32 {
        (*config).bs_one_icc = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD) as UINT32;
    }
    (*config).bs_arbitrary_downmix = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
        as UINT32;
    if (*config).bs_tree_config != 7 as UINT32 {
        (*config).bs_fixed_gain_sur = ixheaacd_read_bits_buf(it_bit_buff, 3 as WORD)
            as UINT32;
        (*config).bs_fixed_gain_LFE = ixheaacd_read_bits_buf(it_bit_buff, 3 as WORD)
            as UINT32;
    }
    (*config).bs_fixed_gain_dmx = ixheaacd_read_bits_buf(it_bit_buff, 3 as WORD)
        as UINT32;
    if (*config).bs_tree_config != 7 as UINT32 {
        (*config).bs_matrix_mode = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
            as UINT32;
    }
    (*config).bs_temp_shape_config = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD)
        as UINT32;
    (*config).bs_decorr_config = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD)
        as UINT32;
    if (*config).bs_tree_config != 7 as UINT32 {
        (*config).bs_3D_audio_mode = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
            as UINT32;
    } else {
        (*config).bs_3D_audio_mode = 0 as UINT32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*config).num_ott_boxes {
        if (*config).ott_mode_lfe[i as usize] != 0 {
            (*config).bs_ott_bands[i as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                5 as WORD,
            ) as UINT32;
        } else {
            (*config).bs_ott_bands[i as usize] = ixheaacd_freq_res_table[(*config)
                .bs_freq_res as usize] as UINT32;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < (*config).num_ttt_boxes {
        (*config).bs_ttt_dual_mode[i as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            1 as WORD,
        ) as UINT32;
        (*config).bs_ttt_mode_low[i as usize] = ixheaacd_read_bits_buf(
            it_bit_buff,
            3 as WORD,
        ) as UINT32;
        if (*config).bs_ttt_dual_mode[i as usize] != 0 {
            (*config).bs_ttt_mode_high[i as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                3 as WORD,
            ) as UINT32;
            (*config).bs_ttt_bands_low[i as usize] = ixheaacd_read_bits_buf(
                it_bit_buff,
                5 as WORD,
            ) as UINT32;
            (*config).bs_ttt_bands_high[i as usize] = ixheaacd_freq_res_table[(*config)
                .bs_freq_res as usize] as UINT32;
        } else {
            (*config).bs_ttt_bands_low[i as usize] = ixheaacd_freq_res_table[(*config)
                .bs_freq_res as usize] as UINT32;
        }
        i += 1;
    }
    if (*config).bs_temp_shape_config == 2 as UINT32 {
        (*config).bs_env_quant_mode = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
            as UINT32;
    }
    if (*config).bs_3D_audio_mode != 0 {
        (*config).bs_3D_audio_HRTF_set = ixheaacd_read_bits_buf(it_bit_buff, 2 as WORD)
            as UINT32;
        if (*config).bs_3D_audio_HRTF_set == 0 as UINT32 {
            (*config).bs_HRTF_freq_res = ixheaacd_read_bits_buf(it_bit_buff, 3 as WORD)
                as UINT32;
            (*config).bs_HRTF_num_chan = 5 as UINT32;
            (*config).bs_HRTF_asymmetric = ixheaacd_read_bits_buf(it_bit_buff, 1 as WORD)
                as UINT32;
            (*config).HRTF_num_band = ixheaacd_hrtf_freq_res_table[0 as core::ffi::c_int
                as usize][(*config).bs_HRTF_freq_res as usize] as UINT32;
            (*config).HRTF_num_phase = ixheaacd_hrtf_freq_res_table[1 as core::ffi::c_int
                as usize][(*config).bs_HRTF_freq_res as usize] as UINT32;
            hc = 0 as UWORD32;
            while hc < (*config).bs_HRTF_num_chan {
                hb = 0 as UWORD32;
                while hb < (*config).HRTF_num_band {
                    (*config).bs_HRTF_level_left[hc as usize][hb as usize] = ixheaacd_read_bits_buf(
                        it_bit_buff,
                        6 as WORD,
                    ) as UINT32;
                    hb = hb.wrapping_add(1);
                }
                hb = 0 as UWORD32;
                while hb < (*config).HRTF_num_band {
                    (*config).bs_HRTF_level_right[hc as usize][hb as usize] = if (*config)
                        .bs_HRTF_asymmetric != 0
                    {
                        ixheaacd_read_bits_buf(it_bit_buff, 6 as WORD) as UINT32
                    } else {
                        (*config).bs_HRTF_level_left[hc as usize][hb as usize]
                    };
                    hb = hb.wrapping_add(1);
                }
                (*config).bs_HRTF_phase[hc as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                ) as UINT32;
                hb = 0 as UWORD32;
                while hb < (*config).HRTF_num_phase {
                    (*config).bs_HRTF_phase_LR[hc as usize][hb as usize] = (if (*config)
                        .bs_HRTF_phase[hc as usize] != 0
                    {
                        ixheaacd_read_bits_buf(it_bit_buff, 6 as WORD)
                            as core::ffi::c_int
                    } else {
                        0 as core::ffi::c_int
                    }) as UINT32;
                    hb = hb.wrapping_add(1);
                }
                (*config).bs_HRTF_icc[hc as usize] = ixheaacd_read_bits_buf(
                    it_bit_buff,
                    1 as WORD,
                ) as UINT32;
                if (*config).bs_HRTF_icc[hc as usize] != 0 {
                    hb = 0 as UWORD32;
                    while hb < (*config).HRTF_num_band {
                        (*config).bs_HRTF_icc_LR[hc as usize][hb as usize] = ixheaacd_read_bits_buf(
                            it_bit_buff,
                            3 as WORD,
                        ) as UINT32;
                        hb = hb.wrapping_add(1);
                    }
                }
                hc = hc.wrapping_add(1);
            }
        }
    }
    i = ((*it_bit_buff).cnt_bits as core::ffi::c_int & 0x7 as core::ffi::c_int)
        as WORD32;
    ixheaacd_read_bits_buf(it_bit_buff, i as WORD);
    num_header_bits = tmp - (*it_bit_buff).cnt_bits;
    bits_available -= num_header_bits;
    err = ixheaacd_ld_spatial_extension_config(it_bit_buff, config, bits_available)
        as WORD32;
    return err as IA_ERRORCODE;
}
