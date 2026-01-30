extern "C" {
    pub type ia_mps_dec_ducker_interface;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memmove(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn _setjmp(__env: *mut __jmp_buf_tag) -> core::ffi::c_int;
    fn ixheaacd_byte_align(
        it_bit_buff: *mut ia_bit_buf_struct,
        ptr_byte_align_bits: *mut WORD32,
    ) -> VOID;
    fn ixheaacd_create_bit_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        ptr_bit_buf_base: *mut UWORD8,
        bit_buf_size: WORD32,
    ) -> *mut ia_bit_buf_struct;
    fn ixheaacd_create_init_bit_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        ptr_bit_buf_base: *mut UWORD8,
        bit_buf_size: WORD32,
    ) -> VOID;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
    fn ixheaacd_read_bidirection(
        it_bit_buff: *mut ia_bit_buf_struct,
        ixheaacd_drc_offset: WORD32,
    ) -> VOID;
    fn ixheaacd_find_syncword(
        adts: *mut ia_adts_header_struct,
        it_bit_buff: *mut ia_bit_buf_struct,
    ) -> WORD32;
    fn ixheaacd_check_if_adts(
        adts: *mut ia_adts_header_struct,
        it_bit_buff: *mut ia_bit_buf_struct,
        usr_max_ch: WORD32,
    ) -> WORD32;
    static ixheaacd_aac_block_tables: ia_aac_dec_block_tables_struct;
    static ixheaacd_aac_huffmann_tables: ia_aac_dec_huffman_tables_struct;
    static ixheaacd_imdct_tables: ia_aac_dec_imdct_tables_struct;
    static ixheaacd_aac_dec_env_calc_tables: ia_env_calc_tables_struct;
    static ixheaacd_aac_qmf_dec_tables: ia_qmf_dec_tables_struct;
    static ixheaacd_aac_dec_env_extr_tables: ia_env_extr_tables_struct;
    static ixheaacd_aac_dec_ps_tables: ia_ps_tables_struct;
    fn ixheaacd_drc_dec_create(
        pstr_hdrc_dec: *mut ia_drc_dec_struct,
        drc_ref_level: WORD16,
        drc_def_level: WORD16,
    ) -> VOID;
    fn ixheaacd_applysbr(
        self_0: ia_handle_sbr_dec_inst_struct,
        p_sbr_bit_stream: *mut ia_aac_dec_sbr_bitstream_struct,
        core_sample_buf: *mut WORD16,
        codec_num_channels: *mut WORD16,
        frame_status: FLAG,
        down_samp_flag: FLAG,
        down_mix_flag: FLAG,
        sbr_scratch_struct: *mut ia_sbr_scr_struct,
        ps_enable: WORD32,
        ch_fac: WORD32,
        slot_element: WORD32,
        it_bit_buff: *mut ia_bit_buf_struct,
        pstr_drc_dec: *mut ia_drc_dec_struct,
        eld_sbr_flag: WORD,
        audio_object_type: WORD,
        init_flag: WORD32,
        ldmps_present: WORD32,
        frame_size: WORD32,
        heaac_mps_present: WORD32,
        ec_flag: WORD32,
        first_frame: FLAG,
    ) -> IA_ERRORCODE;
    fn ixheaacd_getsize_sbr_persistent() -> WORD32;
    fn ixheaacd_set_sbr_persistent_buffers(
        aac_persistent_mem_v: *mut core::ffi::c_void,
        persistent_used: *mut WORD32,
        channels: WORD32,
        ps_enable: WORD32,
    ) -> VOID;
    static ixheaacd_str_fft_n_transcendent_tables: ixheaacd_misc_tables;
    fn ixheaacd_huff_tables_create(_: *mut ia_aac_dec_tables_struct) -> VOID;
    fn ixheaacd_set_aac_persistent_buffers(
        aac_persistent_mem_v: *mut core::ffi::c_void,
        channels: WORD32,
    ) -> WORD32;
    fn ixheaacd_samples_sat(
        outbuffer: *mut WORD8,
        num_samples_out: WORD32,
        pcmsize: WORD32,
        out_samples: *mut [FLOAT32; 4096],
        out_bytes: *mut WORD32,
        num_channel_out: WORD32,
    ) -> VOID;
    fn ixheaacd_samples_sat_mc(
        outbuffer: *mut WORD8,
        num_samples_out: WORD32,
        out_samples: *mut [FLOAT32; 4096],
        out_bytes: *mut WORD32,
        num_channel_out: WORD32,
        ch_fac: WORD32,
    ) -> VOID;
    fn ixheaacd_mps_persistent_buffer_sizes() -> WORD32;
    fn ixheaacd_getsize_mps_persistent() -> WORD32;
    fn ixheaacd_set_mps_persistent_buffers(
        pstr_mps_state: *mut ia_heaac_mps_state_struct,
        persistent_used: *mut WORD32,
        num_channel: WORD32,
        persistent_mem: *mut core::ffi::c_void,
    ) -> VOID;
    fn ixheaacd_set_scratch_buffers(
        pstr_mps_state: *mut ia_heaac_mps_state_struct,
        scratch_mem: *mut core::ffi::c_void,
    ) -> VOID;
    fn ixheaacd_aacdec_decodeframe(
        p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
        aac_scratch_ptrs: *mut ia_aac_dec_scratch_struct,
        time_data: *mut core::ffi::c_void,
        frame_status: FLAG,
        type_0: *mut WORD,
        ch_idx: *mut WORD,
        init_flag: WORD,
        channel: WORD,
        element_index_order: *mut WORD,
        skip_full_decode: WORD,
        ch_fac: WORD,
        slot_element: WORD,
        max_channels: WORD,
        total_channels: WORD32,
        frame_length: WORD32,
        frame_size: WORD32,
        pstr_drc_dec: *mut ia_drc_dec_struct,
        object_type: WORD32,
        ch_config: WORD32,
        eld_specific_config: ia_eld_specific_config_struct,
        adtsheader: WORD16,
        drc_dummy: *mut ia_drc_dec_struct,
        ldmps_present: WORD32,
        slot_pos: *mut UWORD8,
        mps_buffer: *mut UWORD8,
        mps_header: *mut WORD32,
        mps_bytes: *mut WORD32,
        is_init: WORD32,
        first_frame: WORD32,
    ) -> WORD32;
    fn ixheaacd_get_channel_mask(
        p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
    ) -> WORD;
    fn ixheaacd_allocate_mem_persistent(
        p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
        p_state_enhaacplus_dec: *mut ia_aac_dec_state_struct,
        channels: WORD,
        persistent_used_total: *mut WORD,
        sbr_persistent_start: *mut WORD,
        ps_enable: WORD,
    ) -> VOID;
    fn ixheaacd_aac_mps_init(
        p_obj_mps_dec: *mut ia_exhaacplus_dec_api_struct,
        databuf: *mut UWORD8,
        buffer_size: WORD32,
        sample_rate: WORD32,
    ) -> IA_ERRORCODE;
    fn ixheaacd_peak_limiter_init(
        peak_limiter: *mut ia_peak_limiter_struct,
        num_channels: UWORD32,
        sample_rate: UWORD32,
        buffer: *mut FLOAT32,
        delay_in_samples: *mut UWORD32,
    ) -> WORD32;
    fn ixheaacd_peak_limiter_process(
        peak_limiter: *mut ia_peak_limiter_struct,
        samples: *mut core::ffi::c_void,
        frame_len: UWORD32,
        qshift_adj: *mut WORD8,
    ) -> VOID;
    fn ixheaacd_scale_adjust(
        samples: *mut core::ffi::c_void,
        frame_len: UWORD32,
        qshift_adj: *mut WORD8,
        num_channels: WORD,
    ) -> VOID;
    fn ixheaacd_aac_headerdecode(
        p_obj_enhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
        buffer: *mut UWORD8,
        bytes_consumed: *mut WORD32,
        ptr_huffmann_tables: *const ia_aac_dec_huffman_tables_struct,
    ) -> WORD32;
    fn ixheaacd_aac_decoder_init(
        p_state_enhaacplus_dec: *mut ia_aac_dec_state_struct,
        ptr_sbr_bitstream: *mut ia_aac_dec_sbr_bitstream_struct,
        channels: WORD,
        aac_persistent_mem_v: *mut core::ffi::c_void,
        frame_length: WORD32,
    ) -> *mut ia_aac_decoder_struct;
    fn ixheaacd_latm_audio_mux_element(
        it_bit_buff: *mut ia_bit_buf_struct,
        latm_element: *mut ixheaacd_latm_struct,
        aac_state_struct: *mut ia_aac_dec_state_struct,
        sample_rate_info: *mut ia_sampling_rate_info_struct,
    ) -> WORD32;
    fn ixheaacd_get_element_index_tag(
        p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
        ch_idx1: WORD,
        ch_idx: *mut WORD,
        channel: *mut WORD,
        element_index_order: *mut WORD,
        total_elements: WORD,
        element_used: *mut WORD8,
        total_channels: WORD,
        pstr_drc_dec: *mut ia_drc_dec_struct,
        drc_dummy: *mut ia_drc_dec_struct,
        mps_buffer: *mut UWORD8,
        mps_header: *mut WORD32,
        mps_bytes: *mut WORD32,
    ) -> WORD32;
    fn ixheaacd_set_sbr_persistent_table_pointer(
        sbr_persistent_mem_v: *mut core::ffi::c_void,
        ptr_sbr_tables: *mut ia_sbr_tables_struct,
        ptr_common_tables: *mut ixheaacd_misc_tables,
    ) -> VOID;
    fn ixheaacd_adts_crc_check_crc(
        ptr_adts_crc_info: *mut ia_adts_crc_info_struct,
    ) -> WORD32;
    fn ixheaacd_dec_ind_coupling(
        p_obj_enhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
        coup_ch_output: *mut WORD32,
        frame_size: WORD16,
        total_channels: WORD,
        ptr_time_data: *mut core::ffi::c_void,
    ) -> IA_ERRORCODE;
    fn ixheaacd_dec_downmix_to_stereo(
        p_obj_enhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
        frame_size: WORD16,
        total_elements: WORD,
        ptr_time_data: *mut WORD16,
        total_channels: WORD,
    );
    fn ixheaacd_dec_main(
        handle: *mut core::ffi::c_void,
        inbuffer: *mut WORD8,
        outbuffer: *mut WORD8,
        out_bytes: *mut WORD32,
        frames_done: WORD32,
        pcmsize: WORD32,
        num_channel_out: *mut WORD32,
    ) -> WORD32;
    fn ixheaacd_init_sbr(
        sample_rate_dec: WORD32,
        samp_per_frame: WORD32,
        down_sample_flag: *mut FLAG,
        sbr_persistent_mem_v: *mut core::ffi::c_void,
        ptr_overlap_buf: *mut WORD32,
        channel: WORD,
        ps_enable: WORD,
        sbr_ratio_idx: WORD,
        output_frame_size: WORD,
        use_hbe: *mut WORD,
        p_usac_dflt_header: *mut core::ffi::c_void,
        str_sbr_config: ia_sbr_header_data_struct,
        audio_object_type: WORD,
        ldmps_present: WORD32,
        ldsbr_present: WORD32,
    ) -> ia_handle_sbr_dec_inst_struct;
    fn ixheaacd_ld_mps_apply(
        p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
        output_buf: *mut WORD16,
    ) -> WORD32;
    fn ixheaacd_extension_payload(
        it_bit_buff: *mut ia_bit_buf_struct,
        cnt: *mut WORD32,
        self_0: *mut ia_mps_dec_state_struct,
    ) -> WORD32;
    fn ixheaacd_heaac_mps_apply(
        self_0: *mut ia_exhaacplus_dec_api_struct,
        output_buf: *mut WORD16,
        mps_buffer: *mut UWORD8,
        mps_bytes: WORD32,
    ) -> IA_ERRORCODE;
    static ixheaacd_mps_dec_m1_m2_tables: ia_mps_dec_m1_m2_tables_struct;
    static ixheaacd_mps_dec_decorr_tables: ia_mps_dec_decorr_tables_struct;
    static ixheaacd_mps_dec_tp_process_tables: ia_mps_dec_tp_process_tables_struct;
    static ixheaacd_mps_dec_mdct2qmf_table: ia_mps_dec_mdct2qmf_table_struct;
    static ixheaacd_mps_dec_tonality_tables: ia_mps_dec_tonality_tables_struct;
    static mut ixheaacd_mps_dec_bitdec_tables: ia_mps_dec_bitdec_tables_struct;
    static ixheaacd_mps_dec_blind_tables: ia_mps_dec_blind_tables_struct;
    static ixheaacd_mps_dec_mdct2qmf_tables: ia_mps_dec_mdct2qmf_tables_struct;
    static ixheaacd_mps_dec_qmf_tables: ia_mps_dec_qmf_tables_struct;
    static ixheaacd_mps_dec_common_tables: ia_mps_dec_common_tables_struct;
    static ixheaacd_mps_dec_hybrid_tables: ia_mps_dec_hybrid_tables_struct;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [core::ffi::c_ulong; 16],
}
pub type WORD8 = core::ffi::c_schar;
pub type pWORD8 = *mut core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type pUWORD8 = *mut core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type pWORD16 = *mut core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type pWORD32 = *mut core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type pUWORD32 = *mut core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type FLOAT64 = core::ffi::c_double;
pub type VOID = ();
pub type pVOID = *mut core::ffi::c_void;
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
pub type IA_ERRORCODE = WORD32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mem_info_struct {
    pub ui_size: UWORD32,
    pub ui_alignment: UWORD32,
    pub ui_type: UWORD32,
    pub ui_placement: [UWORD32; 2],
    pub ui_priority: UWORD32,
    pub ui_placed: [UWORD32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_lib_info_struct {
    pub p_lib_name: *mut WORD8,
    pub p_version_num: *mut WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_qmf_ana_filter_bank {
    pub p_filter_ana: *const WORD32,
    pub ref_co_eff_ptr_l: *const WORD32,
    pub ref_co_eff_ptr_r: *const WORD32,
    pub offset_l: WORD32,
    pub offset_r: WORD32,
    pub qmf_states_buffer: *mut WORD32,
    pub flag: WORD16,
    pub offset: WORD16,
    pub qmf_states_curr_pos: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_qmf_syn_filter_bank {
    pub p_filter_syn: *const WORD32,
    pub sbr_qmf_states_synthesis: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_qmf_tables_struct {
    pub ia_mps_enc_qmf_64_640: [WORD32; 650],
    pub sbr_alt_sin_twiddle: [WORD16; 33],
    pub sbr_cos_twiddle: [WORD16; 32],
    pub sbr_sin_twiddle: [WORD16; 32],
    pub fft_c: [WORD16; 4],
    pub ia_qmf_anl_addt_cos: [WORD16; 32],
    pub ia_qmf_anl_addt_sin: [WORD16; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_common_tables_struct {
    pub sqrt_tab: [WORD32; 513],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_hybrid_tables_struct {
    pub p8_13: [WORD16; 19],
    pub p2_6: [WORD16; 6],
    pub sine_array: [WORD32; 2048],
    pub cosine_array: [WORD32; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_index_tables_struct {
    pub row_2_channel_stp: [[WORD32; 8]; 7],
    pub row_2_channel_ges: [[WORD32; 8]; 7],
    pub row_2_residual: [[WORD32; 8]; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_m1_m2_tables_struct {
    pub idx_table: ia_mps_dec_index_tables_struct,
    pub hybrid_2_param_28: [WORD32; 71],
    pub r1_matrix_l: [WORD32; 31],
    pub ten_cld_by_10: [WORD32; 31],
    pub w00_cld2_15: [WORD32; 31],
    pub table_kappa: [WORD32; 8],
    pub dec_pow: [WORD32; 31],
    pub cld_tab_1: [WORD32; 31],
    pub cld_tab_2: [WORD32; 31],
    pub cld_tab_3: [WORD32; 31],
    pub reciprocal: [WORD32; 576],
    pub c_l_table: [WORD32; 31],
    pub cos_table: [[WORD32; 31]; 16],
    pub sin_table: [[WORD32; 31]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_rev_tables_struct {
    pub rev_delay: [[WORD32; 10]; 4],
    pub rev_split_freq_0: [WORD32; 4],
    pub rev_split_freq_1: [WORD32; 4],
    pub rev_split_freq_2: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_decorr_tables_struct {
    pub rev_table: ia_mps_dec_rev_tables_struct,
    pub lattice_coeff_0: [[WORD32; 20]; 10],
    pub lattice_coeff_1: [[WORD32; 15]; 10],
    pub lattice_coeff_2: [[WORD32; 6]; 10],
    pub lattice_coeff_3: [[WORD32; 3]; 10],
    pub den_coef_0: [[WORD32; 21]; 10],
    pub den_coef_1: [[WORD32; 16]; 10],
    pub den_coef_2: [[WORD32; 7]; 10],
    pub den_coef_3: [[WORD32; 4]; 10],
    pub lattice_delta_phi: [[WORD32; 20]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_tp_process_tables_struct {
    pub bp: [WORD32; 25],
    pub bpxgf: [WORD32; 25],
    pub bp2xgf2: [WORD32; 25],
    pub ia_mps_dec_qmf_64_640: [WORD32; 325],
    pub time_out_idx_5xxx: [WORD32; 6],
    pub time_out_idx_7xxx: [WORD32; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_wf_ptr_table_struct {
    pub wf: [*const WORD32; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mdct2qmf_table_struct {
    pub twi_post_cos: [WORD32; 64],
    pub twi_post_sin: [WORD32; 64],
    pub hybrid_2_qmf: [WORD32; 71],
    pub local_sin_4: [WORD32; 4],
    pub local_sin_15: [WORD32; 16],
    pub local_sin_16: [WORD32; 16],
    pub local_sin_18: [WORD32; 18],
    pub local_sin_24: [WORD32; 24],
    pub local_sin_30: [WORD32; 30],
    pub local_sin_32: [WORD32; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_tonality_tables_struct {
    pub gmax_fix: [WORD16; 72],
    pub dwin_fix: [WORD32; 72],
    pub nstart_fix: [[WORD32; 72]; 5],
    pub dfrac_fix: [[WORD32; 56]; 5],
    pub part4: [WORD32; 4],
    pub part5: [WORD32; 5],
    pub part7: [WORD32; 7],
    pub part10: [WORD32; 10],
    pub part14: [WORD32; 14],
    pub part20: [WORD32; 20],
    pub part28: [WORD32; 28],
    pub part40: [WORD32; 40],
    pub w_real: [WORD32; 16],
    pub w_imag: [WORD32; 16],
    pub bitrev: [WORD32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_kernels_table_struct {
    pub kernels_4_to_71: [WORD32; 71],
    pub kernels_5_to_71: [WORD32; 71],
    pub kernels_7_to_71: [WORD32; 71],
    pub kernels_10_to_71: [WORD32; 71],
    pub kernels_14_to_71: [WORD32; 71],
    pub kernels_20_to_71: [WORD32; 71],
    pub kernels_28_to_71: [WORD32; 71],
    pub bb_env_kernels: [WORD32; 71],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mapping_table_struct {
    pub mapping_4_to_28: [WORD32; 28],
    pub mapping_5_to_28: [WORD32; 28],
    pub mapping_7_to_28: [WORD32; 28],
    pub mapping_10_to_28: [WORD32; 28],
    pub mapping_14_to_28: [WORD32; 28],
    pub mapping_20_to_28: [WORD32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_tree_properties_struct {
    pub num_input_channels: WORD32,
    pub num_output_channels: WORD32,
    pub num_ott_boxes: WORD32,
    pub num_ttt_boxes: WORD32,
    pub ott_mode_lfe: [WORD32; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_bitdec_tables_struct {
    pub kernel_table: ia_mps_dec_kernels_table_struct,
    pub map_table: ia_mps_dec_mapping_table_struct,
    pub tree_property_table: [ia_mps_dec_tree_properties_struct; 7],
    pub sampling_freq_table: [WORD32; 15],
    pub freq_res_table: [WORD32; 8],
    pub temp_shape_chan_table: [[WORD32; 7]; 2],
    pub surround_gain_table: [WORD32; 5],
    pub lfe_gain_table: [WORD32; 5],
    pub clip_gain_table: [WORD32; 8],
    pub pb_stride_table: [WORD32; 4],
    pub smg_time_table: [WORD32; 4],
    pub dequant_cld: [WORD32; 31],
    pub dequant_cld_coarse: [WORD32; 15],
    pub dequant_cpc: [WORD32; 52],
    pub dequant_cpc_coarse: [WORD32; 26],
    pub dequant_icc: [WORD32; 8],
    pub factor_cld_tab_1: [WORD32; 31],
    pub hrtf_power: [WORD32; 64],
    pub envshape_data: [[WORD32; 5]; 2],
    pub pcm_chnksz_level_3: [WORD32; 5],
    pub pcm_chnksz_level_4: WORD32,
    pub pcm_chnksz_level_7: [WORD32; 6],
    pub pcm_chnksz_level_8: WORD32,
    pub pcm_chnksz_level_11: [WORD32; 2],
    pub pcm_chnksz_level_13: [WORD32; 4],
    pub pcm_chnksz_level_15: WORD32,
    pub pcm_chnksz_level_19: [WORD32; 4],
    pub pcm_chnksz_level_25: [WORD32; 3],
    pub pcm_chnksz_level_26: WORD32,
    pub pcm_chnksz_level_31: WORD32,
    pub pcm_chnksz_level_51: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mesh_tables_struct {
    pub blind_cld_mesh: [[WORD32; 21]; 31],
    pub blind_icc_mesh: [[WORD32; 21]; 31],
    pub blind_cpc_1_mesh: [[WORD32; 21]; 31],
    pub blind_cpc_2_mesh: [[WORD32; 21]; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_blind_tables_struct {
    pub mesh_table: ia_mps_dec_mesh_tables_struct,
    pub exp_1: [WORD32; 13],
    pub exp_2: [WORD32; 13],
    pub exp_4: [WORD32; 13],
    pub exp_8: [WORD32; 13],
    pub exp_16: [WORD32; 13],
    pub exp_32: [WORD32; 13],
    pub exp_64: [WORD32; 13],
    pub exp_128: [WORD32; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mdct2qmf_cos_table_struct {
    pub cos_table_long: [*mut WORD16; 64],
    pub cos_table_short: [*mut WORD16; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mdct2qmf_tables_struct {
    pub cos_table_long_32_00: [WORD16; 32],
    pub cos_table_long_32_01: [WORD16; 32],
    pub cos_table_long_32_02: [WORD16; 32],
    pub cos_table_long_32_03: [WORD16; 32],
    pub cos_table_long_32_04: [WORD16; 32],
    pub cos_table_long_32_05: [WORD16; 32],
    pub cos_table_long_32_06: [WORD16; 32],
    pub cos_table_long_32_07: [WORD16; 32],
    pub cos_table_long_32_08: [WORD16; 32],
    pub cos_table_long_32_09: [WORD16; 32],
    pub cos_table_long_32_10: [WORD16; 32],
    pub cos_table_long_32_11: [WORD16; 32],
    pub cos_table_long_32_12: [WORD16; 32],
    pub cos_table_long_32_13: [WORD16; 32],
    pub cos_table_long_32_14: [WORD16; 32],
    pub cos_table_long_32_15: [WORD16; 32],
    pub cos_table_long_32_16: [WORD16; 32],
    pub cos_table_long_32_17: [WORD16; 32],
    pub cos_table_long_32_18: [WORD16; 32],
    pub cos_table_long_32_19: [WORD16; 32],
    pub cos_table_long_32_20: [WORD16; 32],
    pub cos_table_long_32_21: [WORD16; 32],
    pub cos_table_long_32_22: [WORD16; 32],
    pub cos_table_long_32_23: [WORD16; 32],
    pub cos_table_long_32_24: [WORD16; 32],
    pub cos_table_long_32_25: [WORD16; 32],
    pub cos_table_long_32_26: [WORD16; 32],
    pub cos_table_long_32_27: [WORD16; 32],
    pub cos_table_long_32_28: [WORD16; 32],
    pub cos_table_long_32_29: [WORD16; 32],
    pub cos_table_long_32_30: [WORD16; 32],
    pub cos_table_long_32_31: [WORD16; 32],
    pub cos_table_long_30_00: [WORD16; 30],
    pub cos_table_long_30_01: [WORD16; 30],
    pub cos_table_long_30_02: [WORD16; 30],
    pub cos_table_long_30_03: [WORD16; 30],
    pub cos_table_long_30_04: [WORD16; 30],
    pub cos_table_long_30_05: [WORD16; 30],
    pub cos_table_long_30_06: [WORD16; 30],
    pub cos_table_long_30_07: [WORD16; 30],
    pub cos_table_long_30_08: [WORD16; 30],
    pub cos_table_long_30_09: [WORD16; 30],
    pub cos_table_long_30_10: [WORD16; 30],
    pub cos_table_long_30_11: [WORD16; 30],
    pub cos_table_long_30_12: [WORD16; 30],
    pub cos_table_long_30_13: [WORD16; 30],
    pub cos_table_long_30_14: [WORD16; 30],
    pub cos_table_long_30_15: [WORD16; 30],
    pub cos_table_long_30_16: [WORD16; 30],
    pub cos_table_long_30_17: [WORD16; 30],
    pub cos_table_long_30_18: [WORD16; 30],
    pub cos_table_long_30_19: [WORD16; 30],
    pub cos_table_long_30_20: [WORD16; 30],
    pub cos_table_long_30_21: [WORD16; 30],
    pub cos_table_long_30_22: [WORD16; 30],
    pub cos_table_long_30_23: [WORD16; 30],
    pub cos_table_long_30_24: [WORD16; 30],
    pub cos_table_long_30_25: [WORD16; 30],
    pub cos_table_long_30_26: [WORD16; 30],
    pub cos_table_long_30_27: [WORD16; 30],
    pub cos_table_long_30_28: [WORD16; 30],
    pub cos_table_long_30_29: [WORD16; 30],
    pub cos_table_long_24_00: [WORD16; 24],
    pub cos_table_long_24_01: [WORD16; 24],
    pub cos_table_long_24_02: [WORD16; 24],
    pub cos_table_long_24_03: [WORD16; 24],
    pub cos_table_long_24_04: [WORD16; 24],
    pub cos_table_long_24_05: [WORD16; 24],
    pub cos_table_long_24_06: [WORD16; 24],
    pub cos_table_long_24_07: [WORD16; 24],
    pub cos_table_long_24_08: [WORD16; 24],
    pub cos_table_long_24_09: [WORD16; 24],
    pub cos_table_long_24_10: [WORD16; 24],
    pub cos_table_long_24_11: [WORD16; 24],
    pub cos_table_long_24_12: [WORD16; 24],
    pub cos_table_long_24_13: [WORD16; 24],
    pub cos_table_long_24_14: [WORD16; 24],
    pub cos_table_long_24_15: [WORD16; 24],
    pub cos_table_long_24_16: [WORD16; 24],
    pub cos_table_long_24_17: [WORD16; 24],
    pub cos_table_long_24_18: [WORD16; 24],
    pub cos_table_long_24_19: [WORD16; 24],
    pub cos_table_long_24_20: [WORD16; 24],
    pub cos_table_long_24_21: [WORD16; 24],
    pub cos_table_long_24_22: [WORD16; 24],
    pub cos_table_long_24_23: [WORD16; 24],
    pub cos_table_long_18_00: [WORD16; 18],
    pub cos_table_long_18_01: [WORD16; 18],
    pub cos_table_long_18_02: [WORD16; 18],
    pub cos_table_long_18_03: [WORD16; 18],
    pub cos_table_long_18_04: [WORD16; 18],
    pub cos_table_long_18_05: [WORD16; 18],
    pub cos_table_long_18_06: [WORD16; 18],
    pub cos_table_long_18_07: [WORD16; 18],
    pub cos_table_long_18_08: [WORD16; 18],
    pub cos_table_long_18_09: [WORD16; 18],
    pub cos_table_long_18_10: [WORD16; 18],
    pub cos_table_long_18_11: [WORD16; 18],
    pub cos_table_long_18_12: [WORD16; 18],
    pub cos_table_long_18_13: [WORD16; 18],
    pub cos_table_long_18_14: [WORD16; 18],
    pub cos_table_long_18_15: [WORD16; 18],
    pub cos_table_long_18_16: [WORD16; 18],
    pub cos_table_long_18_17: [WORD16; 18],
    pub cos_table_long_16_00: [WORD16; 16],
    pub cos_table_long_16_01: [WORD16; 16],
    pub cos_table_long_16_02: [WORD16; 16],
    pub cos_table_long_16_03: [WORD16; 16],
    pub cos_table_long_16_04: [WORD16; 16],
    pub cos_table_long_16_05: [WORD16; 16],
    pub cos_table_long_16_06: [WORD16; 16],
    pub cos_table_long_16_07: [WORD16; 16],
    pub cos_table_long_16_08: [WORD16; 16],
    pub cos_table_long_16_09: [WORD16; 16],
    pub cos_table_long_16_10: [WORD16; 16],
    pub cos_table_long_16_11: [WORD16; 16],
    pub cos_table_long_16_12: [WORD16; 16],
    pub cos_table_long_16_13: [WORD16; 16],
    pub cos_table_long_16_14: [WORD16; 16],
    pub cos_table_long_16_15: [WORD16; 16],
    pub cos_table_long_15_00: [WORD16; 15],
    pub cos_table_long_15_01: [WORD16; 15],
    pub cos_table_long_15_02: [WORD16; 15],
    pub cos_table_long_15_03: [WORD16; 15],
    pub cos_table_long_15_04: [WORD16; 15],
    pub cos_table_long_15_05: [WORD16; 15],
    pub cos_table_long_15_06: [WORD16; 15],
    pub cos_table_long_15_07: [WORD16; 15],
    pub cos_table_long_15_08: [WORD16; 15],
    pub cos_table_long_15_09: [WORD16; 15],
    pub cos_table_long_15_10: [WORD16; 15],
    pub cos_table_long_15_11: [WORD16; 15],
    pub cos_table_long_15_12: [WORD16; 15],
    pub cos_table_long_15_13: [WORD16; 15],
    pub cos_table_long_15_14: [WORD16; 15],
    pub cos_table_short_4_00: [WORD16; 4],
    pub cos_table_short_4_01: [WORD16; 4],
    pub cos_table_short_4_02: [WORD16; 4],
    pub cos_table_short_4_03: [WORD16; 4],
    pub cos_table_short_3_00: [WORD16; 3],
    pub cos_table_short_3_01: [WORD16; 3],
    pub cos_table_short_3_02: [WORD16; 3],
    pub cos_table_short_2_00: [WORD16; 2],
    pub cos_table_short_2_01: [WORD16; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_res_block_tables_struct {
    pub pow_table_q17: [WORD32; 129],
    pub scale_table: [WORD32; 4],
    pub scale_table_960: [WORD32; 4],
    pub tns_max_bands_tbl: [[WORD8; 2]; 12],
    pub tns_coeff3_16: [WORD16; 8],
    pub tns_coeff4_16: [WORD16; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_res_huffmann_tables_struct {
    pub sfb_96_1024: [WORD8; 43],
    pub sfb_96_128: [WORD8; 14],
    pub sfb_64_1024: [WORD8; 49],
    pub sfb_48_1024: [WORD8; 51],
    pub sfb_48_128: [WORD8; 16],
    pub sfb_32_1024: [WORD8; 53],
    pub sfb_24_1024: [WORD8; 49],
    pub sfb_24_128: [WORD8; 17],
    pub sfb_16_1024: [WORD8; 45],
    pub sfb_16_128: [WORD8; 17],
    pub sfb_8_1024: [WORD8; 42],
    pub sfb_8_128: [WORD8; 17],
    pub sfb_96_960: [WORD8; 41],
    pub sfb_96_120: [WORD8; 13],
    pub sfb_64_960: [WORD8; 47],
    pub sfb_48_960: [WORD8; 50],
    pub sfb_48_120: [WORD8; 15],
    pub sfb_24_960: [WORD8; 47],
    pub sfb_24_120: [WORD8; 16],
    pub sfb_16_960: [WORD8; 43],
    pub sfb_16_120: [WORD8; 16],
    pub sfb_8_960: [WORD8; 41],
    pub sfb_8_120: [WORD8; 16],
    pub huffman_code_book_1: [UWORD16; 108],
    pub huffman_code_book_2: [UWORD16; 110],
    pub huffman_code_book_3: [UWORD16; 136],
    pub huffman_code_book_4: [UWORD16; 116],
    pub huffman_code_book_5: [UWORD16; 126],
    pub huffman_code_book_6: [UWORD16; 120],
    pub huffman_code_book_7: [UWORD16; 112],
    pub huffman_code_book_8: [UWORD16; 92],
    pub huffman_code_book_9: [UWORD16; 236],
    pub huffman_code_book_10: [UWORD16; 218],
    pub huffman_codebook_11: [UWORD16; 344],
    pub huffman_code_book_scl: [UWORD16; 273],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_aac_tables_struct {
    pub res_block_tables_ptr: *mut ia_mps_dec_res_block_tables_struct,
    pub res_huffmann_tables_ptr: *mut ia_mps_dec_res_huffmann_tables_struct,
    pub scale_factor_bands_long: [*mut WORD8; 24],
    pub scale_factor_bands_short: [*mut WORD8; 24],
    pub sfb_index_long: *mut WORD16,
    pub sfb_index_short: *mut WORD16,
    pub sfb_index_long_width: *mut WORD8,
    pub sfb_index_short_width: *mut WORD8,
    pub code_book: [*mut UWORD16; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sampling_rate_info_struct {
    pub sampling_frequency: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_dynamic_data_struct {
    pub a_scale_factor: [WORD16; 128],
    pub a_code_book: [WORD8; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_ics_info_struct {
    pub window_sequence: WORD16,
    pub max_sf_bands: WORD16,
    pub total_sf_bands: WORD16,
    pub sampling_rate_index: WORD16,
    pub window_groups: WORD16,
    pub window_group_length: [WORD8; 8],
    pub frame_length: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_filter_struct {
    pub start_band: WORD16,
    pub stop_band: WORD16,
    pub direction: WORD8,
    pub resolution: WORD8,
    pub order: WORD8,
    pub coeff: [WORD8; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_tns_data {
    pub tns_data_present: FLAG,
    pub number_of_filters: [WORD8; 8],
    pub filter: [[ia_mps_dec_residual_filter_struct; 3]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_pulse_data_struct {
    pub pulse_data_present: FLAG,
    pub number_pulse: WORD16,
    pub pulse_start_band: WORD16,
    pub pulse_offset: [WORD8; 4],
    pub pulse_amp: [WORD8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_pns_data_struct {
    pub pns_used: [UWORD8; 128],
    pub current_energy: WORD16,
    pub pns_active: UWORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_channel_info_struct {
    pub p_scale_factor: *mut WORD16,
    pub p_code_book: *mut WORD8,
    pub p_spectral_coefficient: *mut WORD32,
    pub ics_info: ia_mps_dec_residual_ics_info_struct,
    pub tns_data: ia_mps_dec_residual_tns_data,
    pub pulse_data: ia_mps_dec_residual_pulse_data_struct,
    pub pns_data: ia_mps_dec_residual_pns_data_struct,
    pub common_window: WORD16,
    pub global_gain: WORD16,
    pub p_tns_scratch: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_residual_sfband_info_struct {
    pub sfb_long_idx: [WORD16; 52],
    pub sfb_short_idx: [WORD16; 16],
}
pub type __jmp_buf = [core::ffi::c_long; 8];
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
pub type AUDIO_OBJECT_TYPE = core::ffi::c_uint;
pub const AOT_USAC: AUDIO_OBJECT_TYPE = 42;
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
pub const AOT_ESC: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_31: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_30: AUDIO_OBJECT_TYPE = 30;
pub const AOT_PS: AUDIO_OBJECT_TYPE = 29;
pub const AOT_RSVD_28: AUDIO_OBJECT_TYPE = 28;
pub const AOT_ER_PARA: AUDIO_OBJECT_TYPE = 27;
pub const AOT_ER_HILN: AUDIO_OBJECT_TYPE = 26;
pub const AOT_ER_HVXC: AUDIO_OBJECT_TYPE = 25;
pub const AOT_ER_CELP: AUDIO_OBJECT_TYPE = 24;
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_ER_BSAC: AUDIO_OBJECT_TYPE = 22;
pub const AOT_ER_TWIN_VQ: AUDIO_OBJECT_TYPE = 21;
pub const AOT_ER_AAC_SCAL: AUDIO_OBJECT_TYPE = 20;
pub const AOT_ER_AAC_LTP: AUDIO_OBJECT_TYPE = 19;
pub const AOT_RSVD_18: AUDIO_OBJECT_TYPE = 18;
pub const AOT_ER_AAC_LC: AUDIO_OBJECT_TYPE = 17;
pub const AOT_ALG_SYNTH_AUD_FX: AUDIO_OBJECT_TYPE = 16;
pub const AOT_GEN_MIDI: AUDIO_OBJECT_TYPE = 15;
pub const AOT_WAV_TAB_SYNTH: AUDIO_OBJECT_TYPE = 14;
pub const AOT_MAIN_SYNTH: AUDIO_OBJECT_TYPE = 13;
pub const AOT_TTSI: AUDIO_OBJECT_TYPE = 12;
pub const AOT_RSVD_11: AUDIO_OBJECT_TYPE = 11;
pub const AOT_RSVD_10: AUDIO_OBJECT_TYPE = 10;
pub const AOT_HVXC: AUDIO_OBJECT_TYPE = 9;
pub const AOT_CELP: AUDIO_OBJECT_TYPE = 8;
pub const AOT_TWIN_VQ: AUDIO_OBJECT_TYPE = 7;
pub const AOT_AAC_SCAL: AUDIO_OBJECT_TYPE = 6;
pub const AOT_SBR: AUDIO_OBJECT_TYPE = 5;
pub const AOT_AAC_LTP: AUDIO_OBJECT_TYPE = 4;
pub const AOT_AAC_SSR: AUDIO_OBJECT_TYPE = 3;
pub const AOT_AAC_LC: AUDIO_OBJECT_TYPE = 2;
pub const AOT_AAC_MAIN: AUDIO_OBJECT_TYPE = 1;
pub const AOT_NULL_OBJECT: AUDIO_OBJECT_TYPE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_adts_header_struct {
    pub sync_word: WORD16,
    pub id: WORD32,
    pub layer: WORD32,
    pub protection_absent: WORD32,
    pub profile: WORD32,
    pub samp_freq_index: WORD32,
    pub channel_configuration: WORD32,
    pub aac_frame_length: WORD16,
    pub no_raw_data_blocks: WORD32,
    pub crc_check: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_block_tables_struct {
    pub ixheaacd_pow_table_Q13: [WORD32; 129],
    pub scale_table: [WORD32; 4],
    pub tns_max_bands_tbl: [[WORD8; 2]; 12],
    pub tns_coeff3_16: [WORD16; 8],
    pub tns_coeff4_16: [WORD16; 16],
    pub scale_mant_tab: [WORD32; 4],
    pub tns_coeff3: [WORD32; 8],
    pub tns_coeff4: [WORD32; 16],
    pub tns_coeff3_32: [WORD32; 8],
    pub tns_coeff4_32: [WORD32; 16],
    pub tns_max_bands_tbl_usac: [[WORD32; 2]; 16],
    pub tns_max_bands_tbl_ld: [WORD8; 12],
    pub tns_max_bands_tbl_480: [WORD8; 12],
    pub scale_table_960: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_huffman_tables_struct {
    pub ixheaacd_sfb_96_1024: [WORD8; 43],
    pub ixheaacd_sfb_96_128: [WORD8; 14],
    pub ixheaacd_sfb_64_1024: [WORD8; 49],
    pub ixheaacd_sfb_48_1024: [WORD8; 51],
    pub ixheaacd_sfb_48_128: [WORD8; 16],
    pub ixheaacd_sfb_32_1024: [WORD8; 53],
    pub ixheaacd_sfb_24_1024: [WORD8; 49],
    pub ixheaacd_sfb_24_128: [WORD8; 17],
    pub ixheaacd_sfb_16_1024: [WORD8; 45],
    pub ixheaacd_sfb_16_128: [WORD8; 17],
    pub ixheaacd_sfb_8_1024: [WORD8; 42],
    pub ixheaacd_sfb_8_128: [WORD8; 17],
    pub str_sample_rate_info: [ia_sampling_rate_info_struct; 13],
    pub idx_table_hf11: [UWORD32; 21],
    pub idx_table_hf10: [UWORD32; 20],
    pub idx_table_hf9: [UWORD32; 23],
    pub idx_table_hf8: [UWORD32; 17],
    pub idx_table_hf7: [UWORD32; 18],
    pub idx_table_hf6: [UWORD32; 17],
    pub idx_table_hf5: [UWORD32; 19],
    pub idx_table_hf4: [UWORD32; 19],
    pub idx_table_hf3: [UWORD32; 27],
    pub idx_table_hf2: [UWORD32; 16],
    pub idx_table_hf1: [UWORD32; 12],
    pub input_table_cb11: [UWORD16; 290],
    pub input_table_cb10: [UWORD16; 170],
    pub input_table_cb9: [UWORD16; 170],
    pub input_table_cb8: [UWORD16; 65],
    pub input_table_cb7: [UWORD16; 65],
    pub input_table_cb6: [UWORD16; 82],
    pub input_table_cb5: [UWORD16; 82],
    pub input_table_cb4: [UWORD16; 82],
    pub input_table_cb3: [UWORD16; 82],
    pub input_table_cb2: [UWORD16; 82],
    pub input_table_cb1: [UWORD16; 82],
    pub huffman_code_book_scl: [UWORD16; 122],
    pub huffman_code_book_scl_index: [UWORD32; 33],
    pub ixheaacd_sfb_48_512: [WORD8; 37],
    pub ixheaacd_sfb_32_512: [WORD8; 38],
    pub ixheaacd_sfb_24_512: [WORD8; 32],
    pub ixheaacd_sfb_48_480: [WORD8; 36],
    pub ixheaacd_sfb_32_480: [WORD8; 38],
    pub ixheaacd_sfb_24_480: [WORD8; 31],
    pub ixheaacd_sfb_96_960: [WORD8; 41],
    pub ixheaacd_sfb_96_120: [WORD8; 13],
    pub ixheaacd_sfb_64_960: [WORD8; 47],
    pub ixheaacd_sfb_48_960: [WORD8; 50],
    pub ixheaacd_sfb_48_120: [WORD8; 15],
    pub ixheaacd_sfb_24_960: [WORD8; 47],
    pub ixheaacd_sfb_24_120: [WORD8; 16],
    pub ixheaacd_sfb_16_960: [WORD8; 43],
    pub ixheaacd_sfb_16_120: [WORD8; 16],
    pub ixheaacd_sfb_8_960: [WORD8; 41],
    pub ixheaacd_sfb_8_120: [WORD8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_imdct_tables_struct {
    pub cosine_array_2048_256: [WORD16; 514],
    pub dig_rev_table8_long: [WORD8; 64],
    pub dig_rev_table8_short: [WORD8; 8],
    pub fft_twiddle: [WORD32; 448],
    pub only_long_window_sine: [WORD16; 1024],
    pub only_long_window_kbd: [WORD16; 1024],
    pub only_short_window_sine: [WORD16; 128],
    pub only_short_window_kbd: [WORD16; 128],
    pub cosine_array_2048_256p: [WORD16; 514],
    pub w1024: [WORD32; 768],
    pub bit_rev_1024: [UWORD8; 256],
    pub bit_rev_512: [UWORD8; 64],
    pub bit_rev_128: [UWORD8; 16],
    pub bit_rev_32: [UWORD8; 4],
    pub w_256: [WORD32; 504],
    pub low_overlap_win: [WORD32; 512],
    pub window_sine_512: [WORD32; 512],
    pub cosine_array_1024: [WORD32; 512],
    pub low_overlap_win_480: [WORD32; 480],
    pub window_sine_480: [WORD32; 480],
    pub re_arr_tab_16: [UWORD8; 240],
    pub re_arr_tab_sml_240: [UWORD8; 240],
    pub cosine_array_960: [WORD32; 480],
    pub w_16: [WORD32; 24],
    pub window_sine_480_eld: [WORD16; 1920],
    pub window_sine_512_eld: [WORD16; 2048],
    pub only_long_window_sine_960: [WORD16; 960],
    pub only_long_window_kbd_960: [WORD16; 960],
    pub only_short_window_sine_120: [WORD16; 120],
    pub only_short_window_kbd_120: [WORD16; 120],
    pub re_arr_tab_32: [WORD16; 480],
    pub re_arr_tab_sml: [WORD16; 16],
    pub re_arr_tab_4: [WORD16; 60],
    pub re_arr_tab_15_4: [WORD16; 60],
    pub re_arr_tab_120: [WORD16; 60],
    pub re_arr_tab_5: [WORD16; 16],
    pub re_arr_tab_3: [WORD16; 16],
    pub re_arr_tab_sml_480: [WORD16; 480],
    pub cosine_array_1920: [WORD32; 960],
    pub w_512: [WORD16; 1020],
    pub w_32: [WORD16; 60],
    pub cosine_array_240: [WORD16; 120],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_sfb_info {
    pub sfb_index: *mut WORD16,
    pub sfb_width: *mut WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_tables_struct {
    pub pstr_block_tables: *mut ia_aac_dec_block_tables_struct,
    pub pstr_huffmann_tables: *mut ia_aac_dec_huffman_tables_struct,
    pub pstr_imdct_tables: *mut ia_aac_dec_imdct_tables_struct,
    pub str_aac_sfb_info: [ia_aac_sfb_info; 4],
    pub scale_factor_bands_long: [*mut WORD8; 24],
    pub scale_factor_bands_short: [*mut WORD8; 24],
    pub sfb_long_table: [WORD16; 52],
    pub sfb_short_table: [WORD16; 16],
    pub code_book: [*mut UWORD16; 13],
    pub index_table: [*mut UWORD32; 13],
    pub scale_fac_bands_512: [*mut WORD8; 16],
    pub scale_fac_bands_480: [*mut WORD8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ec_sfb_str {
    pub num_sfb_long: WORD32,
    pub num_sfb_short: WORD32,
    pub ptr_sfb_long: *mut WORD16,
    pub ptr_sfb_short: *mut WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ec_scratch_str {
    pub prev_sfb_nrg: [WORD32; 51],
    pub pres_sfb_nrg: [WORD32; 51],
    pub spec_coeff: [WORD32; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ec_state_str {
    pub spectral_coeff: [WORD32; 1024],
    pub q_spec_coeff: [WORD16; 128],
    pub prev_frame_ok: [WORD32; 2],
    pub win_shape: UWORD8,
    pub win_shape_prev: UWORD8,
    pub win_seq: WORD32,
    pub td_frame_prev: WORD32,
    pub fac_data_present: WORD32,
    pub prev_win_group_len: UWORD8,
    pub conceal_state: WORD32,
    pub prev_core_mode: WORD32,
    pub fade_idx: WORD32,
    pub lsf4: [FLOAT32; 16],
    pub str_ec_sfb: ia_ec_sfb_str,
    pub pstr_ec_scratch: *mut ia_ec_scratch_str,
    pub str_ec_scratch: ia_ec_scratch_str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_scale_fact_struct {
    pub lb_scale: WORD16,
    pub st_lb_scale: WORD16,
    pub ov_lb_scale: WORD16,
    pub hb_scale: WORD16,
    pub ov_hb_scale: WORD16,
    pub st_syn_scale: WORD16,
    pub ps_scale: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_patch_param_struct {
    pub src_start_band: WORD16,
    pub src_end_band: WORD16,
    pub guard_start_band: WORD16,
    pub dst_start_band: WORD16,
    pub dst_end_band: WORD16,
    pub num_bands_in_patch: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_transposer_settings_struct {
    pub num_columns: WORD16,
    pub num_patches: WORD16,
    pub start_patch: WORD16,
    pub stop_patch: WORD16,
    pub bw_borders: [WORD16; 10],
    pub str_patch_param: [ia_patch_param_struct; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_hf_generator_struct {
    pub pstr_settings: *mut ia_transposer_settings_struct,
    pub bw_array_prev: [WORD32; 6],
    pub lpc_filt_states_real: [*mut WORD32; 2],
    pub lpc_filt_states_imag: [*mut WORD32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_lpp_trans_patch {
    pub num_patches: WORD32,
    pub start_subband: [WORD32; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_freq_band_data_struct {
    pub num_sf_bands: [WORD16; 2],
    pub num_nf_bands: WORD16,
    pub num_mf_bands: WORD16,
    pub sub_band_start: WORD16,
    pub sub_band_end: WORD16,
    pub freq_band_tbl_lim: [WORD16; 13],
    pub num_lf_bands: WORD16,
    pub num_if_bands: WORD16,
    pub freq_band_table: [*mut WORD16; 2],
    pub freq_band_tbl_lo: [WORD16; 29],
    pub freq_band_tbl_hi: [WORD16; 57],
    pub freq_band_tbl_noise: [WORD16; 6],
    pub f_master_tbl: [WORD16; 57],
    pub qmf_sb_prev: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_header_data_struct {
    pub sync_state: WORD32,
    pub err_flag: FLAG,
    pub err_flag_prev: FLAG,
    pub num_time_slots: WORD16,
    pub time_step: WORD16,
    pub core_frame_size: WORD16,
    pub out_sampling_freq: WORD32,
    pub channel_mode: WORD32,
    pub amp_res: WORD16,
    pub start_freq: WORD16,
    pub stop_freq: WORD16,
    pub xover_band: WORD16,
    pub freq_scale: WORD16,
    pub alter_scale: WORD16,
    pub noise_bands: WORD16,
    pub limiter_bands: WORD16,
    pub limiter_gains: WORD16,
    pub interpol_freq: WORD16,
    pub smoothing_mode: WORD16,
    pub pstr_freq_band_data: *mut ia_freq_band_data_struct,
    pub header_extra_1: WORD16,
    pub header_extra_2: WORD16,
    pub pre_proc_flag: WORD16,
    pub status: WORD32,
    pub sbr_ratio_idx: WORD32,
    pub upsamp_fac: WORD32,
    pub is_usf_4: WORD32,
    pub output_framesize: WORD32,
    pub usac_independency_flag: WORD32,
    pub pvc_flag: FLAG,
    pub hbe_flag: FLAG,
    pub esbr_start_up: WORD32,
    pub esbr_start_up_pvc: WORD32,
    pub usac_flag: WORD32,
    pub pvc_mode: UWORD8,
    pub enh_sbr: FLAG,
    pub esbr_hq: FLAG,
    pub enh_sbr_ps: FLAG,
    pub eld_sbr: FLAG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_frame_info_struct {
    pub frame_class: WORD16,
    pub num_env: WORD16,
    pub transient_env: WORD16,
    pub num_noise_env: WORD16,
    pub border_vec: [WORD16; 9],
    pub freq_res: [WORD16; 8],
    pub noise_border_vec: [WORD16; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_env_calc_tables_struct {
    pub sbr_lim_gains_m: [WORD16; 8],
    pub sbr_lim_bands_per_octave_q13: [WORD16; 4],
    pub sbr_smooth_filter: [WORD16; 4],
    pub sbr_inv_int_table: [WORD16; 49],
    pub sbr_rand_ph: [WORD32; 568],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_qmf_dec_tables_struct {
    pub w_32: [WORD16; 60],
    pub w_16: [WORD16; 24],
    pub dig_rev_table2_32: [WORD32; 4],
    pub dig_rev_table4_16: [WORD32; 2],
    pub sbr_sin_cos_twiddle_l64: [WORD16; 64],
    pub sbr_alt_sin_twiddle_l64: [WORD16; 32],
    pub sbr_cos_sin_twiddle_ds_l32: [WORD16; 64],
    pub sbr_sin_cos_twiddle_l32: [WORD16; 32],
    pub sbr_alt_sin_twiddle_l32: [WORD16; 16],
    pub sbr_t_cos_sin_l32: [WORD16; 64],
    pub post_fft_tbl: [WORD16; 18],
    pub dct23_tw: [WORD16; 66],
    pub qmf_c: [WORD16; 1280],
    pub dig_rev_table2_128: [UWORD8; 4],
    pub w1024: [WORD32; 1536],
    pub esbr_qmf_c: [WORD32; 1280],
    pub esbr_qmf_c_24: [WORD32; 480],
    pub esbr_w_32: [WORD32; 60],
    pub esbr_w_16: [WORD32; 24],
    pub esbr_sin_cos_twiddle_l64: [WORD32; 64],
    pub esbr_alt_sin_twiddle_l64: [WORD32; 32],
    pub esbr_sin_cos_twiddle_l32: [WORD32; 32],
    pub esbr_alt_sin_twiddle_l32: [WORD32; 16],
    pub esbr_t_cos_sin_l32: [WORD32; 64],
    pub esbr_sin_cos_twiddle_l24: [WORD32; 24],
    pub esbr_alt_sin_twiddle_l24: [WORD32; 12],
    pub esbr_t_cos_sin_l24: [WORD32; 48],
    pub esbr_sin_cos_twiddle_l16: [WORD32; 16],
    pub esbr_alt_sin_twiddle_l16: [WORD32; 8],
    pub esbr_t_cos_sin_l16: [WORD32; 32],
    pub ixheaacd_sbr_t_cos_sin_l32_eld: [WORD16; 64],
    pub qmf_c_eld: [WORD16; 640],
    pub qmf_c_eld2: [WORD16; 640],
    pub qmf_c_eld3: [WORD16; 640],
    pub qmf_c_ldsbr_mps: [WORD32; 640],
    pub ixheaacd_sbr_synth_cos_sin_l32: [WORD16; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_env_extr_tables_struct {
    pub sbr_frame_info1_2_4_16: [ia_frame_info_struct; 7],
    pub str_sbr_default_header: ia_sbr_header_data_struct,
    pub ixheaacd_t_huffman_env_bal_1_5db_inp_table: [WORD16; 50],
    pub ixheaacd_f_huffman_env_bal_1_5db_inp_table: [WORD16; 50],
    pub ixheaacd_t_huffman_env_bal_3_0db_inp_table: [WORD16; 26],
    pub ixheaacd_f_huffman_env_bal_3_0db_inp_table: [WORD16; 26],
    pub ixheaacd_t_huffman_noise_3_0db_inp_table: [WORD16; 64],
    pub ixheaacd_t_huffman_noise_bal_3_0db_inp_table: [WORD16; 26],
    pub ixheaacd_t_huffman_env_1_5db_inp_table: [WORD16; 122],
    pub ixheaacd_f_huffman_env_1_5db_inp_table: [WORD16; 122],
    pub ixheaacd_t_huffman_env_3_0db_inp_table: [WORD16; 64],
    pub ixheaacd_f_huffman_env_3_0db_inp_table: [WORD16; 64],
    pub ixheaacd_t_huffman_env_bal_1_5db_idx_table: [WORD32; 20],
    pub ixheaacd_f_huffman_env_bal_1_5db_idx_table: [WORD32; 23],
    pub ixheaacd_t_huffman_env_bal_3_0db_idx_table: [WORD32; 16],
    pub ixheaacd_f_huffman_env_bal_3_0db_idx_table: [WORD32; 17],
    pub ixheaacd_t_huffman_noise_3_0db_idx_table: [WORD32; 17],
    pub ixheaacd_t_huffman_noise_bal_3_0db_idx_table: [WORD32; 11],
    pub ixheaacd_t_huffman_env_1_5db_idx_table: [WORD32; 27],
    pub ixheaacd_f_huffman_env_1_5db_idx_table: [WORD32; 28],
    pub ixheaacd_t_huffman_env_3_0db_idx_table: [WORD32; 26],
    pub ixheaacd_f_huffman_env_3_0db_idx_table: [WORD32; 25],
    pub start_min: [WORD8; 12],
    pub offset_idx: [WORD8; 12],
    pub ixheaacd_drc_offset: [[WORD8; 16]; 7],
    pub stop_min: [WORD8; 12],
    pub stop_off: [[WORD8; 14]; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ps_tables_struct {
    pub decay_scale_factor: [WORD16; 72],
    pub hyb_resol: [WORD16; 3],
    pub rev_link_decay_ser: [WORD16; 3],
    pub rev_link_delay_ser: [WORD16; 3],
    pub borders_group: [WORD16; 23],
    pub group_shift: [WORD16; 6],
    pub group_to_bin: [WORD16; 22],
    pub hybrid_to_bin: [WORD16; 10],
    pub delay_to_bin: [WORD16; 32],
    pub frac_delay_phase_fac_qmf_re_im: [WORD16; 48],
    pub frac_delay_phase_fac_qmf_sub_re_im: [WORD16; 32],
    pub frac_delay_phase_fac_qmf_ser_re_im: [[WORD16; 64]; 3],
    pub frac_delay_phase_fac_qmf_sub_ser_re_im: [[WORD16; 32]; 3],
    pub scale_factors: [WORD16; 15],
    pub scale_factors_fine: [WORD16; 31],
    pub alpha_values: [WORD16; 8],
    pub p2_6: [WORD16; 6],
    pub p8_13: [WORD16; 13],
    pub huff_iid_dt: [WORD16; 28],
    pub huff_iid_df: [WORD16; 28],
    pub huff_icc_dt: [WORD16; 14],
    pub huff_icc_df: [WORD16; 14],
    pub huff_iid_dt_fine: [WORD16; 60],
    pub huff_iid_df_fine: [WORD16; 60],
    pub dummy: WORD32,
    pub qmf_fract_delay_phase_factor_im: [FLOAT32; 64],
    pub qmf_fract_delay_phase_factor_re: [FLOAT32; 64],
    pub frac_delay_phase_fac_qmf_sub_im_20: [FLOAT32; 12],
    pub frac_delay_phase_fac_qmf_sub_re_20: [FLOAT32; 12],
    pub frac_delay_phase_fac_qmf_sub_im_34: [FLOAT32; 32],
    pub frac_delay_phase_fac_qmf_sub_re_34: [FLOAT32; 32],
    pub qmf_ser_fract_delay_phase_factor_im: [[FLOAT32; 3]; 64],
    pub qmf_ser_fract_delay_phase_factor_re: [[FLOAT32; 3]; 64],
    pub frac_delay_phase_fac_ser_qmf_sub_im_20: [[FLOAT32; 3]; 12],
    pub frac_delay_phase_fac_ser_qmf_sub_re_20: [[FLOAT32; 3]; 12],
    pub frac_delay_phase_fac_ser_qmf_sub_im_34: [[FLOAT32; 3]; 32],
    pub frac_delay_phase_fac_ser_qmf_sub_re_34: [[FLOAT32; 3]; 32],
    pub scale_factors_flt: [FLOAT32; 15],
    pub scale_factors_fine_flt: [FLOAT32; 31],
    pub alphas: [FLOAT32; 8],
    pub all_pass_link_decay_ser: [FLOAT32; 3],
    pub p8_13_20: [FLOAT32; 13],
    pub p2_13_20: [FLOAT32; 13],
    pub p12_13_34: [FLOAT32; 13],
    pub p8_13_34: [FLOAT32; 13],
    pub p4_13_34: [FLOAT32; 13],
    pub cos_mod_2channel: [[FLOAT32; 13]; 2],
    pub cos_sin_mod_4channel: [[FLOAT32; 26]; 4],
    pub cos_sin_mod_8channel: [[FLOAT32; 26]; 8],
    pub cos_sin_mod_12channel: [[FLOAT32; 26]; 12],
    pub qmf_delay_idx_tbl: [WORD32; 64],
    pub group_borders_20_tbl: [WORD32; 23],
    pub group_borders_34_tbl: [WORD32; 51],
    pub bin_group_map_20: [WORD32; 22],
    pub bin_group_map_34: [WORD32; 50],
    pub quantized_iids: [WORD32; 7],
    pub quantized_iids_fine: [WORD32; 15],
    pub quantized_rhos: [FLOAT32; 8],
    pub ipd_bins_tbl: [WORD32; 3],
    pub band_res_hyb20: [WORD16; 3],
    pub band_res_hyb34: [WORD16; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_tables_struct {
    pub env_calc_tables_ptr: *mut ia_env_calc_tables_struct,
    pub qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    pub env_extr_tables_ptr: *mut ia_env_extr_tables_struct,
    pub ps_tables_ptr: *mut ia_ps_tables_struct,
    pub sbr_rand_ph: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hybrid_struct {
    pub ptr_resol: *const WORD16,
    pub ptr_qmf_buf: WORD8,
    pub ptr_work_re: *mut WORD32,
    pub ptr_work_im: *mut WORD32,
    pub ptr_qmf_buf_re: [*mut WORD32; 3],
    pub ptr_qmf_buf_im: [*mut WORD32; 3],
    pub ptr_temp_re: *mut WORD32,
    pub ptr_temp_im: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hybrid_flt_struct {
    pub num_qmf_bands: WORD32,
    pub frame_size: WORD32,
    pub ptr_resol: *mut WORD16,
    pub ptr_work_re: *mut FLOAT32,
    pub ptr_work_im: *mut FLOAT32,
    pub ptr_qmf_buf_re: *mut [FLOAT32; 12],
    pub ptr_qmf_buf_im: *mut [FLOAT32; 12],
    pub ptr_temp_re: *mut [FLOAT32; 64],
    pub ptr_temp_im: *mut [FLOAT32; 64],
}
pub type REVERB_BUFFERS_RI = *mut [[WORD16; 64]; 3];
pub type REVERB_BUFFERS_CH_RI = [[[WORD16; 32]; 3]; 5];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ps_dec_struct {
    pub delay_buf_qmf_ap_re_im: *mut [WORD16; 64],
    pub delay_buf_qmf_ld_re_im: *mut [WORD16; 24],
    pub delay_buf_qmf_sd_re_im: *mut [WORD16; 64],
    pub delay_buf_idx_ser: [WORD16; 3],
    pub delay_sample_ser: [WORD16; 3],
    pub delay_buf_qmf_ser_re_im: REVERB_BUFFERS_RI,
    pub delay_buf_idx: WORD16,
    pub delay_buf_idx_long: WORD16,
    pub peak_decay_diff: *mut WORD32,
    pub energy_prev: *mut WORD32,
    pub peak_decay_diff_prev: *mut WORD32,
    pub ptr_hyb_left_re: *mut WORD32,
    pub ptr_hyb_left_im: *mut WORD32,
    pub ptr_hyb_right_re: *mut WORD32,
    pub ptr_hyb_right_im: *mut WORD32,
    pub delay_buf_qmf_sub_re_im: [[WORD16; 32]; 2],
    pub delay_buf_qmf_sub_ser_re_im: REVERB_BUFFERS_CH_RI,
    pub h11_h12_vec: [WORD16; 48],
    pub h21_h22_vec: [WORD16; 48],
    pub H11_H12: [WORD16; 48],
    pub H21_H22: [WORD16; 48],
    pub delta_h11_h12: [WORD16; 48],
    pub delta_h21_h22: [WORD16; 48],
    pub force_mono: FLAG,
    pub delay_buffer_scale: WORD16,
    pub usb: WORD16,
    pub iid_par_prev: [WORD16; 34],
    pub icc_par_prev: [WORD16; 34],
    pub ps_data_present: FLAG,
    pub enable_iid: FLAG,
    pub enable_icc: FLAG,
    pub enable_ext: FLAG,
    pub iid_mode: WORD16,
    pub icc_mode: WORD16,
    pub iid_quant: FLAG,
    pub frame_class: FLAG,
    pub num_env: WORD16,
    pub border_position: [WORD16; 7],
    pub iid_dt: [FLAG; 5],
    pub icc_dt: [FLAG; 5],
    pub iid_par_table: [[WORD16; 34]; 7],
    pub icc_par_table: [[WORD16; 34]; 7],
    pub str_hybrid: ia_hybrid_struct,
    pub hyb_left_re: [[FLOAT32; 32]; 32],
    pub hyb_left_im: [[FLOAT32; 32]; 32],
    pub hyb_right_re: [[FLOAT32; 32]; 32],
    pub hyb_right_im: [[FLOAT32; 32]; 32],
    pub h11_re_vec: [FLOAT32; 34],
    pub h11_im_vec: [FLOAT32; 34],
    pub h12_re_vec: [FLOAT32; 34],
    pub h12_im_vec: [FLOAT32; 34],
    pub h21_re_vec: [FLOAT32; 34],
    pub h21_im_vec: [FLOAT32; 34],
    pub h22_re_vec: [FLOAT32; 34],
    pub h22_im_vec: [FLOAT32; 34],
    pub h11_re_prev: [FLOAT32; 34],
    pub h11_im_prev: [FLOAT32; 34],
    pub h12_re_prev: [FLOAT32; 34],
    pub h12_im_prev: [FLOAT32; 34],
    pub h21_re_prev: [FLOAT32; 34],
    pub h21_im_prev: [FLOAT32; 34],
    pub h22_re_prev: [FLOAT32; 34],
    pub h22_im_prev: [FLOAT32; 34],
    pub qmf_delay_buf_re: [[FLOAT32; 64]; 14],
    pub qmf_delay_buf_im: [[FLOAT32; 64]; 14],
    pub sub_qmf_delay_buf_re: [[FLOAT32; 64]; 14],
    pub sub_qmf_delay_buf_im: [[FLOAT32; 64]; 14],
    pub ser_qmf_delay_buf_re: [[[FLOAT32; 64]; 5]; 3],
    pub ser_qmf_delay_buf_im: [[[FLOAT32; 64]; 5]; 3],
    pub ptr_hybrid: *mut ia_hybrid_flt_struct,
    pub str_flt_hybrid20: ia_hybrid_flt_struct,
    pub str_flt_hybrid34: ia_hybrid_flt_struct,
    pub use_34_st_bands: WORD32,
    pub use_34_st_bands_prev: WORD32,
    pub ps_mode: WORD32,
    pub ptr_group_borders: *mut WORD32,
    pub num_groups: WORD32,
    pub num_sub_qmf_groups: WORD32,
    pub num_bins: WORD32,
    pub first_delay_gr: WORD32,
    pub ptr_bins_group_map: *mut WORD32,
    pub num_sub_samples: WORD32,
    pub num_chans: WORD32,
    pub use_pca_rot_flg: WORD32,
    pub freq_res_ipd: WORD32,
    pub delay_qmf_delay_buf_idx: [WORD32; 64],
    pub delay_qmf_delay_num_samp: [WORD32; 64],
    pub peak_decay_fast_bin: [FLOAT32; 34],
    pub prev_nrg_bin: [FLOAT32; 34],
    pub prev_peak_diff_bin: [FLOAT32; 34],
    pub ipd_idx_map_1: [WORD32; 17],
    pub opd_idx_map_1: [WORD32; 17],
    pub ipd_idx_map_2: [WORD32; 17],
    pub opd_idx_map_2: [WORD32; 17],
    pub ipd_idx_map: [[WORD32; 17]; 5],
    pub opd_idx_map: [[WORD32; 17]; 5],
    pub ser_sub_qmf_dealy_buf_re: [[[FLOAT32; 64]; 5]; 3],
    pub ser_sub_qmf_dealy_buf_im: [[[FLOAT32; 64]; 5]; 3],
    pub hyb_work_re_20: [FLOAT32; 44],
    pub hyb_work_im_20: [FLOAT32; 44],
    pub hyb_qmf_buf_re_20: [[FLOAT32; 12]; 5],
    pub hyb_qmf_buf_im_20: [[FLOAT32; 12]; 5],
    pub hyb_temp_re_20: [[FLOAT32; 64]; 32],
    pub hyb_temp_im_20: [[FLOAT32; 64]; 32],
    pub hyb_work_re_34: [FLOAT32; 44],
    pub hyb_work_im_34: [FLOAT32; 44],
    pub hyb_qmf_buf_re_34: [[FLOAT32; 12]; 5],
    pub hyb_qmf_buf_im_34: [[FLOAT32; 12]; 5],
    pub hyb_temp_re_34: [[FLOAT32; 64]; 32],
    pub hyb_temp_im_34: [[FLOAT32; 64]; 32],
    pub pp_qmf_buf_real: [*mut *mut FLOAT32; 2],
    pub pp_qmf_buf_imag: [*mut *mut FLOAT32; 2],
    pub time_sample_buf: [*mut FLOAT32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ps_dec_config_struct {
    pub enable_iid: FLAG,
    pub enable_icc: FLAG,
    pub iid_mode: WORD16,
    pub icc_mode: WORD16,
    pub frame_class: FLAG,
    pub freq_res_ipd: WORD32,
    pub border_position: [WORD16; 7],
    pub iid_dt: [FLAG; 5],
    pub icc_dt: [FLAG; 5],
    pub iid_par_table: [[WORD16; 34]; 7],
    pub icc_par_table: [[WORD16; 34]; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pulse_info_struct {
    pub pulse_data_present: FLAG,
    pub number_pulse: WORD16,
    pub pulse_start_band: WORD16,
    pub pulse_offset: [WORD8; 4],
    pub pulse_amp: [WORD8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_correlation_info_struct {
    pub correlated: [UWORD8; 16],
    pub random_vector: [WORD32; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_rand_vec_struct {
    pub current_seed: WORD32,
    pub pns_frame_number: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_info_struct {
    pub pns_used: [UWORD8; 128],
    pub noise_energy: WORD16,
    pub pns_active: UWORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaac_drc_data_struct {
    pub prog_ref_level: WORD32,
    pub n_mdct_bands: [WORD16; 16],
    pub drc_fac: [WORD16; 16],
    pub drc_fac_dvb: [WORD16; 16],
    pub drc_exp: WORD8,
    pub short_block: UWORD8,
    pub drc_interp_scheme: UWORD8,
    pub n_drc_bands: UWORD8,
    pub new_prog_ref_level: UWORD8,
    pub new_drc_fac: UWORD8,
    pub prev_interp_scheme: UWORD8,
    pub drc_factors_sbr: [[WORD32; 64]; 64],
    pub drc_factors_sbr_lat: [[WORD32; 64]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaac_drc_bs_data_struct {
    pub b_channel_on: [UWORD8; 8],
    pub prog_ref_level_present: UWORD8,
    pub prog_ref_level: UWORD8,
    pub drc_num_bands: UWORD8,
    pub drc_band_top: [UWORD8; 16],
    pub dyn_rng_dlbl: [WORD8; 16],
    pub dyn_rng_dlbl_dvb: [WORD8; 16],
    pub max_dyn_rng_dlbl: WORD8,
    pub drc_interpolation_scheme: UWORD8,
    pub drc_data_type: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_dec_struct {
    pub str_drc_bs_data: [ixheaac_drc_bs_data_struct; 10],
    pub str_drc_channel_data: [ixheaac_drc_data_struct; 10],
    pub drc_ref_level: WORD16,
    pub drc_def_level: WORD16,
    pub drc_channel_next_index: [UWORD8; 10],
    pub sbr_allowed: UWORD8,
    pub sbr_found: UWORD8,
    pub drc_element_found: UWORD8,
    pub max_audio_channels: UWORD8,
    pub length_history: UWORD8,
    pub num_drc_elements: UWORD8,
    pub state: WORD32,
    pub target_ref_level: WORD32,
    pub prog_ref_level: WORD32,
    pub cut_factor: WORD32,
    pub boost_factor: WORD32,
    pub drc_dig_norm: FLAG,
    pub drc_on: FLAG,
    pub dvb_anc_data_present: FLAG,
    pub dvb_anc_data_pos: WORD32,
    pub pres_mode: WORD32,
    pub heavy_mode: WORD32,
}
pub type C2RustUnnamed = core::ffi::c_uint;
pub const ltp_buffer_size: C2RustUnnamed = 4096;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ltp_info {
    pub last_band: UWORD8,
    pub data_present: UWORD8,
    pub lag: UWORD16,
    pub lag_update: UWORD8,
    pub coef: UWORD8,
    pub long_used: [UWORD8; 51],
    pub short_used: [UWORD8; 8],
    pub short_lag_present: [UWORD8; 8],
    pub short_lag: [UWORD8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ics_info_struct {
    pub window_shape: WORD16,
    pub window_sequence: WORD16,
    pub max_sfb: WORD16,
    pub num_swb_window: WORD16,
    pub sampling_rate_index: WORD16,
    pub num_window_groups: WORD16,
    pub window_group_length: [WORD8; 8],
    pub frame_length: WORD16,
    pub frame_size: WORD32,
    pub predictor_data_present: WORD16,
    pub ltp: ltp_info,
    pub ltp2: ltp_info,
    pub qshift_adj: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_err_config_struct {
    pub aac_sect_data_resil_flag: WORD32,
    pub aac_sf_data_resil_flag: WORD32,
    pub aac_spect_data_resil_flag: WORD32,
    pub ep_config: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eld_specific_config_struct {
    pub ld_sbr_flag_present: WORD32,
    pub ld_sbr_samp_rate: WORD32,
    pub ld_sbr_crc_flag: WORD32,
    pub ldSbrHeaderPresent: WORD32,
    pub aac_sect_data_resil_flag: WORD32,
    pub aac_sf_data_resil_flag: WORD32,
    pub aac_spect_data_resil_flag: WORD32,
    pub ep_config: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_stereo_info_struct {
    pub ms_used: [[UWORD8; 64]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_info_struct {
    pub start_band: WORD16,
    pub stop_band: WORD16,
    pub direction: WORD8,
    pub resolution: WORD8,
    pub order: WORD8,
    pub coef: [WORD8; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tns_info_aac_struct {
    pub tns_data_present: FLAG,
    pub n_filt: [WORD8; 8],
    pub str_filter: [[ia_filter_info_struct; 3]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_overlap_info {
    pub ptr_long_window: [*const WORD16; 2],
    pub ptr_short_window: [*const WORD16; 2],
    pub window_shape: WORD16,
    pub window_sequence: WORD16,
    pub ptr_overlap_buf: *mut WORD32,
    pub rvlc_prev_sf: [WORD16; 128],
    pub rvlc_prev_cb: [WORD16; 128],
    pub rvlc_prev_blk_type: WORD8,
    pub rvlc_prev_sf_ok: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_rvlc_info_struct {
    pub sf_concealment: WORD32,
    pub rev_global_gain: WORD32,
    pub rvlc_sf_len: WORD16,
    pub dpcm_noise_nrg: WORD32,
    pub sf_esc_present: WORD32,
    pub rvlc_esc_len: WORD16,
    pub dpcm_noise_last_pos: WORD32,
    pub dpcm_is_last_pos: WORD32,
    pub rvlc_sf_fwd_len: WORD16,
    pub rvlc_sf_bwd_len: WORD16,
    pub ptr_rvl_bit_cnt: *mut WORD16,
    pub ptr_rvl_bit_str_idx: *mut UWORD16,
    pub num_wind_grps: WORD16,
    pub max_sfb_transmitted: WORD16,
    pub first_noise_group: UWORD8,
    pub first_noise_band: UWORD8,
    pub direction: UWORD8,
    pub rvl_fwd_bit_str_idx: UWORD16,
    pub rvl_bwd_bit_str_idx: UWORD16,
    pub esc_bit_str_idx: UWORD16,
    pub ptr_huff_tree_rvl_cw: *const UWORD32,
    pub ptr_huff_tree_rvl_esc: *const UWORD32,
    pub num_fwd_esc_words_decoded: UWORD8,
    pub num_bwd_esc_words_decoded: UWORD8,
    pub num_esc_words_decoded: UWORD8,
    pub noise_used: WORD8,
    pub intensity_used: WORD8,
    pub sf_used: WORD8,
    pub firt_scale_fac: WORD16,
    pub last_scale_fac: WORD16,
    pub first_nrg: WORD16,
    pub last_nrg: WORD16,
    pub is_first: WORD16,
    pub is_last: WORD16,
    pub rvlc_err_log: UWORD32,
    pub conceal_min: WORD16,
    pub conceal_max: WORD16,
    pub conceal_min_esc: WORD16,
    pub conceal_max_esc: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reorder_io_struct {
    pub err_log: UWORD32,
    pub ptr_quant_spec_coeff_base: *mut WORD32,
    pub quant_spec_coeff_idx: WORD32,
    pub reordered_spec_data_len: WORD16,
    pub num_sect: WORD16,
    pub ptr_num_line_in_sect: *mut WORD16,
    pub bit_str_idx: UWORD16,
    pub longest_cw_len: WORD8,
    pub ptr_cb: *mut UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reorder_cb_pairs_struct {
    pub ptr_min_cb_pair_tbl: *const UWORD8,
    pub ptr_max_cb_pair_tbl: *const UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reorder_tbl_struct {
    pub ptr_lav_tbl: *const UWORD16,
    pub ptr_max_cw_len_tbl: *const UWORD8,
    pub ptr_cb_dimension_tbl: *const UWORD8,
    pub ptr_cb_dim_shift_tbl: *const UWORD8,
    pub ptr_cb_sign_tbl: *const UWORD8,
    pub ptr_cb_priority: *const UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reord_seg_info_struct {
    pub num_segment: WORD32,
    pub segment_offset: UWORD32,
    pub arr_temp_values: [WORD32; 1024],
    pub arr_seg_start_l: [UWORD16; 512],
    pub arr_seg_start_r: [UWORD16; 512],
    pub p_remaining_bits_in_seg: [WORD8; 512],
    pub code: [WORD32; 512],
    pub code_extra: [WORD32; 512],
    pub p_num_bits: [WORD8; 512],
    pub read_direction: UWORD8,
    pub is_decoded: [WORD32; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_huff_code_reord_sect_info_struct {
    pub num_code_word: UWORD32,
    pub current_codeword: UWORD32,
    pub num_sorted_section: UWORD32,
    pub ptr_num_cw_in_sect: [UWORD16; 256],
    pub ptr_num_sorted_cw_in_sect: [UWORD16; 256],
    pub ptr_num_ext_sorted_cw_in_sect: [UWORD16; 270],
    pub num_ext_sorted_cw_in_sect_idx: WORD32,
    pub ptr_num_ext_sorted_sect_in_sets: [UWORD16; 14],
    pub num_ext_sorted_sect_in_sets_idx: WORD32,
    pub ptr_reorder_offset: [UWORD16; 256],
    pub ptr_sorted_cb: [UWORD8; 256],
    pub ptr_ext_sorted_cw: [UWORD8; 270],
    pub ext_sorted_cw_idx: WORD32,
    pub ptr_ext_sorted_sect_max_cb_len: [UWORD8; 270],
    pub ext_sorted_sect_max_cb_len_idx: WORD32,
    pub ptr_cb_switch: [UWORD8; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hcr_non_pcw_sideinfo_struct {
    pub ptr_result_base: *mut WORD32,
    pub res_ptr_idx: [UWORD16; 256],
    pub cw_offset: UWORD32,
    pub ptr_cb: [UWORD8; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_hcr_info_struct {
    pub str_dec_io: ia_huff_code_reorder_io_struct,
    pub codebook_pairs: ia_huff_code_reorder_cb_pairs_struct,
    pub table_info: ia_huff_code_reorder_tbl_struct,
    pub str_segment_info: ia_huff_code_reord_seg_info_struct,
    pub sect_info: ia_huff_code_reord_sect_info_struct,
    pub str_non_pcw_side_info: ia_hcr_non_pcw_sideinfo_struct,
    pub global_hcr_type: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_sfb_code_book_struct {
    pub scale_factor: [WORD16; 128],
    pub code_book: [WORD8; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pns_stereo_data_struct {
    pub str_stereo_info: ia_stereo_info_struct,
    pub str_pns_corr_info: ia_pns_correlation_info_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_ola_data {
    pub win_shape: WORD16,
    pub win_seq: WORD16,
    pub ptr_overlap_buf: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_channel_info {
    pub ptr_long_window: [*const WORD16; 2],
    pub ptr_short_window: [*const WORD16; 2],
    pub overlap_add_data: ia_aac_dec_ola_data,
    pub ltp_buf: *mut WORD16,
    pub ltp_lag_1: UWORD16,
    pub ltp_lag_2: UWORD16,
    pub str_ec_state: ia_ec_state_str,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_channel_info_struct {
    pub ptr_scale_factor: *mut WORD16,
    pub ptr_code_book: *mut WORD8,
    pub ptr_spec_coeff: *mut WORD32,
    pub pstr_stereo_info: *mut ia_stereo_info_struct,
    pub pstr_pns_corr_info: *mut ia_pns_correlation_info_struct,
    pub pstr_pns_rand_vec_data: *mut ia_pns_rand_vec_struct,
    pub str_ics_info: ia_ics_info_struct,
    pub str_tns_info: ia_tns_info_aac_struct,
    pub str_pulse_info: ia_pulse_info_struct,
    pub str_pns_info: ia_pns_info_struct,
    pub common_window: WORD16,
    pub element_instance_tag: WORD16,
    pub global_gain: WORD16,
    pub scratch_buf_ptr: *mut WORD32,
    pub pulse_scratch: *mut WORD32,
    pub ptr_rvlc_info: ia_rvlc_info_struct,
    pub str_hcr_info: ia_hcr_info_struct,
    pub reorder_spect_data_len: WORD16,
    pub longest_cw_len: WORD8,
    pub rvlc_scf_esc_arr: [WORD16; 128],
    pub rvlc_scf_fwd_arr: [WORD16; 128],
    pub rvlc_scf_bwd_arr: [WORD16; 128],
    pub rvlc_intensity_used: WORD8,
    pub num_line_in_sec4_hcr_arr: [WORD16; 256],
    pub cb4_hcr_arr: [UWORD8; 256],
    pub number_sect: WORD32,
    pub granule_len: WORD32,
    pub rvlc_curr_sf_flag: WORD16,
    pub ltp_buf: *mut WORD16,
    pub ltp_lag: UWORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_element_stream_struct {
    pub sbr_ele_id: WORD32,
    pub extension_type: WORD32,
    pub size_payload: WORD32,
    pub ptr_sbr_data: *mut WORD8,
    pub ptr_prev_sbr_data: *mut WORD8,
    pub prev_size_payload: WORD32,
    pub frame_error_flag: [WORD32; 2],
    pub use_frame_slot: WORD32,
    pub prev_sbr_ele_id: WORD32,
    pub prev_extension_type: WORD32,
    pub size_payload_old: WORD32,
    pub sbr_prev_data: [WORD8; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_sbr_bitstream_struct {
    pub no_elements: WORD16,
    pub str_sbr_ele: [ia_sbr_element_stream_struct; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_dec_inst_struct {
    pub pstr_ps_stereo_dec: *mut ia_ps_dec_struct,
    pub str_ps_config_prev: ia_ps_dec_config_struct,
    pub ps_present: FLAG,
    pub pstr_sbr_channel: [*mut ia_sbr_channel_struct; 2],
    pub pstr_sbr_header: [*mut ia_sbr_header_data_struct; 2],
    pub pstr_freq_band_data: [*mut ia_freq_band_data_struct; 2],
    pub pstr_sbr_tables: *mut ia_sbr_tables_struct,
    pub pstr_common_tables: *mut ixheaacd_misc_tables,
    pub ptr_pvc_data_str: *mut ia_pvc_data_struct,
    pub hbe_txposer_buffers: *mut core::ffi::c_void,
    pub time_sample_buf: [*mut FLOAT32; 2],
    pub scratch_mem_v: *mut core::ffi::c_void,
    pub frame_buffer: [*mut core::ffi::c_void; 2],
    pub str_sbr_dflt_header: ia_sbr_header_data_struct,
    pub stereo_config_idx: FLAG,
    pub usac_independency_flag: FLAG,
    pub pvc_flag: FLAG,
    pub hbe_flag: FLAG,
    pub sbr_mode: FLAG,
    pub prev_sbr_mode: FLAG,
    pub inter_tes_flag: FLAG,
    pub aot_usac_flag: FLAG,
    pub band_count: [WORD32; 2],
    pub xaac_jmp_buf: *mut jmp_buf,
    pub ptr_mps_data: *mut WORD8,
    pub left_mps_bits: WORD32,
    pub mps_bits_pos: WORD32,
    pub esbr_hq: FLAG,
    pub enh_sbr: FLAG,
    pub enh_sbr_ps: FLAG,
    pub eld_sbr: FLAG,
    pub num_delay_frames: WORD32,
    pub sbr_parse_err_flag: FLAG,
    pub frame_ok: FLAG,
    pub ec_flag: FLAG,
    pub first_frame: FLAG,
    pub prev_usac_independency_flag: FLAG,
    pub sbr_parse_complete: FLAG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_pvc_data_struct {
    pub pvc_mode: UWORD8,
    pub div_mode: UWORD8,
    pub ns_mode: UWORD8,
    pub num_time_slots: UWORD8,
    pub nb_high: UWORD8,
    pub nb_high_per_grp: UWORD8,
    pub prev_pvc_flg: UWORD8,
    pub prev_pvc_mode: UWORD8,
    pub pvc_rate: UWORD8,
    pub prev_pvc_rate: UWORD8,
    pub dummy: WORD16,
    pub pvc_id: [UWORD16; 16],
    pub prev_pvc_id: UWORD16,
    pub p_pred_coeff_tab_1: *mut WORD8,
    pub p_pred_coeff_tab_2: *mut WORD8,
    pub p_pvc_id_boundary: *mut UWORD8,
    pub prev_first_bnd_idx: WORD16,
    pub prev_sbr_mode: WORD32,
    pub p_smth_wind_coeff: *mut FLOAT32,
    pub p_q_fac: *mut FLOAT32,
    pub esg: [[FLOAT32; 3]; 31],
    pub smooth_esg_arr: [FLOAT32; 48],
    pub sbr_range_esg_arr: [FLOAT32; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_misc_tables {
    pub trig_data: [WORD16; 513],
    pub sine_table8_16: [WORD16; 8],
    pub log_dual_is_table: [WORD16; 65],
    pub down_mix_martix: [[[WORD32; 8]; 2]; 4],
    pub cc_gain_scale: [WORD32; 4],
    pub inv_table: [WORD16; 256],
    pub sqrt_table: [WORD16; 257],
    pub dummy: WORD32,
    pub start_band: [[WORD32; 16]; 10],
    pub stop_band: [[WORD32; 16]; 10],
    pub stop_freq_table_fs40k_2: [WORD32; 14],
    pub stop_freq_table_fs40k_4: [WORD32; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_channel_struct {
    pub pstr_prev_frame_data: *mut ia_sbr_prev_frame_data_struct,
    pub str_sbr_dec: ia_sbr_dec_struct,
    pub output_frame_size: WORD32,
    pub sync_state: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_dec_struct {
    pub ptr_sbr_overlap_buf: *mut WORD32,
    pub drc_factors_sbr: *mut *mut WORD32,
    pub str_codec_qmf_bank: ia_sbr_qmf_filter_bank_struct,
    pub str_synthesis_qmf_bank: ia_sbr_qmf_filter_bank_struct,
    pub str_sbr_calc_env: ia_sbr_calc_env_struct,
    pub str_hf_generator: ia_sbr_hf_generator_struct,
    pub str_sbr_scale_fact: ia_sbr_scale_fact_struct,
    pub max_samp_val: WORD32,
    pub band_count: WORD32,
    pub p_hbe_txposer: *mut ia_esbr_hbe_txposer_struct,
    pub core_sample_buf: [FLOAT32; 2624],
    pub core_sample_buf_sbr: [WORD16; 2624],
    pub ph_vocod_qmf_real: [[FLOAT32; 64]; 78],
    pub ph_vocod_qmf_imag: [[FLOAT32; 64]; 78],
    pub sbr_qmf_out_real: [[FLOAT32; 64]; 78],
    pub sbr_qmf_out_imag: [[FLOAT32; 64]; 78],
    pub qmf_buf_real: [[FLOAT32; 64]; 142],
    pub qmf_buf_imag: [[FLOAT32; 64]; 142],
    pub mps_qmf_buf_real: [[FLOAT32; 64]; 78],
    pub mps_qmf_buf_imag: [[FLOAT32; 64]; 78],
    pub mps_sbr_qmf_buf_real: [[FLOAT32; 64]; 78],
    pub mps_sbr_qmf_buf_imag: [[FLOAT32; 64]; 78],
    pub sbr_scratch_local: [WORD32; 256],
    pub scratch_buff: [FLOAT32; 320],
    pub qmf_energy_buf: [[FLOAT32; 32]; 64],
    pub pvc_qmf_enrg_arr: [FLOAT32; 512],
    pub pp_qmf_buf_real: *mut *mut FLOAT32,
    pub pp_qmf_buf_imag: *mut *mut FLOAT32,
    pub p_arr_qmf_buf_real: [*mut WORD32; 38],
    pub p_arr_qmf_buf_imag: [*mut WORD32; 38],
    pub time_sample_buf: *mut FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_esbr_hbe_txposer_struct {
    pub x_over_qmf: [WORD32; 6],
    pub max_stretch: WORD32,
    pub core_frame_length: WORD32,
    pub hbe_qmf_in_len: WORD32,
    pub hbe_qmf_out_len: WORD32,
    pub no_bins: WORD32,
    pub start_band: WORD32,
    pub end_band: WORD32,
    pub upsamp_4_flag: WORD32,
    pub synth_buf_offset: WORD32,
    pub ptr_input_buf: *mut FLOAT32,
    pub qmf_in_buf: *mut *mut FLOAT32,
    pub qmf_out_buf: *mut *mut FLOAT32,
    pub k_start: WORD32,
    pub synth_size: WORD32,
    pub synth_buf: [FLOAT32; 1280],
    pub analy_buf: [FLOAT32; 640],
    pub synth_wind_coeff: *mut FLOAT32,
    pub analy_wind_coeff: *mut FLOAT32,
    pub synth_cos_tab: *mut FLOAT32,
    pub analy_cos_sin_tab: *mut FLOAT32,
    pub norm_qmf_in_buf: [[FLOAT32; 128]; 46],
    pub ixheaacd_real_synth_fft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
    >,
    pub ixheaacd_cmplx_anal_fft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32) -> VOID,
    >,
    pub esbr_hq: WORD32,
    pub in_hop_size: WORD32,
    pub fft_size: [WORD32; 2],
    pub anal_window: *mut FLOAT32,
    pub synth_window: *mut FLOAT32,
    pub ptr_spectrum: *mut FLOAT32,
    pub ptr_spectrum_tx: *mut FLOAT32,
    pub mag: *mut FLOAT32,
    pub phase: *mut FLOAT32,
    pub ptr_output_buf: *mut FLOAT32,
    pub ana_fft_size: [WORD32; 2],
    pub syn_fft_size: [WORD32; 2],
    pub out_hop_size: WORD32,
    pub analy_size: WORD32,
    pub x_over_bin: [[WORD32; 2]; 4],
    pub a_start: WORD32,
    pub spectrum_buf: [FLOAT32; 1536],
    pub spectrum_transposed_buf: [FLOAT32; 1536],
    pub mag_buf: [FLOAT32; 1536],
    pub phase_buf: [FLOAT32; 1536],
    pub output_buf: [FLOAT32; 4096],
    pub fd_win_buf: [[[FLOAT32; 1536]; 3]; 3],
    pub analysis_window_buf: [FLOAT32; 1024],
    pub synthesis_window_buf: [FLOAT32; 1024],
    pub oversampling_flag: WORD32,
    pub str_dft_hbe_anal_coeff: ia_dft_hbe_anal_coeff,
    pub ixheaacd_hbe_anal_fft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32, WORD32) -> VOID,
    >,
    pub ixheaacd_hbe_synth_ifft: Option<
        unsafe extern "C" fn(*mut FLOAT32, *mut FLOAT32, WORD32, WORD32) -> VOID,
    >,
    pub syn_cos_sin_tab: *mut FLOAT32,
    pub ana_cos_sin_tab: *mut FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_dft_hbe_anal_coeff {
    pub real: [[FLOAT32; 128]; 64],
    pub imag: [[FLOAT32; 128]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_calc_env_struct {
    pub filt_buf_me: *mut WORD16,
    pub filt_buf_noise_m: *mut WORD16,
    pub filt_buf_noise_e: WORD32,
    pub start_up: FLAG,
    pub ph_index: WORD16,
    pub tansient_env_prev: WORD16,
    pub harm_flags_prev: [WORD8; 56],
    pub harm_index: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_qmf_filter_bank_struct {
    pub no_channels: WORD32,
    pub analy_win_coeff: *const WORD16,
    pub p_filter: *const WORD16,
    pub cos_twiddle: *const WORD16,
    pub sin_twiddle: *const WORD16,
    pub alt_sin_twiddle: *const WORD16,
    pub t_cos: *const WORD16,
    pub t_sin: *const WORD16,
    pub anal_filter_states: *mut WORD16,
    pub filter_states: *mut WORD16,
    pub num_time_slots: WORD16,
    pub lsb: WORD16,
    pub usb: WORD16,
    pub qmf_filter_state_size: WORD16,
    pub core_samples_buffer: *mut WORD16,
    pub ana_offset: WORD16,
    pub filter_pos: *mut WORD16,
    pub dummy_0: *mut WORD16,
    pub ixheaacd_drc_offset: WORD16,
    pub filter_pos_syn: *mut WORD16,
    pub dummy_1: *mut WORD16,
    pub analy_win_coeff_32: *mut WORD32,
    pub p_filter_32: *const WORD32,
    pub esbr_cos_twiddle: *const WORD32,
    pub esbr_alt_sin_twiddle: *const WORD32,
    pub esbr_t_cos: *const WORD32,
    pub anal_filter_states_32: *mut WORD32,
    pub state_new_samples_pos_low_32: *mut WORD32,
    pub filter_states_32: *mut WORD32,
    pub filter_pos_32: *mut WORD32,
    pub filter_pos_syn_32: *mut WORD32,
    pub fp1_anal: *mut WORD16,
    pub fp2_anal: *mut WORD16,
    pub filter_2: *mut WORD16,
    pub fp1_syn: *mut WORD16,
    pub fp2_syn: *mut WORD16,
    pub sixty4: WORD16,
    pub core_samples_buffer_32: *mut WORD32,
    pub fp1_anal_32: *mut WORD32,
    pub fp2_anal_32: *mut WORD32,
    pub filter_2_32: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_prev_frame_data_struct {
    pub sfb_nrg_prev: [WORD16; 56],
    pub prev_noise_level: [WORD16; 5],
    pub amp_res: WORD16,
    pub end_position: WORD16,
    pub max_qmf_subband_aac: WORD32,
    pub coupling_mode: WORD32,
    pub sbr_invf_mode: [WORD32; 10],
}
pub type ia_handle_sbr_dec_inst_struct = *mut ia_sbr_dec_inst_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_scr_struct {
    pub ptr_work_buf_core: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_frame_info_data_struct {
    pub num_env_sfac: WORD16,
    pub str_frame_info_details: ia_frame_info_struct,
    pub del_cod_dir_arr: [WORD16; 8],
    pub del_cod_dir_noise_arr: [WORD16; 2],
    pub sbr_invf_mode: [WORD32; 10],
    pub coupling_mode: WORD32,
    pub amp_res: WORD16,
    pub max_qmf_subband_aac: WORD32,
    pub add_harmonics: [FLAG; 56],
    pub int_env_sf_arr: [WORD16; 448],
    pub int_noise_floor: [WORD16; 10],
    pub num_noise_sfac: WORD32,
    pub str_pvc_frame_info: ia_frame_info_struct,
    pub env_short_flag_prev: WORD32,
    pub pstr_sbr_header: *mut ia_sbr_header_data_struct,
    pub num_time_slots: WORD32,
    pub rate: WORD32,
    pub sbr_patching_mode: WORD32,
    pub prev_sbr_patching_mode: WORD32,
    pub over_sampling_flag: WORD32,
    pub pitch_in_bins: WORD32,
    pub pvc_mode: WORD32,
    pub cov_count: WORD32,
    pub sbr_invf_mode_prev: [WORD32; 10],
    pub flt_env_sf_arr: [FLOAT32; 448],
    pub flt_noise_floor: [FLOAT32; 10],
    pub sfb_nrg_prev: [FLOAT32; 56],
    pub prev_noise_level: [FLOAT32; 10],
    pub inter_temp_shape_mode: [WORD32; 8],
    pub var_len: WORD32,
    pub bs_sin_pos_present: WORD32,
    pub sine_position: WORD32,
    pub sin_start_for_next_top: WORD32,
    pub sin_len_for_next_top: WORD32,
    pub sin_start_for_cur_top: WORD32,
    pub sin_len_for_cur_top: WORD32,
    pub var_len_id_prev: WORD32,
    pub str_frame_info_prev: ia_frame_info_struct,
    pub bw_array_prev: [FLOAT32; 6],
    pub patch_param: ixheaacd_lpp_trans_patch,
    pub harm_index: WORD32,
    pub phase_index: WORD32,
    pub harm_flag_prev: [WORD8; 64],
    pub e_gain: [[FLOAT32; 64]; 5],
    pub noise_buf: [[FLOAT32; 64]; 5],
    pub lim_table: [[WORD32; 13]; 4],
    pub gate_mode: [WORD32; 4],
    pub harm_flag_varlen_prev: [WORD8; 64],
    pub harm_flag_varlen: [WORD8; 64],
    pub qmapped_pvc: [[FLOAT32; 48]; 64],
    pub env_tmp: [[FLOAT32; 48]; 64],
    pub noise_level_pvc: [[FLOAT32; 48]; 64],
    pub nrg_est_pvc: [[FLOAT32; 48]; 64],
    pub nrg_ref_pvc: [[FLOAT32; 48]; 64],
    pub nrg_gain_pvc: [[FLOAT32; 48]; 64],
    pub nrg_tone_pvc: [[FLOAT32; 48]; 64],
    pub stereo_config_idx: WORD32,
    pub reset_flag: FLAG,
    pub mps_sbr_flag: FLAG,
    pub usac_independency_flag: FLAG,
    pub inter_tes_flag: FLAG,
    pub sbr_mode: FLAG,
    pub prev_sbr_mode: FLAG,
    pub eld_sbr_flag: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sbr_pers_struct {
    pub sbr_qmf_analy_states: *mut WORD16,
    pub sbr_qmf_analy_states_32: *mut WORD32,
    pub sbr_qmf_synth_states: *mut WORD16,
    pub sbr_qmf_synth_states_32: *mut WORD32,
    pub sbr_lpc_filter_states_real: [*mut *mut WORD32; 2],
    pub sbr_lpc_filter_states_imag: [*mut *mut WORD32; 2],
    pub ptr_sbr_overlap_buf: [*mut WORD32; 2],
    pub str_sbr_dec_inst: ia_sbr_dec_inst_struct,
    pub str_sbr_tran_settings: ia_transposer_settings_struct,
    pub sbr_smooth_gain_buf: [*mut WORD16; 2],
    pub sbr_smooth_noise_buf: [*mut WORD16; 2],
    pub pstr_prev_frame_data: [*mut ia_sbr_prev_frame_data_struct; 2],
}
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const END_HDR: C2RustUnnamed_0 = 12;
pub const CRC_LEVEL_FIN: C2RustUnnamed_0 = 11;
pub const ID_IIND_ICS: C2RustUnnamed_0 = 10;
pub const ID_NULL: C2RustUnnamed_0 = 9;
pub const ID_HDR: C2RustUnnamed_0 = 8;
pub const ID_END: C2RustUnnamed_0 = 7;
pub const ID_FIL: C2RustUnnamed_0 = 6;
pub const ID_PCE: C2RustUnnamed_0 = 5;
pub const ID_DSE: C2RustUnnamed_0 = 4;
pub const ID_LFE: C2RustUnnamed_0 = 3;
pub const ID_CCE: C2RustUnnamed_0 = 2;
pub const ID_CPE: C2RustUnnamed_0 = 1;
pub const ID_SCE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_latm_specs {
    pub aot: AUDIO_OBJECT_TYPE,
    pub sampling_freq_index: UWORD32,
    pub sampling_freq: UWORD32,
    pub channel_config: WORD32,
    pub samples_per_frame: UWORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_latm_layer_info {
    pub frame_len_type: UWORD32,
    pub buffer_fullness: UWORD32,
    pub frame_len_bits: UWORD32,
    pub asc: ixheaacd_latm_specs,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_latm_struct {
    pub use_same_stream_mux: UWORD32,
    pub audio_mux_version: UWORD32,
    pub all_streams_same_time_framing: UWORD32,
    pub num_sub_frames: UWORD32,
    pub num_program: UWORD32,
    pub num_layer: UWORD32,
    pub use_same_config: UWORD32,
    pub other_data_present: UWORD32,
    pub other_data_length: UWORD32,
    pub crc_check_present: UWORD32,
    pub crc_check_sum: UWORD32,
    pub frame_length: UWORD32,
    pub layer_info: [[ixheaacd_latm_layer_info; 8]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_decoder_struct {
    pub frame_status: FLAG,
    pub byte_align_bits: WORD32,
    pub pstr_sbr_bitstream: *mut ia_aac_dec_sbr_bitstream_struct,
    pub pstr_aac_dec_ch_info: [*mut ia_aac_dec_channel_info_struct; 2],
    pub ptr_aac_dec_static_channel_info: [*mut ia_aac_dec_channel_info; 2],
    pub pstr_aac_dec_overlap_info: [*mut ia_aac_dec_overlap_info; 2],
    pub pstr_pns_rand_vec_data: *mut ia_pns_rand_vec_struct,
    pub p_ind_channel_info: *mut core::ffi::c_void,
    pub block_number: WORD32,
    pub sampling_rate_index: WORD16,
    pub sampling_rate: WORD32,
    pub samples_per_frame: WORD32,
    pub channels: WORD16,
    pub num_swb_window: [WORD8; 2],
    pub pstr_aac_tables: *mut ia_aac_dec_tables_struct,
    pub pstr_common_tables: *mut ixheaacd_misc_tables,
    pub is_first: WORD32,
    pub conceal_count: WORD32,
    pub sbr_num_elements: WORD16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_persistent_struct {
    pub overlap_buffer: *mut WORD32,
    pub str_aac_dec_overlap_info: [ia_aac_dec_overlap_info; 2],
    pub str_pns_rand_vec_data: ia_pns_rand_vec_struct,
    pub str_aac_decoder: ia_aac_decoder_struct,
    pub sbr_payload_buffer: *mut WORD8,
    pub ptr_aac_dec_static_channel_info: [*mut ia_aac_dec_channel_info; 2],
    pub ltp_buf: [*mut WORD16; 2],
    pub prev_sbr_payload_buffer: *mut WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_scratch_struct {
    pub base_scr_8k: *mut core::ffi::c_void,
    pub extra_scr_4k: [*mut core::ffi::c_void; 4],
    pub in_data: *mut WORD32,
    pub out_data: *mut WORD32,
}
pub type UINT32 = UWORD32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_dec_sbr_config_struct {
    pub harmonic_sbr: UINT32,
    pub bs_inter_tes: UINT32,
    pub bs_pvc: UINT32,
    pub dflt_start_freq: WORD16,
    pub dflt_stop_freq: WORD16,
    pub dflt_header_extra1: WORD16,
    pub dflt_header_extra2: WORD16,
    pub dflt_freq_scale: WORD16,
    pub dflt_alter_scale: WORD16,
    pub dflt_noise_bands: WORD16,
    pub dflt_limiter_bands: WORD16,
    pub dflt_limiter_gains: WORD16,
    pub dflt_interpol_freq: WORD16,
    pub dflt_smoothing_mode: WORD16,
}
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
pub struct ia_usac_dec_element_config_struct {
    pub tw_mdct: UINT32,
    pub noise_filling: UINT32,
    pub stereo_config_index: UINT32,
    pub usac_ext_eleme_def_len: UINT32,
    pub usac_ext_elem_pld_frag: UINT32,
    pub str_usac_sbr_config: ia_usac_dec_sbr_config_struct,
    pub str_usac_mps212_config: ia_usac_dec_mps_config_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_decoder_config_struct {
    pub num_elements: UWORD32,
    pub num_config_extensions: UWORD32,
    pub usac_element_type: [UWORD32; 16],
    pub str_usac_element_config: [ia_usac_dec_element_config_struct; 16],
    pub usac_cfg_ext_info_present: [WORD32; 16],
    pub usac_ext_ele_payload_present: [WORD32; 16],
    pub usac_cfg_ext_info_len: [WORD32; 16],
    pub usac_ext_ele_payload_len: [WORD32; 16],
    pub usac_ext_gain_payload_len: [WORD32; 5],
    pub usac_cfg_ext_info_buf: [[UWORD8; 768]; 16],
    pub usac_ext_ele_payload_buf: [[UWORD8; 768]; 16],
    pub usac_ext_gain_payload_buf: [UWORD8; 2304],
    pub preroll_bytes: [UWORD32; 5],
    pub preroll_counter: WORD32,
    pub preroll_flag: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_config_struct {
    pub usac_sampling_frequency_index: UINT32,
    pub usac_sampling_frequency: UINT32,
    pub core_sbr_framelength_index: UINT32,
    pub channel_configuration_index: UINT32,
    pub num_out_channels: UINT32,
    pub output_channel_pos: [UINT32; 255],
    pub str_usac_dec_config: ia_usac_decoder_config_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_lim_struct {
    pub parametric_lim_threshold_present: WORD32,
    pub parametric_lim_threshold: FLOAT32,
    pub parametric_lim_attack: WORD32,
    pub parametric_lim_release_present: WORD32,
    pub parametric_lim_release: WORD32,
    pub drc_characteristic: WORD32,
    pub disable_paramteric_drc: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_type_feed_forward_struct {
    pub level_estim_k_weighting_type: WORD32,
    pub level_estim_integration_time_present: WORD32,
    pub level_estim_integration_time: WORD32,
    pub drc_curve_definition_type: WORD32,
    pub drc_characteristic: WORD32,
    pub node_count: WORD32,
    pub node_level: [WORD32; 9],
    pub node_gain: [WORD32; 9],
    pub drc_gain_smooth_parameters_present: WORD32,
    pub gain_smooth_attack_time_slow: WORD32,
    pub gain_smooth_release_time_slow: WORD32,
    pub gain_smooth_time_fast_present: WORD32,
    pub gain_smooth_attack_time_fast: WORD32,
    pub gain_smooth_release_time_fast: WORD32,
    pub gain_smooth_threshold_present: WORD32,
    pub gain_smooth_attack_threshold: WORD32,
    pub gain_smooth_rel_threshold: WORD32,
    pub gain_smooth_hold_off_count_present: WORD32,
    pub gain_smooth_hold_off: WORD32,
    pub disable_paramteric_drc: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_instructions_struct {
    pub parametric_drc_id: WORD32,
    pub parametric_drc_look_ahead_flag: WORD32,
    pub parametric_drc_look_ahead: WORD32,
    pub parametric_drc_preset_id_present: WORD32,
    pub parametric_drc_preset_id: WORD32,
    pub parametric_drc_type: WORD32,
    pub len_bit_size: WORD32,
    pub str_parametric_drc_type_feed_forward: ia_parametric_drc_type_feed_forward_struct,
    pub parametric_drc_lim: ia_parametric_drc_lim_struct,
    pub drc_characteristic: WORD32,
    pub disable_paramteric_drc: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_parametric_drc_gain_set_params_struct {
    pub parametric_drc_id: WORD32,
    pub side_chain_config_type: WORD32,
    pub downmix_id: WORD32,
    pub level_estim_channel_weight_format: WORD32,
    pub level_estim_ch_weight: [FLOAT32; 128],
    pub drc_input_loudness_present: WORD32,
    pub drc_input_loudness: FLOAT32,
    pub ch_count_from_dwnmix_id: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_coeff_parametric_drc_struct {
    pub drc_location: WORD32,
    pub parametric_drc_frame_size_format: WORD32,
    pub parametric_drc_frame_size: WORD32,
    pub parametric_drc_delay_max_present: WORD32,
    pub parametric_drc_delay_max: WORD32,
    pub reset_parametric_drc: WORD32,
    pub parametric_drc_gain_set_count: WORD32,
    pub str_parametric_drc_gain_set_params: [ia_parametric_drc_gain_set_params_struct; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_loud_eq_instructions_struct {
    pub loud_eq_set_id: WORD32,
    pub drc_location: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub drc_set_id_count: WORD32,
    pub drc_set_id: [WORD32; 16],
    pub eq_set_id_count: WORD32,
    pub eq_set_id: [WORD32; 8],
    pub loudness_after_drc: WORD32,
    pub loudness_after_eq: WORD32,
    pub loud_eq_gain_sequence_count: WORD32,
    pub gain_seq_idx: [WORD32; 4],
    pub drc_characteristic_format_is_cicp: [WORD32; 4],
    pub drc_characteristic: [WORD32; 4],
    pub drc_characteristic_left_index: [WORD32; 4],
    pub drc_characteristic_right_index: [WORD32; 4],
    pub frequency_range_index: [WORD32; 4],
    pub loud_eq_scaling: [FLOAT32; 4],
    pub loud_eq_offset: [FLOAT32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filt_ele_struct {
    pub filt_ele_idx: WORD32,
    pub filt_ele_gain_flag: WORD32,
    pub filt_ele_gain: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filt_block_struct {
    pub filter_element_count: WORD32,
    pub str_filter_element: [ia_filt_ele_struct; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_unique_td_filt_element {
    pub eq_filter_format: WORD32,
    pub bs_real_zero_radius_one_count: WORD32,
    pub real_zero_count: WORD32,
    pub generic_zero_count: WORD32,
    pub real_pole_count: WORD32,
    pub cmplx_pole_count: WORD32,
    pub zero_sign: [WORD32; 14],
    pub real_zero_radius: [FLOAT32; 64],
    pub generic_zero_radius: [FLOAT32; 64],
    pub generic_zero_angle: [FLOAT32; 64],
    pub real_pole_radius: [FLOAT32; 16],
    pub complex_pole_radius: [FLOAT32; 16],
    pub complex_pole_angle: [FLOAT32; 16],
    pub fir_filt_order: WORD32,
    pub fir_symmetry: WORD32,
    pub fir_coeff: [FLOAT32; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_subband_gain_spline_struct {
    pub num_eq_nodes: WORD32,
    pub eq_slope: [FLOAT32; 33],
    pub eq_freq_delta: [WORD32; 33],
    pub eq_gain_initial: FLOAT32,
    pub eq_gain_delta: [FLOAT32; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_subband_gain_vector {
    pub eq_subband_gain: [FLOAT32; 135],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_coeff_struct {
    pub eq_delay_max_present: WORD32,
    pub eq_delay_max: WORD32,
    pub unique_filter_block_count: WORD32,
    pub str_filter_block: [ia_filt_block_struct; 16],
    pub unique_td_filter_element_count: WORD32,
    pub unique_td_filt_ele: [ia_unique_td_filt_element; 16],
    pub unique_eq_subband_gains_count: WORD32,
    pub eq_subband_gain_representation: WORD32,
    pub eq_subband_gain_format: WORD32,
    pub eq_subband_gain_count: WORD32,
    pub str_eq_subband_gain_spline: [ia_eq_subband_gain_spline_struct; 16],
    pub str_eq_subband_gain_vector: [ia_eq_subband_gain_vector; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_filter_block_refs_struct {
    pub filter_block_count: WORD32,
    pub filter_block_index: [WORD32; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_td_filter_cascade_struct {
    pub eq_cascade_gain_present: [WORD32; 4],
    pub eq_cascade_gain: [FLOAT32; 4],
    pub str_filter_block_refs: [ia_filter_block_refs_struct; 4],
    pub eq_phase_alignment_present: WORD32,
    pub eq_phase_alignment: [[WORD32; 4]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_eq_instructions_struct {
    pub eq_set_id: WORD32,
    pub eq_set_complexity_level: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub eq_apply_to_downmix: WORD32,
    pub drc_set_id_count: WORD32,
    pub drc_set_id: [WORD32; 16],
    pub eq_set_purpose: WORD32,
    pub depends_on_eq_set_present: WORD32,
    pub depends_on_eq_set: WORD32,
    pub no_independent_eq_use: WORD32,
    pub eq_channel_count: WORD32,
    pub eq_ch_group_count: WORD32,
    pub eq_ch_group_of_channel: [WORD32; 128],
    pub td_filter_cascade_present: WORD32,
    pub str_td_filter_cascade: ia_td_filter_cascade_struct,
    pub subband_gains_present: WORD32,
    pub subband_gains_index: [WORD32; 4],
    pub eq_transition_duration_present: WORD32,
    pub eq_transition_duration: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_config_ext {
    pub drc_config_ext_type: [WORD32; 2],
    pub ext_bit_size: [WORD32; 1],
    pub parametric_drc_present: WORD32,
    pub str_drc_coeff_param_drc: ia_drc_coeff_parametric_drc_struct,
    pub parametric_drc_instructions_count: WORD32,
    pub str_parametric_drc_instructions: [ia_parametric_drc_instructions_struct; 8],
    pub drc_extension_v1_present: WORD32,
    pub loud_eq_instructions_flag: WORD32,
    pub loud_eq_instructions_count: WORD32,
    pub loud_eq_instructions: [ia_loud_eq_instructions_struct; 8],
    pub eq_flag: WORD32,
    pub str_eq_coeff: ia_eq_coeff_struct,
    pub eq_instructions_count: WORD32,
    pub str_eq_instructions: [ia_eq_instructions_struct; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_coefficients_basic_struct {
    pub drc_location: WORD32,
    pub drc_characteristic: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_instructions_basic_struct {
    pub drc_set_id: WORD32,
    pub drc_location: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub drc_set_effect: WORD32,
    pub limiter_peak_target_present: WORD32,
    pub limiter_peak_target: FLOAT32,
    pub drc_set_target_loudness_present: WORD32,
    pub drc_set_target_loudness_value_upper: WORD32,
    pub drc_set_target_loudness_value_lower_present: WORD32,
    pub drc_set_target_loudness_value_lower: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_params_struct {
    pub gain_seq_idx: WORD32,
    pub drc_characteristic: WORD32,
    pub drc_characteristic_present: WORD32,
    pub drc_characteristic_format_is_cicp: WORD32,
    pub drc_characteristic_left_index: WORD32,
    pub drc_characteristic_right_index: WORD32,
    pub crossover_freq_idx: WORD32,
    pub start_subband_index: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_delta_time_code_table_entry_struct {
    pub size: WORD32,
    pub code: WORD32,
    pub value: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tables_struct {
    pub delta_time_code_table: [ia_delta_time_code_table_entry_struct; 526],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_set_params_struct {
    pub gain_coding_profile: WORD32,
    pub gain_interpolation_type: WORD32,
    pub full_frame: WORD32,
    pub time_alignment: WORD32,
    pub time_delt_min_flag: WORD32,
    pub time_delt_min_val: WORD32,
    pub band_count: WORD32,
    pub drc_band_type: WORD32,
    pub gain_params: [ia_gain_params_struct; 8],
    pub num_gain_max_values: WORD32,
    pub str_tables: ia_tables_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_split_drc_characteristic_struct {
    pub characteristic_format: WORD32,
    pub in_out_ratio: FLOAT32,
    pub gain: FLOAT32,
    pub exp: FLOAT32,
    pub flip_sign: WORD32,
    pub characteristic_node_count: WORD32,
    pub node_level: [FLOAT32; 5],
    pub node_gain: [FLOAT32; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_shape_filter_params_struct {
    pub corner_freq_index: WORD32,
    pub filter_strength_index: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_shape_filter_block_params_struct {
    pub lf_cut_filter_present: WORD32,
    pub str_lf_cut_params: ia_shape_filter_params_struct,
    pub lf_boost_filter_present: WORD32,
    pub str_lf_boost_params: ia_shape_filter_params_struct,
    pub hf_cut_filter_present: WORD32,
    pub str_hf_cut_params: ia_shape_filter_params_struct,
    pub hf_boost_filter_present: WORD32,
    pub str_hf_boost_params: ia_shape_filter_params_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_uni_drc_coeffs_struct {
    pub version: WORD32,
    pub drc_location: WORD32,
    pub drc_frame_size_present: WORD32,
    pub drc_frame_size: WORD32,
    pub gain_set_count: WORD32,
    pub gain_set_params: [ia_gain_set_params_struct; 24],
    pub drc_characteristic_left_present: WORD32,
    pub characteristic_left_count: WORD32,
    pub str_split_characteristic_left: [ia_split_drc_characteristic_struct; 8],
    pub drc_characteristic_right_present: WORD32,
    pub characteristic_right_count: WORD32,
    pub str_split_characteristic_right: [ia_split_drc_characteristic_struct; 8],
    pub shape_filters_present: WORD32,
    pub shape_num_filter: WORD32,
    pub str_shape_filter_block_params: [ia_shape_filter_block_params_struct; 9],
    pub gain_sequence_count: WORD32,
    pub gain_set_params_index_for_gain_sequence: [WORD32; 24],
    pub gain_set_count_plus: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_gain_modifiers_struct {
    pub target_characteristic_left_present: [WORD32; 8],
    pub target_characteristic_left_index: [WORD32; 8],
    pub target_characteristic_right_present: [WORD32; 8],
    pub target_characteristic_right_index: [WORD32; 8],
    pub shape_filter_flag: WORD32,
    pub shape_filter_idx: WORD32,
    pub gain_scaling_flag: [WORD32; 8],
    pub attn_scaling: [FLOAT32; 8],
    pub ampl_scaling: [FLOAT32; 8],
    pub gain_offset_flag: [WORD32; 8],
    pub gain_offset: [FLOAT32; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ducking_modifiers_struct {
    pub ducking_scaling_flag: WORD32,
    pub ducking_scaling: FLOAT32,
    pub ducking_scaling_quantized: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_instructions_struct {
    pub drc_set_id: WORD32,
    pub drc_set_complexity_level: WORD32,
    pub requires_eq: WORD32,
    pub drc_apply_to_dwnmix: WORD32,
    pub drc_location: WORD32,
    pub dwnmix_id_count: WORD32,
    pub downmix_id: [WORD32; 8],
    pub depends_on_drc_set_present: WORD32,
    pub depends_on_drc_set: WORD32,
    pub no_independent_use: WORD32,
    pub drc_set_effect: WORD32,
    pub gain_set_index: [WORD32; 128],
    pub str_gain_modifiers_of_ch_group: [ia_gain_modifiers_struct; 24],
    pub str_ducking_modifiers_for_channel: [ia_ducking_modifiers_struct; 128],
    pub limiter_peak_target_present: WORD32,
    pub limiter_peak_target: FLOAT32,
    pub drc_set_target_loudness_present: WORD32,
    pub drc_set_target_loudness_value_upper: WORD32,
    pub drc_set_target_loudness_value_lower_present: WORD32,
    pub drc_set_target_loudness_value_lower: WORD32,
    pub audio_num_chan: WORD32,
    pub num_drc_ch_groups: WORD32,
    pub gain_set_index_for_channel_group: [WORD32; 24],
    pub band_count_of_ch_group: [WORD32; 24],
    pub gain_interpolation_type_for_channel_group: [WORD32; 24],
    pub time_delta_min_for_channel_group: [WORD32; 24],
    pub time_alignment_for_channel_group: [WORD32; 24],
    pub str_ducking_modifiers_for_channel_group: [ia_ducking_modifiers_struct; 24],
    pub channel_group_of_ch: [WORD32; 128],
    pub num_chan_per_ch_group: [WORD32; 24],
    pub gain_element_count: WORD32,
    pub multiband_audio_sig_count: WORD32,
    pub ch_group_parametric_drc_flag: [WORD32; 24],
    pub gain_set_idx_of_ch_group_parametric_drc: [WORD32; 24],
    pub parametric_drc_look_ahead_samples: [WORD32; 24],
    pub parametric_drc_look_ahead_samples_max: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_channel_layout_struct {
    pub base_channel_count: WORD32,
    pub layout_signaling_present: WORD32,
    pub defined_layout: WORD32,
    pub speaker_position: [WORD32; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_downmix_instructions_struct {
    pub downmix_id: WORD32,
    pub target_channel_count: WORD32,
    pub target_layout: WORD32,
    pub downmix_coefficients_present: WORD32,
    pub downmix_coefficient: [FLOAT32; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_drc_config {
    pub sample_rate_present: WORD32,
    pub sampling_rate: WORD32,
    pub dwnmix_instructions_count: WORD32,
    pub drc_coefficients_drc_count: WORD32,
    pub drc_instructions_uni_drc_count: WORD32,
    pub drc_instructions_count_plus: WORD32,
    pub drc_description_basic_present: WORD32,
    pub drc_coefficients_basic_count: WORD32,
    pub drc_instructions_basic_count: WORD32,
    pub drc_config_ext_present: WORD32,
    pub apply_drc: WORD32,
    pub str_drc_config_ext: ia_drc_config_ext,
    pub str_drc_coefficients_basic: [ia_drc_coefficients_basic_struct; 8],
    pub str_drc_instructions_basic: [ia_drc_instructions_basic_struct; 36],
    pub str_p_loc_drc_coefficients_uni_drc: [ia_uni_drc_coeffs_struct; 8],
    pub str_drc_instruction_str: [ia_drc_instructions_struct; 36],
    pub channel_layout: ia_channel_layout_struct,
    pub dwnmix_instructions: [ia_downmix_instructions_struct; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_cmplx_flt_struct {
    pub re: FLOAT32,
    pub im: FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_decor_filt_struct {
    pub state_len: WORD32,
    pub num_len: WORD32,
    pub den_len: WORD32,
    pub state: [ia_cmplx_flt_struct; 21],
    pub num: *const FLOAT32,
    pub den: *const FLOAT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ixheaacd_mps_decor_energy_adjust_filt_struct {
    pub num_bins: WORD32,
    pub smooth_in_energy: [FLOAT32; 28],
    pub smooth_out_energy: [FLOAT32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_decor_struct {
    pub num_bins: WORD32,
    pub filter: [ia_mps_decor_filt_struct; 71],
    pub decor_nrg_smooth: ixheaacd_mps_decor_energy_adjust_filt_struct,
    pub delay_sample_count: [WORD32; 71],
    pub decor_delay_buffer: [[ia_cmplx_flt_struct; 86]; 71],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_hybrid_filt_struct {
    pub hf_buffer: [[ia_cmplx_flt_struct; 78]; 128],
    pub lf_buffer: [[ia_cmplx_flt_struct; 84]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_data_struct {
    pub bs_xxx_data_mode: [WORD32; 9],
    pub quant_coarse_xxx_flag: [WORD32; 9],
    pub bs_freq_res_stride_xxx: [WORD32; 9],
    pub bs_quant_coarse_xxx: [WORD8; 9],
    pub bs_quant_coarse_xxx_prev: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_bs_frame {
    pub independency_flag: WORD8,
    pub cld_idx: [[WORD32; 28]; 9],
    pub icc_idx: [[WORD32; 28]; 9],
    pub cld_idx_pre: [WORD32; 28],
    pub icc_idx_pre: [WORD32; 28],
    pub cmp_cld_idx: [[WORD32; 28]; 9],
    pub cmp_icc_idx: [[WORD32; 28]; 9],
    pub cmp_cld_idx_prev: [WORD32; 28],
    pub cmp_icc_idx_prev: [WORD32; 28],
    pub cld_data: ia_mps_data_struct,
    pub icc_data: ia_mps_data_struct,
    pub ipd_data: ia_mps_data_struct,
    pub ipd_idx_data: [[WORD32; 28]; 9],
    pub ipd_idx_data_prev: [WORD32; 28],
    pub ipd_idx: [[WORD32; 28]; 9],
    pub ipd_idx_prev: [WORD32; 28],
    pub bs_smooth_mode: [WORD32; 9],
    pub bs_smooth_time: [WORD32; 9],
    pub bs_freq_res_stride_smg: [WORD32; 9],
    pub bs_smg_data: [[WORD32; 28]; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_smoothing_struct {
    pub prev_smg_time: WORD32,
    pub inv_prev_smg_time: FLOAT32,
    pub prev_smg_data: [WORD32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_env_reshape_struct {
    pub pb_energy_prev: [[FLOAT32; 28]; 3],
    pub avg_energy_prev: [FLOAT32; 3],
    pub frame_energy_prev: [FLOAT32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_stp_struct {
    pub nrg_dir: FLOAT32,
    pub nrg_diff: [FLOAT32; 2],
    pub nrg_dir_prev: FLOAT32,
    pub nrg_diff_prev: [FLOAT32; 2],
    pub tp_scale_last: [FLOAT32; 2],
    pub init_flag: WORD32,
    pub update_old_ener: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_opd_smooth_struct {
    pub smooth_l_phase: [WORD32; 28],
    pub smooth_r_phase: [WORD32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_state_struct {
    pub in_ch_count: WORD32,
    pub out_ch_count: WORD32,
    pub input_gain: FLOAT32,
    pub dir_sig_count: WORD32,
    pub decor_sig_count: WORD32,
    pub time_slots: WORD32,
    pub pre_mix_req: WORD32,
    pub temp_shape_enable_ch_stp: [WORD32; 2],
    pub temp_shape_enable_ch_ges: [WORD32; 2],
    pub env_shape_data: [[FLOAT32; 72]; 2],
    pub parse_nxt_frame: WORD8,
    pub band_count: [WORD32; 2],
    pub synth_count: WORD32,
    pub qmf_band_count: WORD32,
    pub hyb_band_count: [WORD32; 2],
    pub hyb_band_count_max: WORD32,
    pub hyb_band_to_processing_band_table: *const WORD32,
    pub res_ch_count: WORD32,
    pub res_bands: WORD32,
    pub max_res_bands: WORD32,
    pub bs_param_bands: WORD32,
    pub ext_frame_flag: WORD32,
    pub num_parameter_sets: WORD32,
    pub num_parameter_sets_prev: WORD32,
    pub param_slots: [WORD32; 9],
    pub param_slot_diff: [WORD32; 9],
    pub inv_param_slot_diff: [FLOAT32; 9],
    pub inv_param_slot_diff_Q30: [WORD32; 9],
    pub frame_length: WORD32,
    pub residual_coding: WORD32,
    pub bs_residual_present: WORD32,
    pub bs_residual_bands: WORD32,
    pub config: *mut ia_usac_dec_mps_config_struct,
    pub ldmps_config: ia_usac_dec_mps_config_struct,
    pub bs_frame: ia_mps_bs_frame,
    pub smoothing_time: [WORD32; 9],
    pub inv_smoothing_time: [FLOAT32; 9],
    pub smoothing_data: [[WORD32; 28]; 9],
    pub bs_tsd_enable: WORD32,
    pub bs_tsd_sep_data: [WORD32; 72],
    pub bs_tsd_tr_phase_data: [WORD32; 72],
    pub tsd_num_tr_slots: WORD32,
    pub tsd_codeword_len: WORD32,
    pub cld_data: [[FLOAT32; 28]; 9],
    pub icc_data: [[FLOAT32; 28]; 9],
    pub ipd_data: [[FLOAT32; 28]; 9],
    pub bs_phase_mode: WORD32,
    pub opd_smoothing_mode: WORD32,
    pub num_bands_ipd: WORD32,
    pub phase_l: [[FLOAT32; 28]; 9],
    pub phase_r: [[FLOAT32; 28]; 9],
    pub phase_l_prev: [FLOAT32; 28],
    pub phase_r_prev: [FLOAT32; 28],
    pub m1_param_re: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m1_param_im: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m2_decor_re: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m2_decor_im: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m2_resid_re: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m2_resid_im: [[[[FLOAT32; 2]; 2]; 28]; 9],
    pub m1_param_re_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m1_param_im_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m2_decor_re_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m2_decor_im_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m2_resid_re_prev: [[[FLOAT32; 2]; 2]; 28],
    pub m2_resid_im_prev: [[[FLOAT32; 2]; 2]; 28],
    pub qmf_in: [[[ia_cmplx_flt_struct; 72]; 64]; 2],
    pub hyb_in: [[[ia_cmplx_flt_struct; 72]; 71]; 2],
    pub hyb_res: [[ia_cmplx_flt_struct; 72]; 71],
    pub v: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub w_diff: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub w_dir: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub hyb_dir_out: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub hyb_diff_out: [[[ia_cmplx_flt_struct; 71]; 72]; 2],
    pub qmf_out_dir: [[[ia_cmplx_flt_struct; 128]; 72]; 2],
    pub scratch: [[ia_cmplx_flt_struct; 71]; 72],
    pub output_buffer: *mut [FLOAT32; 4096],
    pub hyb_filt_state: [ia_mps_hybrid_filt_struct; 2],
    pub qmf_filt_state: [[FLOAT32; 1280]; 2],
    pub mps_decor: ia_mps_decor_struct,
    pub smoothing_filt_state: ia_mps_smoothing_struct,
    pub guided_env_shaping: ia_mps_env_reshape_struct,
    pub bs_high_rate_mode: WORD32,
    pub tmp_buf: [FLOAT32; 10752],
    pub r_out_re_in_m1: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_im_in_m1: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_re_scratch_m1: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_out_im_scratch_m1: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_out_re_in_m2: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_im_in_m2: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_diff_re_in_m2: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_diff_im_in_m2: [[[[FLOAT32; 2]; 2]; 28]; 72],
    pub r_out_re_fix_in_m2: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_out_im_fix_in_m2: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_diff_out_re_fix_in_m2: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_diff_out_im_fix_in_m2: [[[[WORD32; 2]; 2]; 28]; 72],
    pub r_out_ph_re_in_m2: [[[FLOAT32; 2]; 28]; 72],
    pub r_out_ph_im_in_m2: [[[FLOAT32; 2]; 28]; 72],
    pub subband_var: ia_mps_stp_struct,
    pub opd_smooth: ia_mps_opd_smooth_struct,
    pub resolution: WORD32,
    pub p_sbr_dec: [*mut core::ffi::c_void; 2],
    pub p_sbr_frame: [*mut core::ffi::c_void; 2],
    pub p_sbr_header: [*mut core::ffi::c_void; 2],
    pub object_type: WORD32,
    pub mps_init_done: WORD32,
    pub str_mps_qmf_bank: ia_sbr_qmf_filter_bank_struct,
    pub qmf_dec_tables_ptr: *mut ia_qmf_dec_tables_struct,
    pub sbr_tables_ptr: *mut ia_sbr_tables_struct,
    pub str_sbr_scale_fact: *mut ia_sbr_scale_fact_struct,
    pub ec_flag: WORD8,
    pub frame_ok: WORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_decorr_filter_instance_struct {
    pub state_length: WORD32,
    pub num_length: WORD32,
    pub den_length: WORD32,
    pub complex: WORD32,
    pub state_real: *mut WORD32,
    pub state_imag: *mut WORD32,
    pub numerator_real: *mut WORD32,
    pub numerator_imag: *mut WORD32,
    pub denominator_real: *mut WORD32,
    pub denominator_imag: *mut WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub decorr_seed: WORD32,
    pub numbins: WORD32,
    pub filter: [*mut ia_mps_dec_decorr_filter_instance_struct; 71],
    pub ducker: *mut ia_mps_dec_ducker_interface,
    pub no_sample_delay: [WORD32; 71],
    pub delay_buffer_real: *mut *mut WORD32,
    pub delay_buffer_imag: *mut *mut WORD32,
}
pub type ia_mps_dec_decorr_dec_handle = *mut C2RustUnnamed_1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_thyb_filter_state_struct {
    pub buffer_lf_real: [[WORD32; 84]; 3],
    pub buffer_lf_imag: [[WORD32; 84]; 3],
    pub qmf_lf_real: [[WORD32; 84]; 3],
    pub qmf_lf_imag: [[WORD32; 84]; 3],
    pub buffer_hf_real: [[WORD32; 78]; 64],
    pub buffer_hf_imag: [[WORD32; 78]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_spatial_bs_config_struct {
    pub ui_pcm_wdsz: UWORD32,
    pub ui_samp_freq: UWORD32,
    pub ui_in_channels: UWORD32,
    pub ui_out_channels: UWORD32,
    pub ui_channel_mask: WORD32,
    pub frame_ok: WORD32,
    pub ui_bs_is_buried: UWORD32,
    pub ui_dec_type: WORD32,
    pub ui_upmix_type: WORD32,
    pub ui_binaural_quality: WORD32,
    pub ui_hrtf_model: WORD32,
    pub ui_qmf_bands: UWORD32,
    pub bs_frame_length: WORD32,
    pub bs_sampling_freq_index: WORD32,
    pub bs_sampling_frequency: WORD32,
    pub bs_freq_res: WORD32,
    pub bs_tree_config: WORD32,
    pub bs_quant_mode: WORD32,
    pub bs_one_icc: WORD32,
    pub bs_arbitrary_downmix: WORD32,
    pub bs_residual_coding: WORD32,
    pub bs_smooth_config: WORD32,
    pub bs_fixed_gain_sur: WORD32,
    pub bs_fixed_gain_lfe: WORD32,
    pub bs_fixed_gain_dmx: WORD32,
    pub bs_matrix_mode: WORD32,
    pub bs_temp_shape_config: WORD32,
    pub bs_decorr_config: WORD32,
    pub bs_3d_audio_mode: WORD32,
    pub bs_3d_audio_hrtf_set: WORD32,
    pub bs_hrtf_freq_res: WORD32,
    pub hrtf_num_band: WORD32,
    pub bs_hrtf_num_chan: WORD32,
    pub bs_hrtf_asymmetric: WORD32,
    pub bs_hrtf_level_left: [[WORD32; 28]; 8],
    pub bs_hrtf_level_right: [[WORD32; 28]; 8],
    pub bs_hrtf_phase: [WORD32; 8],
    pub bs_hrtf_phase_lr: [[WORD32; 28]; 8],
    pub bs_ott_bands: [WORD32; 5],
    pub bs_ttt_dual_mode: [WORD32; 1],
    pub bs_ttt_mode_low: [WORD32; 1],
    pub bs_ttt_mode_high: [WORD32; 1],
    pub bs_ttt_bands_low: [WORD32; 1],
    pub bs_sac_ext_type: [WORD32; 8],
    pub sac_ext_cnt: WORD32,
    pub bs_residual_present: [WORD32; 10],
    pub bs_residual_sampling_freq_index: WORD32,
    pub bs_residual_frames_per_spatial_frame: WORD32,
    pub bs_residual_bands: [WORD32; 10],
    pub bs_arbitrary_downmix_residual_sampling_freq_index: WORD32,
    pub bs_arbitrary_downmix_residual_frames_per_spatial_frame: WORD32,
    pub bs_arbitrary_downmix_residual_bands: WORD32,
    pub bs_env_quant_mode: WORD32,
    pub arbitrary_tree: WORD32,
    pub num_out_chan_at: WORD32,
    pub num_ott_boxes_at: WORD32,
    pub bs_output_channel_pos_at: [WORD32; 8],
    pub bs_ott_box_present_at: [[WORD32; 7]; 8],
    pub bs_ott_default_cld_at: [WORD32; 56],
    pub bs_ott_mode_lfe_at: [WORD32; 56],
    pub bs_ott_bands_at: [WORD32; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_lossless_data_struct {
    pub bs_xxx_data_mode: [[WORD32; 8]; 33],
    pub bs_quant_coarse_xxx: [[WORD32; 8]; 33],
    pub bs_freq_res_stride_xxx: [[WORD32; 8]; 33],
    pub bs_quant_coarse_xxx_prev: [WORD32; 33],
    pub no_cmp_quant_coarse_xxx: [[WORD32; 8]; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RESIDUAL_FRAME_DATA {
    pub bs_icc_diff_present: [[WORD32; 8]; 10],
    pub bs_icc_diff: [[[WORD32; 28]; 8]; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_spatial_bs_frame_struct {
    pub bs_independency_flag: WORD32,
    pub ott_cld_idx: [[[WORD32; 28]; 8]; 33],
    pub ott_icc_idx: [[[WORD32; 28]; 8]; 5],
    pub ttt_cpc_1_idx: [[[WORD32; 28]; 8]; 1],
    pub ttt_cpc_2_idx: [[[WORD32; 28]; 8]; 1],
    pub ttt_cld_1_idx: [[[WORD32; 28]; 8]; 1],
    pub ttt_cld_2_idx: [[[WORD32; 28]; 8]; 1],
    pub ttt_icc_idx: [[[WORD32; 28]; 8]; 1],
    pub ott_cld_idx_prev: [[WORD32; 28]; 33],
    pub ott_icc_idx_prev: [[WORD32; 28]; 5],
    pub ttt_cpc_1_idx_prev: [[WORD32; 28]; 1],
    pub ttt_cpc_2_idx_prev: [[WORD32; 28]; 1],
    pub ttt_cld_1_idx_prev: [[WORD32; 28]; 1],
    pub ttt_cld_2_idx_prev: [[WORD32; 28]; 1],
    pub ttt_icc_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ott_cld_idx: [[[WORD32; 28]; 8]; 33],
    pub cmp_ott_icc_idx: [[[WORD32; 28]; 8]; 5],
    pub ott_icc_diff_idx: [[[WORD32; 28]; 8]; 5],
    pub cmp_ttt_cpc_1_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ttt_cpc_2_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ttt_cld_1_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ttt_cld_2_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ttt_icc_idx: [[[WORD32; 28]; 8]; 1],
    pub cmp_ott_cld_idx_prev: [[WORD32; 28]; 33],
    pub cmp_ott_icc_idx_prev: [[WORD32; 28]; 5],
    pub cmp_ttt_cpc_1_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ttt_cpc_2_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ttt_cld_1_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ttt_cld_2_idx_prev: [[WORD32; 28]; 1],
    pub cmp_ttt_icc_idx_prev: [[WORD32; 28]; 1],
    pub cld_lossless_data: ia_mps_dec_lossless_data_struct,
    pub icc_lossless_data: ia_mps_dec_lossless_data_struct,
    pub cpc_lossless_data: ia_mps_dec_lossless_data_struct,
    pub bs_smooth_control: WORD32,
    pub bs_smooth_mode: [WORD32; 8],
    pub bs_smooth_time: [WORD32; 8],
    pub bs_freq_res_stride_smg: [WORD32; 8],
    pub bs_smg_data: [[WORD32; 28]; 8],
    pub res_data: RESIDUAL_FRAME_DATA,
    pub arbdmx_gain_idx: [[[WORD32; 28]; 8]; 6],
    pub arbdmx_gain_idx_prev: [[WORD32; 28]; 6],
    pub cmp_arbdmx_gain_idx: [[[WORD32; 28]; 8]; 6],
    pub cmp_arbdmx_gain_idx_prev: [[WORD32; 28]; 6],
    pub bs_arbitrary_downmix_residual_abs: [WORD32; 6],
    pub bs_arbitrary_downmix_residual_alpha_update_set: [WORD32; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_tonality_state_struct {
    pub spec_prev_real: [WORD32; 512],
    pub spec_prev_imag: [WORD32; 512],
    pub p_cross_real: [WORD32; 512],
    pub p_cross_imag: [WORD32; 512],
    pub p_sum: [WORD32; 512],
    pub p_sum_prev: [WORD32; 512],
    pub buf_real: [[WORD32; 6]; 64],
    pub buf_imag: [[WORD32; 6]; 64],
    pub win_buf_real: [[WORD32; 16]; 64],
    pub win_buf_imag: [[WORD32; 16]; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_smoothing_state_struct {
    pub prev_smg_time: WORD32,
    pub prev_smg_data: [WORD32; 28],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_reshape_bb_env_state_struct {
    pub part_nrg_prev: [[WORD32; 28]; 22],
    pub norm_nrg_prev: [WORD32; 22],
    pub frame_nrg_prev: [WORD32; 22],
    pub q_part_nrg_prev: [[WORD16; 28]; 22],
    pub q_norm_nrg_prev: [WORD16; 22],
    pub q_frame_nrg_prev: [WORD16; 22],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_ttt_config_struct {
    pub use_ttt_decorr: WORD32,
    pub mode: WORD32,
    pub start_band: WORD32,
    pub stop_band: WORD32,
    pub bitstream_start_band: WORD32,
    pub bitstream_stop_band: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_blind_decoder_struct {
    pub excitation: [[WORD32; 28]; 3],
    pub filter_coeff: WORD32,
    pub q_excitation: [[WORD16; 28]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_subband_tp_params_struct {
    pub run_dry_ener: [WORD32; 6],
    pub run_wet_ener: [WORD32; 8],
    pub old_dry_ener: [WORD32; 6],
    pub old_wet_ener: [WORD32; 8],
    pub q_run_dry_ener: [WORD16; 6],
    pub q_run_wet_ener: [WORD16; 8],
    pub q_old_dry_ener: [WORD16; 6],
    pub q_old_wet_ener: [WORD16; 8],
    pub update_old_ener: WORD32,
    pub prev_tp_scale: [WORD32; 8],
    pub q_prev_tp_scale: [WORD16; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_persistent_mem {
    pub prev_gain_at: *mut WORD32,
    pub arbdmx_alpha_prev: *mut WORD32,
    pub m1_param_real_prev: *mut WORD32,
    pub m1_param_imag_prev: *mut WORD32,
    pub m2_decor_real_prev: *mut WORD32,
    pub m2_decor_imag_prev: *mut WORD32,
    pub m2_resid_real_prev: *mut WORD32,
    pub m2_resid_imag_prev: *mut WORD32,
    pub qmf_input_delay_real: *mut WORD32,
    pub qmf_input_delay_imag: *mut WORD32,
    pub ana_qmf_states_buffer: *mut WORD32,
    pub syn_qmf_states_buffer: *mut WORD32,
    pub decorr_ptr: *mut core::ffi::c_void,
    pub hyb_filter_state: *mut ia_mps_dec_thyb_filter_state_struct,
    pub ton_state: *mut ia_mps_dec_tonality_state_struct,
    pub smooth_state: *mut ia_mps_dec_smoothing_state_struct,
    pub reshape_bb_env_state: *mut ia_mps_dec_reshape_bb_env_state_struct,
    pub sub_band_params: *mut ia_mps_dec_subband_tp_params_struct,
    pub blind_decoder: *mut ia_mps_dec_blind_decoder_struct,
    pub p_bs_frame: *mut ia_mps_dec_spatial_bs_frame_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_mps_tables_struct {
    pub qmf_table_ptr: *mut ia_mps_dec_qmf_tables_struct,
    pub common_table_ptr: *mut ia_mps_dec_common_tables_struct,
    pub hybrid_table_ptr: *mut ia_mps_dec_hybrid_tables_struct,
    pub m1_m2_table_ptr: *mut ia_mps_dec_m1_m2_tables_struct,
    pub decor_table_ptr: *mut ia_mps_dec_decorr_tables_struct,
    pub tp_process_table_ptr: *mut ia_mps_dec_tp_process_tables_struct,
    pub mdct2qmf_table_ptr: *mut ia_mps_dec_mdct2qmf_table_struct,
    pub tonality_table_ptr: *mut ia_mps_dec_tonality_tables_struct,
    pub bitdec_table_ptr: *mut ia_mps_dec_bitdec_tables_struct,
    pub blind_table_ptr: *mut ia_mps_dec_blind_tables_struct,
    pub mdct2qmfcos_table_ptr: *mut ia_mps_dec_mdct2qmf_tables_struct,
    pub mdct2qmfcos_tab_ptr: *mut ia_mps_dec_mdct2qmf_cos_table_struct,
    pub aac_tab: *mut core::ffi::c_void,
    pub wf_tab_ptr: *mut ia_mps_dec_wf_ptr_table_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_synthesis_interface {
    pub syn_filter_bank: Option<
        unsafe extern "C" fn(
            *mut ia_mps_dec_qmf_syn_filter_bank,
            *mut WORD32,
            *mut WORD32,
            *mut WORD32,
            WORD32,
            WORD32,
            WORD32,
            *mut ia_mps_dec_qmf_tables_struct,
        ) -> VOID,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_m1_param_struct {
    pub m1_param_real: [[[[WORD32; 28]; 8]; 6]; 8],
    pub m1_param_imag: [[[[WORD32; 28]; 8]; 6]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_reuse_array_struct {
    pub qmf_residual_real: *mut WORD32,
    pub qmf_residual_imag: *mut WORD32,
    pub qmf_residual_real_pre: *mut WORD32,
    pub qmf_residual_real_post: *mut WORD32,
    pub qmf_residual_imag_pre: *mut WORD32,
    pub qmf_residual_imag_post: *mut WORD32,
    pub res_mdct: *mut WORD32,
    pub time_out: *mut WORD32,
    pub x_real: *mut WORD32,
    pub x_imag: *mut WORD32,
    pub hyb_output_real_dry: *mut WORD32,
    pub hyb_output_imag_dry: *mut WORD32,
    pub env_dmx_0: *mut WORD32,
    pub env_dmx_1: *mut WORD32,
    pub m_qmf_real: *mut WORD32,
    pub m_qmf_imag: *mut WORD32,
    pub w_dry_real: *mut WORD32,
    pub w_dry_imag: *mut WORD32,
    pub buf_real: *mut WORD32,
    pub buf_imag: *mut WORD32,
    pub buffer_real: *mut WORD32,
    pub buffer_imag: *mut WORD32,
    pub m1_param: *mut ia_mps_dec_m1_param_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_m2_param_struct {
    pub m2_decor_real: [[[WORD32; 28]; 8]; 15],
    pub m2_decor_imag: [[[WORD32; 28]; 8]; 15],
    pub m2_resid_real: [[[WORD32; 28]; 8]; 19],
    pub m2_resid_imag: [[[WORD32; 28]; 8]; 19],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mps_dec_auxilary_struct {
    pub m2_param: *mut ia_mps_dec_m2_param_struct,
    pub temp_shape_enable_channel_stp: [WORD32; 8],
    pub temp_shape_enable_channel_ges: [WORD32; 8],
    pub env_shape_data: [[WORD32; 72]; 8],
    pub num_ott_bands: [WORD32; 5],
    pub ttt_config: [[ia_mps_dec_ttt_config_struct; 1]; 2],
    pub param_slot: [WORD32; 8],
    pub smg_time: [WORD32; 8],
    pub smg_data: [[WORD32; 28]; 8],
    pub ott_cld: [[[WORD32; 28]; 8]; 33],
    pub ott_icc: [[[WORD32; 28]; 8]; 5],
    pub ttt_cpc_1: [[[WORD32; 28]; 8]; 1],
    pub ttt_cpc_2: [[[WORD32; 28]; 8]; 1],
    pub ttt_cld_1: [[[WORD32; 28]; 8]; 1],
    pub ttt_cld_2: [[[WORD32; 28]; 8]; 1],
    pub ttt_icc: [[[WORD32; 28]; 8]; 1],
    pub arbdmx_gain: [[[WORD32; 28]; 8]; 6],
    pub arbdmx_residual_abs: [WORD32; 6],
    pub arbdmx_alpha_upd_set: [WORD32; 6],
    pub arbdmx_alpha: [WORD32; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_heaac_mps_state_struct {
    pub sac_time_align_flag: WORD32,
    pub sac_time_align: WORD32,
    pub sampling_freq: WORD32,
    pub tree_config: WORD32,
    pub num_input_channels: WORD32,
    pub num_output_channels: WORD32,
    pub num_ott_boxes: WORD32,
    pub num_ttt_boxes: WORD32,
    pub num_output_channels_at: WORD32,
    pub quant_mode: WORD32,
    pub one_icc: WORD32,
    pub arbitrary_downmix: WORD32,
    pub residual_coding: WORD32,
    pub smooth_config: WORD32,
    pub temp_shape_config: WORD32,
    pub decorr_config: WORD32,
    pub mtx_inversion: WORD32,
    pub _3d_stereo_inversion: WORD32,
    pub env_quant_mode: WORD32,
    pub clip_protect_gain: WORD32,
    pub surround_gain: WORD32,
    pub lfe_gain: WORD32,
    pub cpc_default: WORD32,
    pub icc_default: WORD32,
    pub arbdmx_gain_default: WORD32,
    pub num_direct_signals: WORD32,
    pub num_residual_signals: WORD32,
    pub num_decor_signals: WORD32,
    pub num_v_channels: WORD32,
    pub num_w_channels: WORD32,
    pub w_start_residual_idx: WORD32,
    pub num_x_channels: WORD32,
    pub time_slots: WORD32,
    pub cur_time_slot: WORD32,
    pub frame_length: WORD32,
    pub dec_type: WORD32,
    pub up_mix_type: WORD32,
    pub binaural_quality: WORD32,
    pub hrtf_model: WORD32,
    pub tp_hyb_band_border: WORD32,
    pub parse_next_bitstream_frame: WORD32,
    pub qmf_bands: WORD32,
    pub hybrid_bands: WORD32,
    pub residual_frames_per_spatial_frame: WORD32,
    pub upd_qmf: WORD32,
    pub arbdmx_residual_bands: WORD32,
    pub arbdmx_frames_per_spatial_frame: WORD32,
    pub arbdmx_upd_qmf: WORD32,
    pub bitstream_parameter_bands: WORD32,
    pub num_parameter_bands: WORD32,
    pub extend_frame: WORD32,
    pub num_parameter_sets: WORD32,
    pub num_parameter_sets_prev: WORD32,
    pub smooth_control: WORD32,
    pub i_bytes_consumed_mps: WORD32,
    pub bytes_remaining: WORD32,
    pub ui_mps_in_bytes: WORD32,
    pub is_sbr_present: WORD32,
    pub bits_per_sample: WORD32,
    pub qmf_input_delay_index: WORD32,
    pub m1_param_imag_present: WORD32,
    pub m2_param_imag_present: WORD32,
    pub m1_param_present: [[WORD32; 6]; 8],
    pub m2_param_present: [[WORD32; 8]; 8],
    pub index: [WORD32; 10],
    pub ott_cld_default: [WORD32; 5],
    pub ttt_cld_1_default: [WORD32; 1],
    pub ttt_cld_2_default: [WORD32; 1],
    pub kernels: [size_t; 71],
    pub res_bands: [WORD32; 10],
    pub ott_mode_lfe: [WORD32; 5],
    pub bitstream_ott_bands: [WORD32; 5],
    pub scaling_enable: WORD32,
    pub is_buried_flag: WORD32,
    pub sfband_info_tab: ia_mps_dec_residual_sfband_info_struct,
    pub pcm_out_buf: *mut WORD16,
    pub res_block_type: [[WORD32; 4]; 10],
    pub bs_config: ia_mps_spatial_bs_config_struct,
    pub ap_decor: [ia_mps_dec_decorr_dec_handle; 5],
    pub qmf_bank: [ia_mps_dec_qmf_ana_filter_bank; 6],
    pub syn_qmf_bank: ia_mps_dec_qmf_syn_filter_bank,
    pub mps_bit_buf: ia_bit_buf_struct,
    pub ptr_mps_bit_buff: *mut ia_bit_buf_struct,
    pub bs_frame: *mut ia_mps_dec_spatial_bs_frame_struct,
    pub array_struct: *mut ia_mps_dec_reuse_array_struct,
    pub aux_struct: *mut ia_mps_dec_auxilary_struct,
    pub mps_scratch_mem_v: *mut core::ffi::c_void,
    pub mps_persistent_mem: ia_mps_persistent_mem,
    pub mps_persistent_mem_v: *mut core::ffi::c_void,
    pub syn: *mut ia_mps_dec_synthesis_interface,
    pub p_aac_decoder_channel_info: [*mut ia_mps_dec_residual_channel_info_struct; 2],
    pub p_aac_decoder_dynamic_data_init: [*mut ia_mps_dec_residual_dynamic_data_struct; 2],
    pub tot_sf_bands_ls: [WORD8; 2],
    pub ia_mps_dec_mps_table: ia_mps_dec_mps_tables_struct,
    pub aac_table: ia_mps_dec_residual_aac_tables_struct,
    pub ia_mps_dec_mdct2qmfcos_table: ia_mps_dec_mdct2qmf_cos_table_struct,
    pub wf_tab: ia_mps_dec_wf_ptr_table_struct,
    pub is_first: WORD32,
    pub mps_decode: WORD32,
    pub temp_buf: [UWORD8; 1024],
    pub heaac_mps_present: WORD32,
    pub mps_with_sbr: WORD32,
    pub mps_init_done: WORD32,
    pub ec_flag: WORD32,
    pub frame_ok: WORD32,
    pub first_frame: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_peak_limiter_struct {
    pub attack_time: FLOAT32,
    pub release_time: FLOAT32,
    pub attack_constant: FLOAT32,
    pub release_constant: FLOAT32,
    pub limit_threshold: FLOAT32,
    pub num_channels: UWORD32,
    pub sample_rate: UWORD32,
    pub attack_time_samples: UWORD32,
    pub limiter_on: UWORD32,
    pub gain_modified: FLOAT32,
    pub pre_smoothed_gain: FLOAT64,
    pub delayed_input: *mut FLOAT32,
    pub delayed_input_index: UWORD32,
    pub max_buf: *mut FLOAT32,
    pub min_gain: FLOAT32,
    pub buffer: [FLOAT32; 16384],
    pub max_idx: WORD32,
    pub cir_buf_pnt: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_program_config_struct {
    pub element_instance_tag: WORD8,
    pub object_type: WORD32,
    pub samp_freq_index: WORD32,
    pub num_front_channel_elements: WORD32,
    pub num_side_channel_elements: WORD32,
    pub num_back_channel_elements: WORD32,
    pub num_lfe_channel_elements: WORD32,
    pub num_assoc_data_elements: WORD32,
    pub num_valid_cc_elements: WORD32,
    pub front_element_is_cpe: [WORD8; 16],
    pub front_element_tag_select: [WORD8; 16],
    pub side_element_is_cpe: [WORD8; 16],
    pub side_element_tag_select: [WORD8; 16],
    pub back_element_is_cpe: [WORD8; 16],
    pub back_element_tag_select: [WORD8; 16],
    pub lfe_element_tag_select: [WORD8; 16],
    pub channels: WORD32,
    pub alignment_bits: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_enhaacplus_dec_ind_cc {
    pub cc_target_is_cpe: [WORD8; 10],
    pub cc_target_tag_select: [WORD8; 10],
    pub cc_l: [WORD8; 10],
    pub cc_r: [WORD8; 10],
    pub cc_gain: [WORD32; 20],
    pub elements_coupled: [WORD8; 10],
    pub num_coupled_elements: WORD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_config_struct {
    pub ui_pcm_wdsz: UWORD32,
    pub ui_samp_freq: UWORD32,
    pub ui_n_channels: UWORD32,
    pub i_channel_mask: WORD32,
    pub ui_channel_mode: UWORD32,
    pub ui_sbr_mode: UWORD32,
    pub ui_effect_type: WORD32,
    pub ui_target_loudness: WORD32,
    pub ui_loud_norm_flag: WORD32,
    pub flag_downmix: UWORD32,
    pub flag_08khz_out: UWORD32,
    pub flag_16khz_out: UWORD32,
    pub flag_to_stereo: UWORD32,
    pub down_sample_flag: UWORD32,
    pub header_dec_done: UWORD32,
    pub ui_mp4_flag: UWORD32,
    pub ui_disable_sync: UWORD32,
    pub ui_auto_sbr_upsample: UWORD32,
    pub frame_status: WORD32,
    pub ui_max_channels: UWORD32,
    pub ui_pce_found_in_hdr: UWORD32,
    pub ui_n_memtabs: UWORD32,
    pub ui_drc_boost: WORD32,
    pub ui_drc_cut: WORD32,
    pub ui_drc_target_level: WORD32,
    pub ui_drc_set: WORD32,
    pub ui_drc_heavy_comp: WORD32,
    pub str_prog_config: ia_program_config_struct,
    pub element_type: [WORD32; 11],
    pub slot_element: [WORD32; 11],
    pub element_instance_order: [WORD; 10],
    pub ui_coupling_channel: WORD,
    pub downmix: WORD,
    pub loas_present: WORD32,
    pub framesize_480: WORD,
    pub ld_decoder: WORD,
    pub eld_sbr_present: WORD,
    pub ui_out_channels: UWORD32,
    pub ui_channel_mask: WORD32,
    pub ui_dec_type: WORD32,
    pub ui_qmf_bands: UWORD32,
    pub ui_flush_cmd: WORD32,
    pub drc_config_struct: ia_drc_config,
    pub output_level: WORD32,
    pub i_loud_ref_level: WORD32,
    pub dup_stereo_flag: UWORD8,
    pub peak_limiter_off: WORD32,
    pub mps_present: WORD32,
    pub ui_frame_size: UWORD32,
    pub ui_enh_sbr: WORD32,
    pub ui_hq_esbr: WORD32,
    pub ui_enh_sbr_ps: WORD32,
    pub ui_usac_flag: WORD32,
    pub ui_err_conceal: WORD32,
    pub first_frame: FLAG,
    pub ui_loudness_leveling_flag: WORD32,
    pub ui_drc_mode_cut: UWORD8,
    pub ui_drc_mode_boost: UWORD8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_aac_dec_state_struct {
    pub p_config: *mut ia_aac_dec_config_struct,
    pub audio_object_type: AUDIO_OBJECT_TYPE,
    pub ui_in_bytes: UWORD32,
    pub ui_out_bytes: UWORD32,
    pub ui_exec_done: UWORD32,
    pub b_n_raw_data_blk: WORD16,
    pub s_adts_hdr_present: WORD16,
    pub s_adif_hdr_present: WORD16,
    pub num_channel_last: WORD16,
    pub sampling_rate: UWORD32,
    pub extension_samp_rate: UWORD32,
    pub bit_rate: UWORD32,
    pub ui_init_done: UWORD32,
    pub ui_input_over: UWORD32,
    pub header_dec_done: UWORD32,
    pub frame_counter: WORD32,
    pub pstr_aac_dec_info: [*mut ia_aac_decoder_struct; 10],
    pub ch_config: UWORD32,
    pub str_bit_buf: ia_bit_buf_struct,
    pub pstr_bit_buf: *mut ia_bit_buf_struct,
    pub pstr_stream_sbr: *mut [ia_aac_dec_sbr_bitstream_struct; 2],
    pub str_sbr_dec_info: [ia_handle_sbr_dec_inst_struct; 10],
    pub sbr_present_flag: WORD32,
    pub ps_present: WORD32,
    pub ptr_bit_stream: *mut ia_bit_buf_struct,
    pub aac_scratch_mem_v: *mut core::ffi::c_void,
    pub aac_persistent_mem_v: *mut core::ffi::c_void,
    pub sbr_persistent_mem_v: *mut core::ffi::c_void,
    pub ptr_overlap_buf: *mut WORD32,
    pub num_of_out_samples: WORD16,
    pub last_frame_ok: WORD32,
    pub i_bytes_consumed: WORD32,
    pub coup_ch_output: *mut WORD32,
    pub ind_cc_info: ia_enhaacplus_dec_ind_cc,
    pub protection_absent: WORD8,
    pub crc_check: WORD32,
    pub ui_flush_cmd: WORD32,
    pub frame_len_flag: WORD32,
    pub depends_on_core_coder: WORD32,
    pub extension_flag: WORD32,
    pub bs_format: WORD32,
    pub bit_count: WORD32,
    pub sync_status: WORD32,
    pub extension_flag_3: WORD32,
    pub latm_struct_element: ixheaacd_latm_struct,
    pub latm_initialized: WORD32,
    pub str_drc_dec_info: ia_drc_dec_struct,
    pub drc_dummy: ia_drc_dec_struct,
    pub pstr_drc_dec: *mut ia_drc_dec_struct,
    pub pstr_hdrc_data: [*mut ixheaac_drc_data_struct; 30],
    pub prev_channel_mode: WORD32,
    pub drc_cut_fac: WORD32,
    pub drc_boost_fac: WORD32,
    pub first_drc_frame: WORD32,
    pub str_err_config: ia_aac_err_config_struct,
    pub frame_size: WORD32,
    pub frame_length: WORD32,
    pub dwnsmp_signal: WORD32,
    pub eld_specific_config: ia_eld_specific_config_struct,
    pub usac_flag: FLAG,
    pub num_of_output_ch: WORD32,
    pub ia_audio_specific_config: *mut core::ffi::c_void,
    pub mps_dec_handle: ia_mps_dec_state_struct,
    pub heaac_mps_handle: ia_heaac_mps_state_struct,
    pub mps_buffer: [UWORD8; 1024],
    pub huffman_code_book_scl: *mut UWORD16,
    pub huffman_code_book_scl_index: *mut UWORD32,
    pub pstr_aac_tables: *mut ia_aac_dec_tables_struct,
    pub pstr_dec_data: *mut core::ffi::c_void,
    pub sbr_persistent_mem_u: *mut core::ffi::c_void,
    pub sbr_scratch_mem_u: *mut core::ffi::c_void,
    pub header_ptr: *mut UWORD8,
    pub header_length: WORD32,
    pub str_sbr_config: ia_sbr_header_data_struct,
    pub xaac_jmp_buf: jmp_buf,
    pub decode_create_done: WORD32,
    pub ldmps_present: WORD32,
    pub fatal_err_present: WORD32,
    pub pers_mem_ptr: *mut WORD8,
    pub preroll_config_present: UWORD8,
    pub preroll_config_prev: [UWORD8; 285],
    pub qshift_cnt: UWORD8,
    pub qshift_adj: [WORD8; 16],
    pub delay_in_samples: UWORD32,
    pub peak_lim_init: UWORD8,
    pub peak_limiter: ia_peak_limiter_struct,
    pub sbr_present: UWORD8,
    pub slot_pos: UWORD8,
    pub mps_header: WORD32,
    pub ui_mps_out_bytes: WORD32,
    pub drc_config_changed: WORD32,
    pub apply_crossfade: WORD32,
    pub ec_enable: WORD32,
    pub first_frame: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_exhaacplus_dec_api_struct {
    pub p_state_aac: *mut ia_aac_dec_state_struct,
    pub aac_config: ia_aac_dec_config_struct,
    pub p_mem_info_aac: *mut ia_mem_info_struct,
    pub pp_mem_aac: *mut pVOID,
    pub aac_tables: ia_aac_dec_tables_struct,
    pub common_tables: *mut ixheaacd_misc_tables,
    pub str_sbr_tables: ia_sbr_tables_struct,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_audio_specific_config_struct {
    pub audio_object_type: UINT32,
    pub samp_frequency_index: UINT32,
    pub sampling_frequency: UINT32,
    pub channel_configuration: UINT32,
    pub ext_audio_object_type: UINT32,
    pub ext_samp_frequency_index: UINT32,
    pub ext_sampling_frequency: UINT32,
    pub sbr_present_flag: UINT32,
    pub frame_length: UINT32,
    pub str_prog_config: *mut ia_prog_config_struct,
    pub str_usac_config: ia_usac_config_struct,
    pub num_front_channel: WORD32,
    pub num_side_channel: WORD32,
    pub num_back_channel: WORD32,
    pub num_lfe_channel: WORD32,
    pub num_ind_cce: WORD32,
    pub avg_bit_rate: UINT32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_prog_config_struct {
    pub tag: WORD32,
    pub profile: WORD32,
    pub sampling_rate_idx: WORD32,
    pub front: ia_ele_list_struct,
    pub side: ia_ele_list_struct,
    pub back: ia_ele_list_struct,
    pub lfe: ia_ele_list_struct,
    pub data: ia_ele_list_struct,
    pub coupling: ia_ele_list_struct,
    pub mono_mix: ia_mix_dwn_struct,
    pub stereo_mix: ia_mix_dwn_struct,
    pub matrix_mix: ia_mix_dwn_struct,
    pub comments: [WORD8; 257],
    pub buffer_fullness: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_mix_dwn_struct {
    pub present: WORD32,
    pub ele_tag: WORD32,
    pub pseudo_enab: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_ele_list_struct {
    pub num_ele: WORD32,
    pub ele_is_cpe: [WORD32; 16],
    pub ele_tag: [WORD32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_dec_data_struct {
    pub dec_bit_buf: ia_bit_buf_struct,
    pub str_frame_data: ia_frame_data_struct,
    pub str_usac_data: ia_usac_data_struct,
    pub xaac_jmp_buf: *mut jmp_buf,
}
pub type ia_usac_data_struct = ia_usac_data_main_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_data_main_struct {
    pub time_sample_vector: [[FLOAT32; 4096]; 6],
    pub time_sample_vector_prev: [[FLOAT32; 4096]; 6],
    pub input_data_ptr: [[WORD32; 4096]; 6],
    pub overlap_data_ptr: [[WORD32; 4096]; 6],
    pub output_data_ptr: [[WORD32; 4096]; 6],
    pub window_shape: [WORD32; 6],
    pub window_shape_prev: [WORD32; 6],
    pub window_sequence: [WORD32; 6],
    pub window_sequence_last: [WORD32; 6],
    pub output_samples: WORD32,
    pub sbr_ratio_idx: WORD32,
    pub usac_independency_flg: WORD32,
    pub sampling_rate_idx: WORD32,
    pub audio_object_type: WORD32,
    pub down_samp_sbr: WORD32,
    pub sbr_mode: WORD32,
    pub tw_mdct: [WORD32; 68],
    pub mps_pseudo_lr: [WORD32; 68],
    pub td_frame_prev: [WORD32; 6],
    pub warp_sum: [[FLOAT32; 2]; 6],
    pub warp_cont_mem: [[FLOAT32; 3072]; 6],
    pub prev_sample_pos: [[FLOAT32; 3072]; 6],
    pub prev_tw_trans_len: [[FLOAT32; 2]; 6],
    pub prev_tw_start_stop: [[WORD32; 2]; 6],
    pub prev_warped_time_sample_vector: [[FLOAT32; 3072]; 6],
    pub lpc_prev: [[FLOAT32; 17]; 6],
    pub acelp_in: [[FLOAT32; 257]; 6],
    pub alpha_q_re: [[WORD32; 136]; 8],
    pub alpha_q_im: [[WORD32; 136]; 8],
    pub cplx_pred_used: [[UWORD8; 136]; 8],
    pub alpha_q_re_prev: [WORD32; 136],
    pub alpha_q_im_prev: [WORD32; 136],
    pub dmx_re_prev: [WORD32; 1024],
    pub sbr_scratch_mem_base: *mut core::ffi::c_void,
    pub coef_fix: [*mut WORD32; 6],
    pub coef: [*mut FLOAT32; 6],
    pub ms_used: [*mut UWORD8; 6],
    pub coef_save: [*mut WORD32; 6],
    pub factors: [*mut WORD16; 6],
    pub group_dis: [*mut UWORD8; 6],
    pub tw_data_present: [WORD32; 6],
    pub tw_ratio: [*mut WORD32; 6],
    pub pstr_tns: [*mut ia_tns_frame_info_struct; 6],
    pub str_tddec: [ia_usac_lpd_decoder_handle; 6],
    pub arith_prev_n: [WORD32; 6],
    pub c_prev: [[WORD8; 516]; 6],
    pub c: [[WORD8; 516]; 6],
    pub noise_filling_config: [WORD32; 16],
    pub seed_value: [UWORD32; 6],
    pub present_chan: WORD32,
    pub fac_data_present: [WORD32; 6],
    pub fac_data: [[WORD32; 129]; 6],
    pub pstr_sfb_info: [*mut ia_sfb_info_struct; 6],
    pub str_only_long_info: ia_sfb_info_struct,
    pub str_eight_short_info: ia_sfb_info_struct,
    pub pstr_usac_winmap: [*mut ia_sfb_info_struct; 5],
    pub sfb_width_short: [WORD16; 16],
    pub ccfl: WORD32,
    pub len_subfrm: WORD32,
    pub num_subfrm: WORD32,
    pub pstr_esbr_dec: ia_handle_sbr_dec_inst_struct,
    pub esbr_bit_str: [ia_aac_dec_sbr_bitstream_struct; 2],
    pub x_ac_dec: [WORD32; 1024],
    pub scratch_buffer: [WORD32; 1024],
    pub synth_buf: [FLOAT32; 1883],
    pub exc_buf: [FLOAT32; 1453],
    pub lp_flt_coff: [FLOAT32; 290],
    pub pitch: [WORD32; 25],
    pub pitch_gain: [FLOAT32; 25],
    pub huffman_code_book_scl: *mut UWORD16,
    pub huffman_code_book_scl_index: *mut UWORD32,
    pub tns_coeff3_32: *mut WORD32,
    pub tns_coeff4_32: *mut WORD32,
    pub tns_max_bands_tbl_usac: *mut [[WORD32; 2]; 16],
    pub sfb_width_long: [WORD16; 64],
    pub usac_flag: WORD32,
    pub arr_coef_fix: [[WORD32; 1152]; 6],
    pub arr_coef: [[FLOAT32; 1152]; 6],
    pub arr_coef_save: [[WORD32; 1152]; 6],
    pub arr_factors: [[WORD16; 128]; 6],
    pub arr_group_dis: [[UWORD8; 8]; 6],
    pub arr_tw_ratio: [[WORD32; 16]; 6],
    pub arr_ms_used: [[UWORD8; 128]; 6],
    pub arr_str_tddec: [ia_usac_lpd_decoder; 6],
    pub arr_str_tns: [ia_tns_frame_info_struct; 6],
    pub enh_sbr: WORD32,
    pub esbr_hq: WORD32,
    pub enh_sbr_ps: WORD32,
    pub drc_config_changed: WORD32,
    pub core_mode: WORD32,
    pub frame_ok: WORD32,
    pub sbr_parse_err_flag: WORD32,
    pub last_frame_ok: WORD32,
    pub ec_flag: WORD32,
    pub first_frame: WORD32,
    pub sbr_parse_complete: WORD32,
    pub max_sfb: [UWORD8; 2],
    pub num_ch_out: WORD32,
    pub spec_scale: [[WORD16; 128]; 6],
    pub str_error_concealment: [ia_ec_state_str; 6],
    pub pstr_td_frame: *mut ia_td_frame_data_struct,
    pub sampling_rate: WORD32,
    pub td_frame_prev_ec: [WORD32; 6],
    pub lsp_coeff: [[FLOAT32; 16]; 5],
    pub lsf_adaptive_mean_cand: [FLOAT32; 16],
    pub lsf_adaptive_mean: [FLOAT32; 16],
    pub lpc4_lsf: [FLOAT32; 16],
    pub bpf_control_info: WORD32,
    pub first_lpd_flag: WORD32,
    pub short_fac_flag: WORD32,
    pub core_mode_last: WORD32,
    pub stability_factor_old: FLOAT32,
    pub num_lost_lpd_frames: [WORD32; 6],
    pub pitch_lag_old: WORD32,
    pub pitch_lag_frac_old: WORD32,
    pub pitch_lag: WORD32,
    pub pitch_lag_frac: WORD32,
    pub seed_ace: WORD16,
    pub pstr_ec_state: *mut ia_ec_state_str,
    pub past_pitch_gain: FLOAT32,
    pub past_gain_code: FLOAT32,
    pub past_gain_tcx: [FLOAT32; 6],
    pub tcx_spec_coeffs: [[WORD32; 1280]; 6],
    pub lspold_ec: [FLOAT32; 16],
    pub lp_flt_coff_a_ec: [FLOAT32; 17],
    pub td_frame_data_prev: [ia_td_frame_data_struct; 6],
    pub last_shiftp: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_td_frame_data_struct {
    pub acelp_core_mode: WORD32,
    pub mod_0: [WORD32; 4],
    pub fac: [WORD32; 512],
    pub fac_data: [WORD32; 129],
    pub mean_energy: [WORD32; 4],
    pub acb_index: [WORD32; 16],
    pub noise_factor: [WORD32; 4],
    pub global_gain: [WORD32; 4],
    pub arith_reset_flag: WORD32,
    pub x_tcx_invquant: [WORD32; 1024],
    pub tcx_lg: [WORD32; 16],
    pub ltp_filtering_flag: [WORD32; 16],
    pub icb_index: [[WORD32; 8]; 16],
    pub gains: [WORD32; 16],
    pub mode_lpc: [WORD32; 4],
    pub lpc_first_approx_idx: [WORD32; 110],
    pub lsp_coeff: [[FLOAT32; 16]; 5],
    pub lsf_adaptive_mean_cand: [FLOAT32; 16],
    pub lsf_adaptive_mean: [FLOAT32; 16],
    pub lpc4_lsf: [FLOAT32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tns_frame_info_struct {
    pub n_subblocks: WORD32,
    pub str_tns_info: [ia_tns_info_struct; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tns_info_struct {
    pub n_filt: WORD32,
    pub coef_res: WORD32,
    pub str_filter: [ia_tns_filter_struct; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_tns_filter_struct {
    pub start_band: WORD32,
    pub stop_band: WORD32,
    pub order: WORD32,
    pub direction: WORD32,
    pub coef_compress: WORD32,
    pub coef: [WORD16; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_usac_lpd_decoder {
    pub mode_prev: WORD32,
    pub synth_prev: [core::ffi::c_float; 859],
    pub xcitation_prev: [core::ffi::c_float; 428],
    pub pitch_prev: [core::ffi::c_int; 7],
    pub gain_prev: [core::ffi::c_float; 7],
    pub lp_flt_coeff_a_prev: [core::ffi::c_float; 34],
    pub exc_prev: [FLOAT32; 257],
    pub bpf_prev: [FLOAT32; 76],
    pub ilspold: [WORD32; 16],
    pub fac_gain: FLOAT32,
    pub fac_fd_data: [FLOAT32; 32],
    pub lsf_prev: [FLOAT32; 16],
    pub lspold: [FLOAT32; 16],
    pub lsfold_first: [WORD32; 16],
    pub gain_threshold: FLOAT32,
    pub fscale: WORD32,
    pub fd_synth_buf: [FLOAT32; 785],
    pub fd_synth: *mut FLOAT32,
    pub bpf_active_prev: WORD32,
    pub last_tcx_pitch: WORD32,
    pub synth_prev_ec: [FLOAT32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_sfb_info_struct {
    pub islong: WORD32,
    pub max_win_len: WORD32,
    pub samp_per_bk: WORD32,
    pub sfb_per_bk: WORD32,
    pub bins_per_sbk: WORD32,
    pub sfb_per_sbk: WORD32,
    pub ptr_sfb_tbl: *const WORD16,
    pub sfb_width: pWORD16,
    pub sfb_idx_tbl: [WORD16; 125],
    pub num_groups: WORD32,
    pub group_len: [WORD16; 8],
}
pub type ia_usac_lpd_decoder_handle = *mut ia_usac_lpd_decoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_frame_data_struct {
    pub stream_count: UINT32,
    pub str_audio_specific_config: ia_audio_specific_config_struct,
    pub str_layer: ia_layer_data_struct,
    pub tracks_in_layer: WORD32,
    pub scal_out_select: UWORD32,
    pub scal_out_object_type: WORD32,
    pub scal_out_num_channels: WORD32,
    pub scal_out_sampling_frequency: WORD32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ia_layer_data_struct {
    pub sample_rate_layer: WORD32,
    pub bit_rate: WORD32,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const MAX_CC_CHANNEL_NUM: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_BS_ELEMENT: core::ffi::c_int = 8 as core::ffi::c_int + MAX_CC_CHANNEL_NUM;
pub const IA_MEMTYPE_PERSIST: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_MEMTYPE_SCRATCH: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const IA_MEMTYPE_INPUT: core::ffi::c_int = 0x2 as core::ffi::c_int;
pub const IA_MEMTYPE_OUTPUT: core::ffi::c_int = 0x3 as core::ffi::c_int;
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
unsafe extern "C" fn ixheaac_round16(mut op1: WORD32) -> WORD16 {
    let mut var_out: WORD16 = 0;
    var_out = (ixheaac_add32_sat(op1, 0x8000 as WORD32) >> 16 as core::ffi::c_int)
        as WORD16;
    return var_out;
}
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_FATAL_ERROR: core::ffi::c_uint = 0x80000000 as core::ffi::c_uint;
pub const IA_API_CMD_GET_LIB_ID_STRINGS: core::ffi::c_int = 1;
pub const IA_API_CMD_GET_API_SIZE: core::ffi::c_int = 2;
pub const IA_API_CMD_INIT: core::ffi::c_int = 3;
pub const IA_API_CMD_SET_CONFIG_PARAM: core::ffi::c_int = 4;
pub const IA_API_CMD_GET_CONFIG_PARAM: core::ffi::c_int = 5;
pub const IA_API_CMD_GET_MEMTABS_SIZE: core::ffi::c_int = 6;
pub const IA_API_CMD_SET_MEMTABS_PTR: core::ffi::c_int = 7;
pub const IA_API_CMD_GET_N_MEMTABS: core::ffi::c_int = 8;
pub const IA_API_CMD_EXECUTE: core::ffi::c_int = 9;
pub const IA_API_CMD_GET_CURIDX_INPUT_BUF: core::ffi::c_int = 11;
pub const IA_API_CMD_SET_INPUT_BYTES: core::ffi::c_int = 12;
pub const IA_API_CMD_GET_OUTPUT_BYTES: core::ffi::c_int = 13;
pub const IA_API_CMD_INPUT_OVER: core::ffi::c_int = 14;
pub const IA_API_CMD_GET_MEM_INFO_SIZE: core::ffi::c_int = 0x11 as core::ffi::c_int;
pub const IA_API_CMD_GET_MEM_INFO_ALIGNMENT: core::ffi::c_int = 18;
pub const IA_API_CMD_GET_MEM_INFO_TYPE: core::ffi::c_int = 19;
pub const IA_API_CMD_SET_MEM_PTR: core::ffi::c_int = 0x16 as core::ffi::c_int;
pub const IA_API_CMD_GET_N_TABLES: core::ffi::c_int = 25;
pub const IA_API_CMD_GET_TABLE_INFO_SIZE: core::ffi::c_int = 0x1a as core::ffi::c_int;
pub const IA_API_CMD_GET_TABLE_INFO_ALIGNMENT: core::ffi::c_int = 27;
pub const IA_API_CMD_SET_TABLE_PTR: core::ffi::c_int = 0x1d as core::ffi::c_int;
pub const IA_API_CMD_GET_TABLE_PTR: core::ffi::c_int = 0x1e as core::ffi::c_int;
pub const IA_API_CMD_GET_LOUDNESS_VAL: core::ffi::c_int = 31;
pub const IA_CMD_TYPE_LIB_NAME: core::ffi::c_int = 0x100 as core::ffi::c_int;
pub const IA_CMD_TYPE_LIB_VERSION: core::ffi::c_int = 0x200 as core::ffi::c_int;
pub const IA_CMD_TYPE_INIT_API_PRE_CONFIG_PARAMS: core::ffi::c_int = 256;
pub const IA_CMD_TYPE_INIT_API_POST_CONFIG_PARAMS: core::ffi::c_int = 512;
pub const IA_CMD_TYPE_INIT_PROCESS: core::ffi::c_int = 768;
pub const IA_CMD_TYPE_INIT_DONE_QUERY: core::ffi::c_int = 1024;
pub const IA_CMD_TYPE_GA_HDR: core::ffi::c_int = 2048;
pub const IA_CMD_TYPE_FLUSH_MEM: core::ffi::c_int = 4096;
pub const IA_CMD_TYPE_DO_EXECUTE: core::ffi::c_int = 256;
pub const IA_CMD_TYPE_DONE_QUERY: core::ffi::c_int = 512;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_PCM_WDSZ: core::ffi::c_int = 0;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_SAMP_FREQ: core::ffi::c_int = 1;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_NUM_CHANNELS: core::ffi::c_int = 0x2
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_CHANNEL_MASK: core::ffi::c_int = 0x3
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_EFFECT_TYPE: core::ffi::c_int = 6;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_TARGET_LOUDNESS: core::ffi::c_int = 7;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DOWNMIX: core::ffi::c_int = 9;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_TOSTEREO: core::ffi::c_int = 10;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DSAMPLE: core::ffi::c_int = 11;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_MP4FLAG: core::ffi::c_int = 12;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_MAX_CHANNEL: core::ffi::c_int = 13;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_COUP_CHANNEL: core::ffi::c_int = 14;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DOWNMIX_STEREO: core::ffi::c_int = 15;
pub const IA_XHEAAC_DEC_CONFIG_DISABLE_SYNC: core::ffi::c_int = 16;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_AUTO_SBR_UPSAMPLE: core::ffi::c_int = 17;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_CUT: core::ffi::c_int = 0x12
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_BOOST: core::ffi::c_int = 0x13
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_TARGET_LEVEL: core::ffi::c_int = 20;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_HEAVY_COMP: core::ffi::c_int = 21;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_FRAMESIZE: core::ffi::c_int = 22;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_LD_TESTING: core::ffi::c_int = 23;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_HQ_ESBR: core::ffi::c_int = 24;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_PS_ENABLE: core::ffi::c_int = 25;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_AOT: core::ffi::c_int = 0x1a as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_PEAK_LIMITER: core::ffi::c_int = 27;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_FRAMELENGTH_FLAG: core::ffi::c_int = 28;
pub const IA_XHEAAC_DEC_CONFIG_ERROR_CONCEALMENT: core::ffi::c_int = 29;
pub const IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_PTR: core::ffi::c_int = 0x1e
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_BUF_SIZES: core::ffi::c_int = 0x1f
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_NUM_ELE: core::ffi::c_int = 0x20 as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_NUM_CONFIG_EXT: core::ffi::c_int = 0x21
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_GAIN_PAYLOAD_LEN: core::ffi::c_int = 0x22
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_GAIN_PAYLOAD_BUF: core::ffi::c_int = 0x23
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_CONFIG_GET_NUM_PRE_ROLL_FRAMES: core::ffi::c_int = 0x24
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_DRC_IS_CONFIG_CHANGED: core::ffi::c_int = 0x25
    as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_DRC_APPLY_CROSSFADE: core::ffi::c_int = 0x26
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_ESBR: core::ffi::c_int = 40;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_LOUDNESS_LEVELING: core::ffi::c_int = 0x29
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_MODE_CUT: core::ffi::c_int = 0x2a
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_PARAM_DRC_MODE_BOOST: core::ffi::c_int = 0x2b
    as core::ffi::c_int;
pub const LIBNAME: [core::ffi::c_char; 14] = unsafe {
    ::core::mem::transmute::<[u8; 14], [core::ffi::c_char; 14]>(*b"IA_XHEAAC_DEC\0")
};
pub const IA_ENHAACPLUS_DEC_PERSIST_IDX: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_SCRATCH_IDX: core::ffi::c_int = 1 as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_INPUT_IDX: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_OUTPUT_IDX: core::ffi::c_int = 3 as core::ffi::c_int;
pub const IA_MAX_PREROLL_FRAMES: core::ffi::c_int = 4 as core::ffi::c_int;
pub const IA_MAX_OUTPUT_PCM_SIZE: core::ffi::c_int = 3 as core::ffi::c_int;
pub const IA_MAX_USAC_CH: core::ffi::c_int = 2 as core::ffi::c_int;
pub const IA_MAX_OUT_SAMPLES_PER_FRAME: core::ffi::c_int = 4096 as core::ffi::c_int;
pub const IA_ENHAACPLUS_DEC_OUT_BUF_SIZE: core::ffi::c_int = IA_MAX_USAC_CH
    * IA_MAX_PREROLL_FRAMES * IA_MAX_OUT_SAMPLES_PER_FRAME * IA_MAX_OUTPUT_PCM_SIZE;
pub const IA_XHEAAC_DEC_API_NONFATAL_CMD_TYPE_NOT_SUPPORTED: core::ffi::c_int = 0x2
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_API_FATAL_INVALID_MEMTAB_INDEX: core::ffi::c_uint = 0xffff8000
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_API_FATAL_INVALID_LIB_ID_STRINGS_IDX: core::ffi::c_uint = 0xffff8001
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_API_FATAL_MEM_ALLOC: core::ffi::c_uint = 0xffff8002
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_API_FATAL_INVALID_CONFIG_PARAM: core::ffi::c_uint = 0xffff8003
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_API_FATAL_INVALID_EXECUTE_TYPE: core::ffi::c_uint = 0xffff8004
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_API_FATAL_INVALID_CMD: core::ffi::c_uint = 0xffff8005
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_API_FATAL_MEM_ALIGN: core::ffi::c_uint = 0xffff8006
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_PCM_WDSZ: core::ffi::c_int = 0x800
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_DOWNMIX: core::ffi::c_int = 0x801
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_ESBR_PS_FLAG: core::ffi::c_int = 0x802
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_DOWNMIX_STEREO: core::ffi::c_int = 0x803
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_TOSTEREO: core::ffi::c_int = 0x804
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_DSAMPLE: core::ffi::c_int = 0x805
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_PEAK_LIM_FLAG_TYPE: core::ffi::c_int = 0x806
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_MP4FLAG: core::ffi::c_int = 0x807
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_MAX_CHANNEL: core::ffi::c_int = 0x808
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_COUP_CHANNEL: core::ffi::c_int = 0x809
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_ERR_CONCEAL_FLAG_TYPE: core::ffi::c_int = 0x80a
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_SYNCFLAG: core::ffi::c_int = 0x80b
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_SBRUPFLAG: core::ffi::c_int = 0x80c
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_DRCFLAG: core::ffi::c_int = 0x80d
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_DRC_CUT: core::ffi::c_int = 0x80e
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_DRC_BOOST: core::ffi::c_int = 0x80f
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_DRC_TARGET: core::ffi::c_int = 0x810
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_FRAMSZ: core::ffi::c_int = 0x811
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_LD_CONFIG: core::ffi::c_int = 0x812
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_EFFECT_TYPE: core::ffi::c_int = 0x813
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_TARGET_LOUDNESS: core::ffi::c_int = 0x814
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_ESBR_HQ_FLAG: core::ffi::c_int = 0x815
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_FRAMELENGTHFLAG: core::ffi::c_int = 0x816
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_ESBR_FLAG: core::ffi::c_int = 0x817
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_LOUDNESS_LEVELING_FLAG: core::ffi::c_int = 0x818
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_CONFIG_FATAL_INVALID_SAMPLE_RATE: core::ffi::c_uint = 0xffff8800
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_INIT_NONFATAL_HEADER_NOT_AT_START: core::ffi::c_int = 0x1000
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_INIT_NONFATAL_EC_INIT_FAIL: core::ffi::c_int = 0x1002
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_INIT_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1003
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_INIT_NONFATAL_DECODE_FRAME_ERROR: core::ffi::c_int = 0x1004
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL: core::ffi::c_uint = 0xffff9000
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_INIT_FATAL_EO_INPUT_REACHED: core::ffi::c_uint = 0xffff9001
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_INIT_FATAL_STREAM_CHAN_GT_MAX: core::ffi::c_uint = 0xffff9002
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_INIT_FATAL_EC_INIT_FAIL: core::ffi::c_uint = 0xffff9004
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_INIT_FATAL_UNIMPLEMENTED_CCE: core::ffi::c_uint = 0xffff9005
    as core::ffi::c_uint;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_ADTS_SYNC_LOST: core::ffi::c_int = 0x1800
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_SBR_TURNED_OFF: core::ffi::c_int = 0x1801
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_SBR_TURNED_ON: core::ffi::c_int = 0x1802
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_ELE_INSTANCE_TAG_NOT_FOUND: core::ffi::c_int = 0x1805
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_DECODE_FRAME_ERROR: core::ffi::c_int = 0x1807
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_CHANGED_ADTS_SF: core::ffi::c_int = 0x180c
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_FATAL_UNIMPLEMENTED_CCE: core::ffi::c_uint = 0xffff9800
    as core::ffi::c_uint;
pub const ER_OBJECT_START: core::ffi::c_int = 17 as core::ffi::c_int;
pub const SBR_UPSAMPLE_FAC: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_FRAME_SIZE: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const NO_SYNTHESIS_CHANNELS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const NO_ANALYSIS_CHANNELS: core::ffi::c_int = NO_SYNTHESIS_CHANNELS
    / SBR_UPSAMPLE_FAC;
pub const LPC_ORDER: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_COLS: core::ffi::c_int = MAX_FRAME_SIZE / NO_ANALYSIS_CHANNELS;
pub const MAX_OV_COLS: core::ffi::c_int = 6 as core::ffi::c_int;
pub const MAX_ENV_COLS: core::ffi::c_int = MAX_COLS + MAX_OV_COLS;
pub const MAX_QMF_BUF_LEN: core::ffi::c_int = 78 as core::ffi::c_int;
pub const SBRDEC_OK: core::ffi::c_int = 0 as core::ffi::c_int;
pub const ADTS_BSFORMAT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const LOAS_BSFORMAT: core::ffi::c_int = 3 as core::ffi::c_int;
pub const IA_MAX_INP_BUFFER_SIZE: core::ffi::c_int = 8 as core::ffi::c_int
    * 1024 as core::ffi::c_int + 11 as core::ffi::c_int;
pub const MAX_CHANNEL_COUNT: core::ffi::c_int = 128 as core::ffi::c_int;
pub const MAX_AUDIO_PREROLLS: core::ffi::c_int = 3 as core::ffi::c_int;
pub const xHE_AAC_DEC_ITTIAM_VER: [core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [core::ffi::c_char; 18]>(*b"_X86 $Rev: 1.41 $\0")
};
pub const MAXNRSBRCHANNELS: core::ffi::c_int = 2 as core::ffi::c_int;
pub const NUM_MPS_TABLES: core::ffi::c_int = 13 as core::ffi::c_int;
pub const IA_ENHAACPDEC_NUM_MEMTABS: core::ffi::c_int = 4 as core::ffi::c_int;
pub const NUM_AAC_TABLES: core::ffi::c_int = 8 as core::ffi::c_int;
pub const LD_OBJ: core::ffi::c_int = -(2 as core::ffi::c_int);
pub const SCR_BASE_SCR_8K_SIZE: usize = ((2 as core::ffi::c_int * 2 as core::ffi::c_int
    * 1024 as core::ffi::c_int) as usize)
    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
    .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize;
pub const SCR_EXTRA_SCR_4K_0_SIZE: usize = (2 as usize)
    .wrapping_mul(
        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>() as usize)
            .wrapping_add(
                (::core::mem::size_of::<WORD32>() as usize).wrapping_sub(1 as usize),
            ) & !(::core::mem::size_of::<WORD32>() as usize).wrapping_sub(1 as usize),
    )
    .wrapping_add(
        (2 as usize)
            .wrapping_mul(
                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>() as usize)
                    .wrapping_add(
                        (::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_sub(1 as usize),
                    )
                    & !(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_sub(1 as usize),
            ),
    )
    .wrapping_add(
        (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
            .wrapping_add(
                (::core::mem::size_of::<WORD32>() as usize).wrapping_sub(1 as usize),
            ) & !(::core::mem::size_of::<WORD32>() as usize).wrapping_sub(1 as usize),
    );
pub const SCR_EXTRA_SCR_4K_2_SIZE: usize = (1024 as usize)
    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
    .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize;
pub const SCR_EXTRA_SCR_4K_3_SIZE: usize = (1024 as usize)
    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
    .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize;
pub const SCR_OUT_DATA_SIZE: usize = (1024 as usize)
    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
    .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize;
pub const SCR_IN_DATA_SIZE: usize = (2 as usize)
    .wrapping_mul(
        (1024 as usize)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
    );
pub const SCR_INTER_SCR_SIZE: usize = (MAX_CHANNEL_COUNT as usize)
    .wrapping_mul(
        (1024 as usize)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
    );
pub const SCR_COUP_CH_OUT_SIZE: usize = (MAX_CHANNEL_COUNT as usize)
    .wrapping_mul(
        (1024 as usize)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
    );
pub const P_IND_CH_INFO_OFFSET: usize = SCR_BASE_SCR_8K_SIZE
    .wrapping_add(SCR_EXTRA_SCR_4K_0_SIZE)
    .wrapping_add(SCR_EXTRA_SCR_4K_2_SIZE);
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_mem_api(
    mut p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
    mut i_cmd: WORD32,
    mut i_idx: WORD32,
    mut pv_value: *mut core::ffi::c_void,
) -> WORD32 {
    let mut pui_value: pUWORD32 = pv_value as pUWORD32;
    if i_idx < 0 as core::ffi::c_int || i_idx >= IA_ENHAACPDEC_NUM_MEMTABS {
        return IA_XHEAAC_DEC_API_FATAL_INVALID_MEMTAB_INDEX as WORD32;
    }
    if i_cmd == IA_API_CMD_SET_MEM_PTR {
        if pv_value.is_null() {
            return 0xffff8002 as WORD32;
        }
        if (pv_value as size_t)
            .wrapping_rem(
                (*((*p_obj_exhaacplus_dec).p_mem_info_aac).offset(i_idx as isize))
                    .ui_alignment as size_t,
            ) != 0 as size_t
        {
            return 0xffff8006 as WORD32;
        }
        let ref mut fresh0 = *((*p_obj_exhaacplus_dec).pp_mem_aac)
            .offset(i_idx as isize);
        *fresh0 = pv_value as pVOID;
        memset(
            *((*p_obj_exhaacplus_dec).pp_mem_aac).offset(i_idx as isize),
            0 as core::ffi::c_int,
            (*((*p_obj_exhaacplus_dec).p_mem_info_aac).offset(i_idx as isize)).ui_size
                as size_t,
        );
        if i_idx == IA_ENHAACPLUS_DEC_PERSIST_IDX {
            let mut p_temp: pUWORD8 = pv_value as pUWORD8;
            let mut meminfo: *mut UWORD32 = ((*p_obj_exhaacplus_dec).p_mem_info_aac
                as *mut UWORD32)
                .offset(i_idx as isize);
            let mut pers_size: UWORD32 = *meminfo.offset(0 as core::ffi::c_int as isize);
            p_temp = p_temp
                .offset(pers_size as isize)
                .offset(
                    -(((::core::mem::size_of::<ia_dec_data_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (::core::mem::size_of::<ia_audio_specific_config_struct>()
                                as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (8300 as core::ffi::c_int
                                + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as usize,
                        ) as isize),
                );
            (*p_obj_exhaacplus_dec).p_state_aac = pv_value
                as *mut ia_aac_dec_state_struct;
            memset(
                (*p_obj_exhaacplus_dec).p_state_aac as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<ia_aac_dec_state_struct>() as size_t)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
            );
            (*(*p_obj_exhaacplus_dec).p_state_aac).pstr_dec_data = p_temp
                as *mut core::ffi::c_void;
            (*(*p_obj_exhaacplus_dec).p_state_aac).ia_audio_specific_config = p_temp
                .offset(
                    ((::core::mem::size_of::<ia_dec_data_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as isize,
                ) as *mut core::ffi::c_void;
            (*(*p_obj_exhaacplus_dec).p_state_aac).header_ptr = p_temp
                .offset(
                    ((::core::mem::size_of::<ia_dec_data_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as isize,
                )
                .offset(
                    ((::core::mem::size_of::<ia_audio_specific_config_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        as isize,
                ) as *mut UWORD8;
        }
    } else {
        let mut meminfo_0: *mut UWORD32 = ((*p_obj_exhaacplus_dec).p_mem_info_aac)
            .offset(i_idx as isize) as *mut UWORD32;
        *pui_value = *meminfo_0
            .offset((i_cmd as core::ffi::c_int - IA_API_CMD_GET_MEM_INFO_SIZE) as isize)
            as core::ffi::c_uint;
    }
    return IA_NO_ERROR;
}
#[inline]
unsafe extern "C" fn ixheaacd_init_sbr_tables(
    mut ptr_sbr_tables: *mut ia_sbr_tables_struct,
) -> VOID {
    (*ptr_sbr_tables).env_calc_tables_ptr = &ixheaacd_aac_dec_env_calc_tables
        as *const ia_env_calc_tables_struct as *mut ia_env_calc_tables_struct;
    (*ptr_sbr_tables).qmf_dec_tables_ptr = &ixheaacd_aac_qmf_dec_tables
        as *const ia_qmf_dec_tables_struct as *mut ia_qmf_dec_tables_struct;
    (*ptr_sbr_tables).env_extr_tables_ptr = &ixheaacd_aac_dec_env_extr_tables
        as *const ia_env_extr_tables_struct as *mut ia_env_extr_tables_struct;
    (*ptr_sbr_tables).ps_tables_ptr = &ixheaacd_aac_dec_ps_tables
        as *const ia_ps_tables_struct as *mut ia_ps_tables_struct;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_updatebytesconsumed(
    mut p_state_enhaacplus_dec: *mut ia_aac_dec_state_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
) -> VOID {
    (*p_state_enhaacplus_dec).i_bytes_consumed = ((*it_bit_buff).ptr_read_next)
        .offset_from((*it_bit_buff).ptr_bit_buf_base) as core::ffi::c_long as WORD32;
    if (*p_state_enhaacplus_dec).i_bytes_consumed == 0 as core::ffi::c_int
        && (*it_bit_buff).cnt_bits == 0 as core::ffi::c_int
    {
        (*p_state_enhaacplus_dec).i_bytes_consumed = (*p_state_enhaacplus_dec)
            .ui_in_bytes as WORD32;
    }
    if (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int {
        (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int as WORD32;
        (*p_state_enhaacplus_dec).ui_out_bytes = 0 as UWORD32;
        (*p_state_enhaacplus_dec).ui_mps_out_bytes = 0 as core::ffi::c_int as WORD32;
        (*p_state_enhaacplus_dec).b_n_raw_data_blk = 0 as WORD16;
    }
}
unsafe extern "C" fn copy_qmf_to_ldmps(
    mut mps_dec_handle: *mut ia_mps_dec_state_struct,
    mut sbr_persistent_mem_v: *mut core::ffi::c_void,
) -> VOID {
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = sbr_persistent_mem_v
        as *mut ia_sbr_pers_struct;
    memcpy(
        &mut (*mps_dec_handle).str_mps_qmf_bank as *mut ia_sbr_qmf_filter_bank_struct
            as *mut core::ffi::c_void,
        &mut (**((*sbr_persistent_mem).str_sbr_dec_inst.pstr_sbr_channel)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .str_sbr_dec
            .str_codec_qmf_bank as *mut ia_sbr_qmf_filter_bank_struct
            as *const core::ffi::c_void,
        ::core::mem::size_of::<ia_sbr_qmf_filter_bank_struct>() as size_t,
    );
    (*mps_dec_handle).sbr_tables_ptr = (*sbr_persistent_mem)
        .str_sbr_dec_inst
        .pstr_sbr_tables;
    (*mps_dec_handle).qmf_dec_tables_ptr = (*(*sbr_persistent_mem)
        .str_sbr_dec_inst
        .pstr_sbr_tables)
        .qmf_dec_tables_ptr;
    (*mps_dec_handle).str_sbr_scale_fact = &mut (**((*sbr_persistent_mem)
        .str_sbr_dec_inst
        .pstr_sbr_channel)
        .as_mut_ptr()
        .offset(0 as core::ffi::c_int as isize))
        .str_sbr_dec
        .str_sbr_scale_fact;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_readifadts(
    mut p_state_enhaacplus_dec: *mut ia_aac_dec_state_struct,
    mut it_bit_buff: *mut ia_bit_buf_struct,
    mut adts: *mut ia_adts_header_struct,
) -> WORD32 {
    let mut error: WORD = 0;
    error = ixheaacd_find_syncword(adts, it_bit_buff) as WORD;
    if error != 0 as core::ffi::c_int {
        ixheaacd_updatebytesconsumed(p_state_enhaacplus_dec, it_bit_buff);
        return IA_XHEAAC_DEC_EXE_NONFATAL_ADTS_SYNC_LOST;
    }
    error = ixheaacd_check_if_adts(
        adts,
        it_bit_buff,
        (*(*p_state_enhaacplus_dec).p_config).ui_max_channels as WORD32,
    ) as WORD;
    if error != 0 as core::ffi::c_int {
        (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int as WORD32;
        if (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int {
            (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int as WORD32;
            (*p_state_enhaacplus_dec).ui_out_bytes = 0 as UWORD32;
            (*p_state_enhaacplus_dec).ui_mps_out_bytes = 0 as core::ffi::c_int as WORD32;
            error = IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES as WORD;
            return error as WORD32;
        }
        return IA_XHEAAC_DEC_EXE_NONFATAL_ADTS_SYNC_LOST;
    }
    (*p_state_enhaacplus_dec).b_n_raw_data_blk = ((*adts).no_raw_data_blocks
        as core::ffi::c_int + 1 as core::ffi::c_int) as WORD8 as WORD16;
    return 0 as WORD32;
}
unsafe extern "C" fn ixheaacd_allocate_aac_scr(
    mut aac_scratch_struct: *mut ia_aac_dec_scratch_struct,
    mut base_scratch_ptr: *mut core::ffi::c_void,
    mut output_ptr: *mut core::ffi::c_void,
    mut channel: WORD,
    mut max_channel: WORD,
    mut audio_object_type: WORD32,
) -> VOID {
    let mut scratch_used: WORD32 = 0 as WORD32;
    (*aac_scratch_struct).base_scr_8k = base_scratch_ptr;
    (*aac_scratch_struct).extra_scr_4k[1 as core::ffi::c_int as usize] = base_scratch_ptr
        as *mut WORD8 as *mut core::ffi::c_void;
    scratch_used = (scratch_used as core::ffi::c_ulong)
        .wrapping_add(SCR_BASE_SCR_8K_SIZE as core::ffi::c_ulong) as WORD32 as WORD32;
    if channel == 1 as core::ffi::c_int {
        (*aac_scratch_struct).extra_scr_4k[0 as core::ffi::c_int as usize] = (base_scratch_ptr
            as *mut WORD8)
            .offset(scratch_used as isize) as *mut core::ffi::c_void;
        scratch_used = (scratch_used as core::ffi::c_ulong)
            .wrapping_add(SCR_EXTRA_SCR_4K_0_SIZE as core::ffi::c_ulong) as WORD32
            as WORD32;
        (*aac_scratch_struct).extra_scr_4k[2 as core::ffi::c_int as usize] = (base_scratch_ptr
            as *mut WORD8)
            .offset(scratch_used as isize) as *mut core::ffi::c_void;
        scratch_used = (scratch_used as core::ffi::c_ulong)
            .wrapping_add(SCR_EXTRA_SCR_4K_2_SIZE as core::ffi::c_ulong) as WORD32
            as WORD32;
    } else {
        (*aac_scratch_struct).extra_scr_4k[0 as core::ffi::c_int as usize] = output_ptr;
        if max_channel > 2 as core::ffi::c_int {
            (*aac_scratch_struct).extra_scr_4k[0 as core::ffi::c_int as usize] = (base_scratch_ptr
                as *mut WORD8)
                .offset(scratch_used as isize) as *mut core::ffi::c_void;
            scratch_used = (scratch_used as core::ffi::c_ulong)
                .wrapping_add(SCR_EXTRA_SCR_4K_0_SIZE as core::ffi::c_ulong) as WORD32
                as WORD32;
        }
        (*aac_scratch_struct).extra_scr_4k[2 as core::ffi::c_int as usize] = (base_scratch_ptr
            as *mut WORD8)
            .offset(scratch_used as isize) as *mut core::ffi::c_void;
        scratch_used = (scratch_used as core::ffi::c_ulong)
            .wrapping_add(SCR_EXTRA_SCR_4K_2_SIZE as core::ffi::c_ulong) as WORD32
            as WORD32;
    }
    if audio_object_type == AOT_ER_AAC_ELD as core::ffi::c_int
        || audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
    {
        (*aac_scratch_struct).extra_scr_4k[0 as core::ffi::c_int as usize] = (base_scratch_ptr
            as *mut WORD8)
            .offset(scratch_used as isize) as *mut core::ffi::c_void;
        scratch_used = (scratch_used as core::ffi::c_ulong)
            .wrapping_add(SCR_EXTRA_SCR_4K_0_SIZE as core::ffi::c_ulong) as WORD32
            as WORD32;
        (*aac_scratch_struct).extra_scr_4k[2 as core::ffi::c_int as usize] = (base_scratch_ptr
            as *mut WORD8)
            .offset(scratch_used as isize) as *mut core::ffi::c_void;
        scratch_used = (scratch_used as core::ffi::c_ulong)
            .wrapping_add(SCR_EXTRA_SCR_4K_2_SIZE as core::ffi::c_ulong) as WORD32
            as WORD32;
        (*aac_scratch_struct).extra_scr_4k[3 as core::ffi::c_int as usize] = (base_scratch_ptr
            as *mut WORD8)
            .offset(scratch_used as isize) as *mut core::ffi::c_void;
        scratch_used = (scratch_used as core::ffi::c_ulong)
            .wrapping_add(SCR_EXTRA_SCR_4K_3_SIZE as core::ffi::c_ulong) as WORD32
            as WORD32;
    }
    if audio_object_type == AOT_ER_AAC_LD as core::ffi::c_int
        || audio_object_type == AOT_AAC_LTP as core::ffi::c_int
    {
        (*aac_scratch_struct).out_data = (base_scratch_ptr as *mut WORD8)
            .offset(scratch_used as isize) as *mut WORD32;
        scratch_used = (scratch_used as core::ffi::c_ulong)
            .wrapping_add(SCR_OUT_DATA_SIZE as core::ffi::c_ulong) as WORD32 as WORD32;
        (*aac_scratch_struct).in_data = (base_scratch_ptr as *mut WORD8)
            .offset(scratch_used as isize) as *mut WORD32;
        scratch_used = (scratch_used as core::ffi::c_ulong)
            .wrapping_add(SCR_IN_DATA_SIZE as core::ffi::c_ulong) as WORD32 as WORD32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_allocate_sbr_scr(
    mut sbr_scratch_struct: *mut ia_sbr_scr_struct,
    mut base_scratch_ptr: *mut core::ffi::c_void,
    mut output_ptr: *mut core::ffi::c_void,
    mut total_channels: WORD32,
    mut p_qshift_arr: *mut WORD8,
    mut slot_pos: UWORD8,
    mut num_ch: UWORD8,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut i: WORD32 = 0;
    (*sbr_scratch_struct).ptr_work_buf_core = base_scratch_ptr;
    if !p_qshift_arr.is_null() && *p_qshift_arr as core::ffi::c_int != LD_OBJ {
        let mut tmp_buf: *mut WORD32 = output_ptr as *mut WORD32;
        j = 1 as core::ffi::c_int as WORD32;
        while j < num_ch as core::ffi::c_int {
            if *p_qshift_arr as WORD32 + j == 0 as core::ffi::c_int {
                *p_qshift_arr.offset(j as isize) = *p_qshift_arr
                    .offset(j as isize)
                    .offset(-(1 as core::ffi::c_int as isize));
            }
            j += 1;
        }
        if total_channels > 2 as core::ffi::c_int {
            j = 0 as core::ffi::c_int as WORD32;
            while j < num_ch as core::ffi::c_int {
                i = 0 as core::ffi::c_int as WORD32;
                while i < 1024 as core::ffi::c_int {
                    *(tmp_buf as *mut WORD16)
                        .offset(slot_pos as core::ffi::c_int as isize)
                        .offset((total_channels * i) as isize)
                        .offset(j as isize) = ixheaac_round16(
                        ixheaac_shl32_sat(
                            *tmp_buf
                                .offset(slot_pos as core::ffi::c_int as isize)
                                .offset((total_channels * i) as isize)
                                .offset(j as isize),
                            *p_qshift_arr.offset(j as isize) as WORD,
                        ),
                    );
                    i += 1;
                }
                j += 1;
            }
        } else {
            j = 0 as core::ffi::c_int as WORD32;
            while j < num_ch as core::ffi::c_int {
                i = 0 as core::ffi::c_int as WORD32;
                while i < 1024 as core::ffi::c_int {
                    *(tmp_buf as *mut WORD16)
                        .offset((total_channels * i) as isize)
                        .offset(j as isize) = ixheaac_round16(
                        ixheaac_shl32_sat(
                            *tmp_buf
                                .offset((total_channels * i) as isize)
                                .offset(j as isize),
                            *p_qshift_arr.offset(j as isize) as WORD,
                        ),
                    );
                    i += 1;
                }
                j += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_get_lib_id_strings(mut pv_output: pVOID) -> VOID {
    let mut pstr_lib_info: *mut ia_lib_info_struct = pv_output
        as *mut ia_lib_info_struct;
    (*pstr_lib_info).p_lib_name = LIBNAME.as_ptr() as *mut WORD8;
    (*pstr_lib_info).p_version_num = xHE_AAC_DEC_ITTIAM_VER.as_ptr() as *mut WORD8;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_api(
    mut p_ia_xheaac_dec_obj: pVOID,
    mut i_cmd: WORD32,
    mut i_idx: WORD32,
    mut pv_value: pVOID,
) -> IA_ERRORCODE {
    let mut p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct = p_ia_xheaac_dec_obj
        as *mut ia_exhaacplus_dec_api_struct;
    let mut pui_value: pUWORD32 = pv_value as pUWORD32;
    let mut pui_value_signed: pWORD32 = pv_value as pWORD32;
    let mut pb_value: pWORD8 = pv_value as pWORD8;
    let mut pp_value: *mut pVOID = pv_value as *mut pVOID;
    let mut pf_value: *mut core::ffi::c_float = pv_value as *mut core::ffi::c_float;
    if i_cmd != IA_API_CMD_GET_API_SIZE && i_cmd != IA_API_CMD_GET_LIB_ID_STRINGS {
        if p_ia_xheaac_dec_obj.is_null() {
            return 0xffff8002 as IA_ERRORCODE;
        }
        if p_ia_xheaac_dec_obj as size_t & 3 as size_t != 0 as size_t {
            return 0xffff8006 as IA_ERRORCODE;
        }
    }
    match i_cmd {
        IA_API_CMD_GET_MEM_INFO_SIZE
        | IA_API_CMD_GET_MEM_INFO_ALIGNMENT
        | IA_API_CMD_GET_MEM_INFO_TYPE
        | IA_API_CMD_SET_MEM_PTR => {
            return ixheaacd_dec_mem_api(
                p_ia_xheaac_dec_obj as *mut ia_exhaacplus_dec_api_struct,
                i_cmd,
                i_idx,
                pv_value as *mut core::ffi::c_void,
            ) as IA_ERRORCODE;
        }
        IA_API_CMD_GET_TABLE_INFO_SIZE
        | IA_API_CMD_GET_TABLE_INFO_ALIGNMENT
        | IA_API_CMD_SET_TABLE_PTR
        | IA_API_CMD_GET_TABLE_PTR => {
            return ixheaacd_dec_table_api(
                p_ia_xheaac_dec_obj as *mut ia_exhaacplus_dec_api_struct,
                i_cmd,
                i_idx,
                pv_value,
            ) as IA_ERRORCODE;
        }
        _ => {}
    }
    match i_cmd {
        IA_API_CMD_GET_LIB_ID_STRINGS => {
            let mut i1_ver: *mut WORD8 = 0 as *mut WORD8;
            let mut ver_char: WORD8 = 0;
            if i_idx == IA_CMD_TYPE_LIB_NAME {
                i1_ver = LIBNAME.as_ptr() as *mut WORD8;
            } else if i_idx == IA_CMD_TYPE_LIB_VERSION {
                i1_ver = xHE_AAC_DEC_ITTIAM_VER.as_ptr() as *mut WORD8;
            } else {
                return IA_XHEAAC_DEC_API_FATAL_INVALID_LIB_ID_STRINGS_IDX as IA_ERRORCODE
            }
            let fresh5 = i1_ver;
            i1_ver = i1_ver.offset(1);
            ver_char = *fresh5;
            while ver_char as core::ffi::c_int != '\0' as i32 {
                if ver_char as core::ffi::c_int != '$' as i32 {
                    let fresh6 = pb_value;
                    pb_value = pb_value.offset(1);
                    *fresh6 = ver_char as core::ffi::c_schar;
                }
                let fresh7 = i1_ver;
                i1_ver = i1_ver.offset(1);
                ver_char = *fresh7;
            }
            *pb_value = ver_char as core::ffi::c_schar;
        }
        IA_API_CMD_GET_API_SIZE => {
            *pui_value = ::core::mem::size_of::<ia_exhaacplus_dec_api_struct>()
                as core::ffi::c_uint;
        }
        IA_API_CMD_INIT => {
            match i_idx {
                IA_CMD_TYPE_INIT_API_PRE_CONFIG_PARAMS => {
                    memset(
                        p_obj_exhaacplus_dec as *mut core::ffi::c_void,
                        0 as core::ffi::c_int,
                        ::core::mem::size_of::<ia_exhaacplus_dec_api_struct>() as size_t,
                    );
                    (*p_obj_exhaacplus_dec).aac_config.ui_pcm_wdsz = 16 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.flag_downmix = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.flag_08khz_out = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.flag_16khz_out = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.down_sample_flag = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.header_dec_done = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 1
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_mp4_flag = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_disable_sync = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_auto_sbr_upsample = 1
                        as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_samp_freq = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_frame_size = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_n_channels = 2 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.i_channel_mask = 3
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_channel_mode = 3 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_sbr_mode = 0 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_effect_type = 0
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_target_loudness = -(24
                        as core::ffi::c_int) as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_loud_norm_flag = 0
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_hq_esbr = 0 as core::ffi::c_int
                        as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr = 1 as core::ffi::c_int
                        as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr_ps = 0
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_pce_found_in_hdr = 0
                        as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.loas_present = 0
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ld_decoder = 0 as core::ffi::c_int
                        as WORD;
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_boost = 100
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_cut = 100
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_mode_cut = 0 as UWORD8;
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_mode_boost = 0 as UWORD8;
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_target_level = 108
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_set = 0 as core::ffi::c_int
                        as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_flush_cmd = 0
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.output_level = -(1
                        as core::ffi::c_int) as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_loudness_leveling_flag = 1
                        as core::ffi::c_int as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_max_channels = 6 as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.ui_coupling_channel = 0
                        as core::ffi::c_int as WORD;
                    (*p_obj_exhaacplus_dec).aac_config.downmix = 0 as core::ffi::c_int
                        as WORD;
                    (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal = 0
                        as core::ffi::c_int as WORD32;
                    let mut pstr_aac_tables: *mut ia_aac_dec_tables_struct = &mut (*p_obj_exhaacplus_dec)
                        .aac_tables;
                    (*pstr_aac_tables).pstr_huffmann_tables = &ixheaacd_aac_huffmann_tables
                        as *const ia_aac_dec_huffman_tables_struct
                        as *mut ia_aac_dec_huffman_tables_struct;
                    (*pstr_aac_tables).pstr_block_tables = &ixheaacd_aac_block_tables
                        as *const ia_aac_dec_block_tables_struct
                        as *mut ia_aac_dec_block_tables_struct;
                    (*pstr_aac_tables).pstr_imdct_tables = &ixheaacd_imdct_tables
                        as *const ia_aac_dec_imdct_tables_struct
                        as *mut ia_aac_dec_imdct_tables_struct;
                    ixheaacd_huff_tables_create(pstr_aac_tables);
                    ixheaacd_init_sbr_tables(
                        &mut (*p_obj_exhaacplus_dec).str_sbr_tables,
                    );
                    (*p_obj_exhaacplus_dec).common_tables = &ixheaacd_str_fft_n_transcendent_tables
                        as *const ixheaacd_misc_tables as *mut ixheaacd_misc_tables;
                    (*p_obj_exhaacplus_dec).aac_config.ui_qmf_bands = 64 as UWORD32;
                }
                IA_CMD_TYPE_INIT_API_POST_CONFIG_PARAMS => {
                    ixheaacd_fill_aac_mem_tables(p_obj_exhaacplus_dec);
                }
                IA_CMD_TYPE_INIT_PROCESS => {
                    let mut err_code: WORD32 = 0 as WORD32;
                    if (*(*p_obj_exhaacplus_dec).p_state_aac).fatal_err_present != 0 {
                        err_code = IA_FATAL_ERROR as WORD32;
                    } else {
                        err_code = ixheaacd_dec_init(p_obj_exhaacplus_dec);
                        if err_code != 0
                            && (*(*p_obj_exhaacplus_dec).p_state_aac).s_adts_hdr_present
                                as core::ffi::c_int != 0
                        {
                            (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done = 0
                                as UWORD32;
                        }
                        if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0
                            && err_code != 0
                        {
                            if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
                                err_code = IA_XHEAAC_DEC_INIT_FATAL_EC_INIT_FAIL as WORD32;
                            } else {
                                err_code = IA_XHEAAC_DEC_INIT_NONFATAL_EC_INIT_FAIL
                                    as WORD32;
                            }
                        }
                    }
                    if err_code != 0 as core::ffi::c_int {
                        if err_code < 0 as core::ffi::c_int {
                            (*(*p_obj_exhaacplus_dec).p_state_aac).fatal_err_present = 1
                                as core::ffi::c_int as WORD32;
                        }
                        (*(*p_obj_exhaacplus_dec).p_state_aac).i_bytes_consumed = (*(*p_obj_exhaacplus_dec)
                            .p_state_aac)
                            .ui_in_bytes as WORD32;
                    }
                    return err_code as IA_ERRORCODE;
                }
                IA_CMD_TYPE_INIT_DONE_QUERY => {
                    if (*(*p_obj_exhaacplus_dec).p_state_aac).ui_init_done
                        == 1 as UWORD32
                    {
                        *pui_value = 1 as core::ffi::c_uint;
                    } else {
                        *pui_value = 0 as core::ffi::c_uint;
                    }
                }
                IA_CMD_TYPE_GA_HDR => {
                    return ixheaacd_decoder_2_ga_hdr(p_obj_exhaacplus_dec)
                        as IA_ERRORCODE;
                }
                IA_CMD_TYPE_FLUSH_MEM => {
                    return ixheaacd_decoder_flush_api(p_obj_exhaacplus_dec)
                        as IA_ERRORCODE;
                }
                _ => return IA_XHEAAC_DEC_API_NONFATAL_CMD_TYPE_NOT_SUPPORTED,
            }
        }
        IA_API_CMD_SET_CONFIG_PARAM => {
            match i_idx {
                IA_XHEAAC_DEC_CONFIG_PARAM_SAMP_FREQ => {
                    if *pui_value < 8000 as core::ffi::c_uint
                        || *pui_value > 96000 as core::ffi::c_uint
                    {
                        return 0xffff8800 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_samp_freq = *pui_value
                        as UWORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_FRAMELENGTH_FLAG => {
                    if *pui_value != 1 as core::ffi::c_uint
                        && *pui_value != 0 as core::ffi::c_uint
                    {
                        return 0x816 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_frame_size = *pui_value
                        as UWORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_PCM_WDSZ => {
                    if *pui_value != 16 as core::ffi::c_uint
                        && *pui_value != 24 as core::ffi::c_uint
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_pcm_wdsz = 16 as UWORD32;
                        return 0x800 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_pcm_wdsz = *pui_value
                        as UWORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DOWNMIX => {
                    if *pui_value != 1 as core::ffi::c_uint
                        && *pui_value != 0 as core::ffi::c_uint
                    {
                        (*p_obj_exhaacplus_dec).aac_config.flag_downmix = 0 as UWORD32;
                        return 0x801 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.flag_downmix = *pui_value
                        as UWORD32;
                    (*p_obj_exhaacplus_dec).aac_config.downmix = *pui_value as WORD;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_TOSTEREO => {
                    if *pui_value != 1 as core::ffi::c_uint
                        && *pui_value != 0 as core::ffi::c_uint
                    {
                        (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo = 1 as UWORD32;
                        return 0x804 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo = *pui_value
                        as UWORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DSAMPLE => {
                    if *pui_value != 1 as core::ffi::c_uint
                        && *pui_value != 0 as core::ffi::c_uint
                    {
                        (*p_obj_exhaacplus_dec).aac_config.down_sample_flag = 0
                            as UWORD32;
                        return 0x805 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.down_sample_flag = *pui_value
                        as UWORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_MP4FLAG => {
                    if *pui_value != 1 as core::ffi::c_uint
                        && *pui_value != 0 as core::ffi::c_uint
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_mp4_flag = 0 as UWORD32;
                        return 0x807 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_mp4_flag = *pui_value
                        as UWORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_CUT => {
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_set = 1 as core::ffi::c_int
                        as WORD32;
                    if *pf_value > 1 as core::ffi::c_int as core::ffi::c_float
                        || *pf_value < 0 as core::ffi::c_int as core::ffi::c_float
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_drc_cut = 0
                            as core::ffi::c_int as WORD32;
                        return 0x80e as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_mode_cut = 1 as UWORD8;
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_cut = (*pf_value
                        * 100 as core::ffi::c_int as core::ffi::c_float) as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_BOOST => {
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_set = 1 as core::ffi::c_int
                        as WORD32;
                    if *pf_value > 1 as core::ffi::c_int as core::ffi::c_float
                        || *pf_value < 0 as core::ffi::c_int as core::ffi::c_float
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_drc_boost = 0
                            as core::ffi::c_int as WORD32;
                        return 0x80f as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_mode_boost = 1 as UWORD8;
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_boost = (*pf_value
                        * 100 as core::ffi::c_int as core::ffi::c_float) as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_TARGET_LEVEL => {
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_set = 1 as core::ffi::c_int
                        as WORD32;
                    (*p_obj_exhaacplus_dec).aac_config.i_loud_ref_level = *pui_value_signed
                        as WORD32;
                    if *pui_value > 127 as core::ffi::c_uint {
                        (*p_obj_exhaacplus_dec).aac_config.ui_drc_target_level = 108
                            as core::ffi::c_int as WORD32;
                        return 0x810 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_target_level = *pui_value
                        as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_HEAVY_COMP => {
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_set = 1 as core::ffi::c_int
                        as WORD32;
                    if *pui_value != 1 as core::ffi::c_uint
                        && *pui_value != 0 as core::ffi::c_uint
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_drc_heavy_comp = 0
                            as core::ffi::c_int as WORD32;
                        return 0x80d as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_drc_heavy_comp = *pui_value
                        as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_DISABLE_SYNC => {
                    if *pui_value != 1 as core::ffi::c_uint
                        && *pui_value != 0 as core::ffi::c_uint
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_disable_sync = 0
                            as UWORD32;
                        return 0x80b as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_disable_sync = *pui_value
                        as UWORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_AUTO_SBR_UPSAMPLE => {
                    if *pui_value != 1 as core::ffi::c_uint
                        && *pui_value != 0 as core::ffi::c_uint
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_auto_sbr_upsample = 0
                            as UWORD32;
                        return 0x80c as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_auto_sbr_upsample = *pui_value
                        as UWORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_MAX_CHANNEL => {
                    if *pui_value > 8 as core::ffi::c_uint {
                        (*p_obj_exhaacplus_dec).aac_config.ui_max_channels = 8
                            as UWORD32;
                        return 0x808 as IA_ERRORCODE;
                    }
                    if *pui_value < 2 as core::ffi::c_uint {
                        (*p_obj_exhaacplus_dec).aac_config.ui_max_channels = 2
                            as UWORD32;
                        return 0x808 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_max_channels = *pui_value
                        as UWORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_FRAMESIZE => {
                    if *pui_value == 1 as core::ffi::c_uint {
                        (*p_obj_exhaacplus_dec).aac_config.framesize_480 = 1
                            as core::ffi::c_int as WORD;
                    } else if *pui_value == 0 as core::ffi::c_uint {
                        (*p_obj_exhaacplus_dec).aac_config.framesize_480 = 0
                            as core::ffi::c_int as WORD;
                    } else {
                        return 0x811 as IA_ERRORCODE
                    }
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_LD_TESTING => {
                    if *pui_value == 1 as core::ffi::c_uint {
                        (*p_obj_exhaacplus_dec).aac_config.ld_decoder = 1
                            as core::ffi::c_int as WORD;
                    } else if *pui_value == 0 as core::ffi::c_uint {
                        (*p_obj_exhaacplus_dec).aac_config.ld_decoder = 0
                            as core::ffi::c_int as WORD;
                    } else {
                        return 0x812 as IA_ERRORCODE
                    }
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_COUP_CHANNEL => {
                    if *pui_value > 16 as core::ffi::c_uint {
                        (*p_obj_exhaacplus_dec).aac_config.ui_coupling_channel = 1
                            as core::ffi::c_int as WORD;
                        return 0x809 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_coupling_channel = *pui_value
                        as WORD;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DOWNMIX_STEREO => {
                    if *pui_value != 1 as core::ffi::c_uint
                        && *pui_value != 0 as core::ffi::c_uint
                    {
                        (*p_obj_exhaacplus_dec).aac_config.downmix = 0
                            as core::ffi::c_int as WORD;
                        return 0x803 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.downmix = *pui_value as WORD;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_EFFECT_TYPE => {
                    if *pui_value_signed > 8 as core::ffi::c_int
                        || *pui_value_signed < -(1 as core::ffi::c_int)
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_effect_type = -(1
                            as core::ffi::c_int) as WORD32;
                        return 0x813 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_effect_type = *pui_value_signed
                        as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_TARGET_LOUDNESS => {
                    if *pui_value_signed >= 0 as core::ffi::c_int {
                        (*p_obj_exhaacplus_dec).aac_config.ui_loud_norm_flag = 1
                            as core::ffi::c_int as WORD32;
                    }
                    *pui_value_signed = -(*pui_value_signed >> 2 as core::ffi::c_int);
                    if *pui_value_signed > 0 as core::ffi::c_int
                        || *pui_value_signed < -(63 as core::ffi::c_int)
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_target_loudness = 0
                            as core::ffi::c_int as WORD32;
                        return 0x814 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_target_loudness = *pui_value_signed
                        as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_HQ_ESBR => {
                    if *pui_value_signed != 0 as core::ffi::c_int
                        && *pui_value_signed != 1 as core::ffi::c_int
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_hq_esbr = 0
                            as core::ffi::c_int as WORD32;
                        return 0x815 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_hq_esbr = *pui_value_signed
                        as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_PS_ENABLE => {
                    if *pui_value_signed != 0 as core::ffi::c_int
                        && *pui_value_signed != 1 as core::ffi::c_int
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr_ps = 0
                            as core::ffi::c_int as WORD32;
                        return 0x802 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr_ps = *pui_value_signed
                        as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_PEAK_LIMITER => {
                    if *pui_value_signed != 0 as core::ffi::c_int
                        && *pui_value_signed != 1 as core::ffi::c_int
                    {
                        (*p_obj_exhaacplus_dec).aac_config.peak_limiter_off = 0
                            as core::ffi::c_int as WORD32;
                        return 0x806 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.peak_limiter_off = *pui_value_signed
                        as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_ERROR_CONCEALMENT => {
                    if *pui_value_signed != 0 as core::ffi::c_int
                        && *pui_value_signed != 1 as core::ffi::c_int
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal = 0
                            as core::ffi::c_int as WORD32;
                        return 0x80a as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal = *pui_value_signed
                        as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_ESBR => {
                    if *pui_value_signed != 0 as core::ffi::c_int
                        && *pui_value_signed != 1 as core::ffi::c_int
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr = 1
                            as core::ffi::c_int as WORD32;
                        return 0x817 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr = *pui_value_signed
                        as WORD32;
                }
                IA_XHEAAC_DEC_CONFIG_PARAM_DRC_LOUDNESS_LEVELING => {
                    if *pui_value_signed != 0 as core::ffi::c_int
                        && *pui_value_signed != 1 as core::ffi::c_int
                    {
                        (*p_obj_exhaacplus_dec).aac_config.ui_loudness_leveling_flag = 1
                            as core::ffi::c_int as WORD32;
                        return 0x818 as IA_ERRORCODE;
                    }
                    (*p_obj_exhaacplus_dec).aac_config.ui_loudness_leveling_flag = *pui_value_signed
                        as WORD32;
                }
                _ => return IA_XHEAAC_DEC_API_FATAL_INVALID_CONFIG_PARAM as IA_ERRORCODE,
            }
        }
        IA_API_CMD_GET_CONFIG_PARAM => {
            let mut i: UWORD32 = 0;
            let mut pvalue: *mut WORD32 = &mut (*p_obj_exhaacplus_dec)
                .aac_config
                .ui_pcm_wdsz as *mut UWORD32 as *mut WORD32;
            if IA_XHEAAC_DEC_CONFIG_PARAM_NUM_CHANNELS == i_idx {
                if !((*p_obj_exhaacplus_dec).p_state_aac).is_null()
                    && (*(*p_obj_exhaacplus_dec).p_state_aac)
                        .heaac_mps_handle
                        .heaac_mps_present == 1 as core::ffi::c_int
                {
                    *pui_value = (*(*p_obj_exhaacplus_dec).p_state_aac)
                        .heaac_mps_handle
                        .num_output_channels_at as core::ffi::c_uint;
                } else {
                    *pui_value = *pvalue.offset(i_idx as isize) as core::ffi::c_uint;
                }
            } else if IA_XHEAAC_DEC_CONFIG_PARAM_CHANNEL_MASK == i_idx {
                if !((*p_obj_exhaacplus_dec).p_state_aac).is_null()
                    && (*(*p_obj_exhaacplus_dec).p_state_aac)
                        .heaac_mps_handle
                        .heaac_mps_present == 1 as core::ffi::c_int
                {
                    *pui_value = (*(*p_obj_exhaacplus_dec).p_state_aac)
                        .heaac_mps_handle
                        .bs_config
                        .ui_channel_mask as core::ffi::c_uint;
                } else {
                    *pui_value = *pvalue.offset(i_idx as isize) as core::ffi::c_uint;
                }
            } else if i_idx >= 0 as core::ffi::c_int && i_idx <= 8 as core::ffi::c_int {
                *pui_value = *pvalue.offset(i_idx as isize) as core::ffi::c_uint;
            } else if IA_ENHAACPLUS_DEC_CONFIG_GET_NUM_PRE_ROLL_FRAMES == i_idx {
                let mut ptri_value: *mut WORD32 = pv_value as *mut WORD32;
                let mut ptr_audio_specific_config: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ia_audio_specific_config as *mut ia_audio_specific_config_struct;
                i = 0 as UWORD32;
                while i < (MAX_AUDIO_PREROLLS + 1 as core::ffi::c_int) as UWORD32 {
                    if (*ptr_audio_specific_config)
                        .str_usac_config
                        .str_usac_dec_config
                        .preroll_bytes[i as usize] == 0 as UWORD32
                    {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                *ptri_value = i as WORD32;
            } else if IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_PTR == i_idx {
                let mut ptr_audio_specific_config_0: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ia_audio_specific_config as *mut ia_audio_specific_config_struct;
                i = 0 as UWORD32;
                while i
                    < (*ptr_audio_specific_config_0)
                        .str_usac_config
                        .str_usac_dec_config
                        .num_config_extensions
                {
                    let ref mut fresh8 = *pp_value.offset(i as isize);
                    *fresh8 = ((*ptr_audio_specific_config_0)
                        .str_usac_config
                        .str_usac_dec_config
                        .usac_cfg_ext_info_buf[i as usize])
                        .as_mut_ptr() as pVOID;
                    i = i.wrapping_add(1);
                }
                i = 0 as UWORD32;
                while i
                    < (*ptr_audio_specific_config_0)
                        .str_usac_config
                        .str_usac_dec_config
                        .num_elements
                {
                    if (*ptr_audio_specific_config_0)
                        .str_usac_config
                        .str_usac_dec_config
                        .usac_ext_ele_payload_present[i as usize] != 0
                    {
                        let ref mut fresh9 = *pp_value
                            .offset(i.wrapping_add(16 as UWORD32) as isize);
                        *fresh9 = ((*ptr_audio_specific_config_0)
                            .str_usac_config
                            .str_usac_dec_config
                            .usac_ext_ele_payload_buf[i as usize])
                            .as_mut_ptr() as pVOID;
                    }
                    i = i.wrapping_add(1);
                }
            } else if IA_ENHAACPLUS_DEC_CONFIG_EXT_ELE_BUF_SIZES == i_idx {
                let mut ptri_value_0: *mut WORD32 = pv_value as *mut WORD32;
                let mut ptr_audio_specific_config_1: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ia_audio_specific_config as *mut ia_audio_specific_config_struct;
                i = 0 as UWORD32;
                while i
                    < (*ptr_audio_specific_config_1)
                        .str_usac_config
                        .str_usac_dec_config
                        .num_config_extensions
                {
                    *ptri_value_0.offset(i as isize) = (*ptr_audio_specific_config_1)
                        .str_usac_config
                        .str_usac_dec_config
                        .usac_cfg_ext_info_len[i as usize];
                    i = i.wrapping_add(1);
                }
                i = 0 as UWORD32;
                while i
                    < (*ptr_audio_specific_config_1)
                        .str_usac_config
                        .str_usac_dec_config
                        .num_elements
                {
                    *ptri_value_0.offset(i.wrapping_add(16 as UWORD32) as isize) = (*ptr_audio_specific_config_1)
                        .str_usac_config
                        .str_usac_dec_config
                        .usac_ext_ele_payload_len[i as usize];
                    i = i.wrapping_add(1);
                }
            } else if IA_ENHAACPLUS_DEC_DRC_IS_CONFIG_CHANGED == i_idx {
                *pui_value = (*(*p_obj_exhaacplus_dec).p_state_aac).drc_config_changed
                    as core::ffi::c_uint;
            } else if IA_ENHAACPLUS_DEC_DRC_APPLY_CROSSFADE == i_idx {
                *pui_value = (*(*p_obj_exhaacplus_dec).p_state_aac).apply_crossfade
                    as core::ffi::c_uint;
            } else if IA_ENHAACPLUS_DEC_CONFIG_NUM_ELE == i_idx {
                let mut ptri_value_1: *mut UWORD32 = pv_value as *mut UWORD32;
                let mut ptr_audio_specific_config_2: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ia_audio_specific_config as *mut ia_audio_specific_config_struct;
                *ptri_value_1 = (*ptr_audio_specific_config_2)
                    .str_usac_config
                    .str_usac_dec_config
                    .num_elements;
            } else if IA_ENHAACPLUS_DEC_CONFIG_NUM_CONFIG_EXT == i_idx {
                let mut ptri_value_2: *mut UWORD32 = pv_value as *mut UWORD32;
                let mut ptr_audio_specific_config_3: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ia_audio_specific_config as *mut ia_audio_specific_config_struct;
                *ptri_value_2 = (*ptr_audio_specific_config_3)
                    .str_usac_config
                    .str_usac_dec_config
                    .num_config_extensions;
            } else if IA_ENHAACPLUS_DEC_CONFIG_GAIN_PAYLOAD_LEN == i_idx {
                let mut ptri_value_3: *mut UWORD32 = pv_value as *mut UWORD32;
                let mut ptr_audio_specific_config_4: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ia_audio_specific_config as *mut ia_audio_specific_config_struct;
                let mut preroll_counter: WORD32 = (*ptr_audio_specific_config_4)
                    .str_usac_config
                    .str_usac_dec_config
                    .preroll_counter;
                *ptri_value_3 = (*ptr_audio_specific_config_4)
                    .str_usac_config
                    .str_usac_dec_config
                    .usac_ext_gain_payload_len[preroll_counter as usize] as UWORD32;
            } else if IA_ENHAACPLUS_DEC_CONFIG_GAIN_PAYLOAD_BUF == i_idx {
                let mut payload_buffer_offeset: WORD32 = 0 as WORD32;
                let mut ptr_audio_specific_config_5: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ia_audio_specific_config as *mut ia_audio_specific_config_struct;
                let mut preroll_counter_0: WORD32 = (*ptr_audio_specific_config_5)
                    .str_usac_config
                    .str_usac_dec_config
                    .preroll_counter;
                i = 0 as UWORD32;
                while i < preroll_counter_0 as UWORD32 {
                    payload_buffer_offeset = (payload_buffer_offeset
                        as core::ffi::c_ulong)
                        .wrapping_add(
                            ((*ptr_audio_specific_config_5)
                                .str_usac_config
                                .str_usac_dec_config
                                .usac_ext_gain_payload_len[i as usize] as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD8>() as usize)
                                as core::ffi::c_ulong,
                        ) as WORD32 as WORD32;
                    i = i.wrapping_add(1);
                }
                *pp_value = ((*ptr_audio_specific_config_5)
                    .str_usac_config
                    .str_usac_dec_config
                    .usac_ext_gain_payload_buf)
                    .as_mut_ptr()
                    .offset(payload_buffer_offeset as isize) as pVOID;
            } else if IA_XHEAAC_DEC_CONFIG_PARAM_AOT == i_idx {
                if !((*p_obj_exhaacplus_dec).p_state_aac).is_null() {
                    if (*p_obj_exhaacplus_dec).aac_config.ui_mp4_flag == 1 as UWORD32 {
                        let mut ptr_audio_specific_config_6: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                            .p_state_aac)
                            .ia_audio_specific_config
                            as *mut ia_audio_specific_config_struct;
                        *pui_value = (*ptr_audio_specific_config_6).audio_object_type
                            as core::ffi::c_uint;
                    } else {
                        *pui_value = (*(*p_obj_exhaacplus_dec).p_state_aac)
                            .audio_object_type as core::ffi::c_uint;
                    }
                } else {
                    *pui_value = AOT_AAC_LC as core::ffi::c_int as core::ffi::c_uint;
                }
            } else if IA_XHEAAC_DEC_CONFIG_PARAM_DRC_LOUDNESS_LEVELING == i_idx {
                let mut ui_value: *mut WORD32 = &mut (*p_obj_exhaacplus_dec)
                    .aac_config
                    .ui_loudness_leveling_flag as *mut WORD32;
                *pui_value = *ui_value as core::ffi::c_uint;
            } else if IA_XHEAAC_DEC_CONFIG_PARAM_DRC_CUT == i_idx {
                let mut ui_value_0: *mut UWORD32 = &mut (*p_obj_exhaacplus_dec)
                    .aac_config
                    .ui_drc_cut as *mut WORD32 as *mut UWORD32;
                *pui_value = *ui_value_0 as core::ffi::c_uint;
            } else if IA_XHEAAC_DEC_CONFIG_PARAM_DRC_BOOST == i_idx {
                let mut ui_value_1: *mut UWORD32 = &mut (*p_obj_exhaacplus_dec)
                    .aac_config
                    .ui_drc_boost as *mut WORD32 as *mut UWORD32;
                *pui_value = *ui_value_1 as core::ffi::c_uint;
            } else if IA_XHEAAC_DEC_CONFIG_PARAM_DRC_MODE_CUT == i_idx {
                let mut ui_value_2: *mut UWORD8 = &mut (*p_obj_exhaacplus_dec)
                    .aac_config
                    .ui_drc_mode_cut;
                *pb_value = *ui_value_2 as core::ffi::c_schar;
            } else if IA_XHEAAC_DEC_CONFIG_PARAM_DRC_MODE_BOOST == i_idx {
                let mut ui_value_3: *mut UWORD8 = &mut (*p_obj_exhaacplus_dec)
                    .aac_config
                    .ui_drc_mode_boost;
                *pb_value = *ui_value_3 as core::ffi::c_schar;
            } else {
                return IA_XHEAAC_DEC_API_FATAL_INVALID_CONFIG_PARAM as IA_ERRORCODE
            }
        }
        IA_API_CMD_GET_MEMTABS_SIZE => {
            *pui_value = (::core::mem::size_of::<ia_mem_info_struct>() as usize)
                .wrapping_add(::core::mem::size_of::<*mut pVOID>() as usize)
                .wrapping_mul(4 as usize) as core::ffi::c_uint;
        }
        IA_API_CMD_GET_LOUDNESS_VAL => {
            *pui_value = (*p_obj_exhaacplus_dec).aac_config.output_level
                as core::ffi::c_uint;
        }
        IA_API_CMD_SET_MEMTABS_PTR => {
            if pv_value.is_null() {
                return IA_XHEAAC_DEC_API_FATAL_MEM_ALLOC as IA_ERRORCODE;
            }
            memset(
                pv_value as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (::core::mem::size_of::<ia_mem_info_struct>() as size_t)
                    .wrapping_add(::core::mem::size_of::<*mut pVOID>() as size_t)
                    .wrapping_mul(4 as size_t),
            );
            (*p_obj_exhaacplus_dec).p_mem_info_aac = pv_value as *mut ia_mem_info_struct;
            (*p_obj_exhaacplus_dec).pp_mem_aac = (pv_value as *mut WORD8)
                .offset(
                    (::core::mem::size_of::<ia_mem_info_struct>() as usize)
                        .wrapping_mul(IA_ENHAACPDEC_NUM_MEMTABS as usize) as isize,
                ) as *mut pVOID;
        }
        IA_API_CMD_GET_N_MEMTABS => {
            *pui_value = IA_ENHAACPDEC_NUM_MEMTABS as core::ffi::c_uint;
        }
        IA_API_CMD_GET_N_TABLES => {
            *pui_value = (NUM_AAC_TABLES + NUM_MPS_TABLES) as core::ffi::c_uint;
        }
        IA_API_CMD_EXECUTE => {
            match i_idx {
                IA_CMD_TYPE_DO_EXECUTE => {
                    let mut err_code_0: WORD32 = 0 as WORD32;
                    if (*(*p_obj_exhaacplus_dec).p_state_aac).ui_init_done == 0
                        || (*(*p_obj_exhaacplus_dec).p_state_aac).fatal_err_present != 0
                    {
                        err_code_0 = IA_FATAL_ERROR as WORD32;
                    } else {
                        memset(
                            ((*(*p_obj_exhaacplus_dec).p_state_aac).qshift_adj)
                                .as_mut_ptr() as *mut core::ffi::c_void,
                            0 as core::ffi::c_int,
                            ::core::mem::size_of::<[WORD8; 16]>() as size_t,
                        );
                        (*(*p_obj_exhaacplus_dec).p_state_aac).qshift_cnt = 0 as UWORD8;
                        err_code_0 = ixheaacd_dec_execute(p_obj_exhaacplus_dec);
                    }
                    if err_code_0 != IA_NO_ERROR {
                        if err_code_0 < 0 as core::ffi::c_int {
                            (*(*p_obj_exhaacplus_dec).p_state_aac).fatal_err_present = 1
                                as core::ffi::c_int as WORD32;
                        }
                        (*(*p_obj_exhaacplus_dec).p_state_aac).i_bytes_consumed = (*(*p_obj_exhaacplus_dec)
                            .p_state_aac)
                            .ui_in_bytes as WORD32;
                    }
                    return err_code_0 as IA_ERRORCODE;
                }
                IA_CMD_TYPE_DONE_QUERY => {
                    if (*(*p_obj_exhaacplus_dec).p_state_aac).ui_input_over
                        == 1 as UWORD32
                    {
                        *pui_value = 1 as core::ffi::c_uint;
                    } else {
                        *pui_value = 0 as core::ffi::c_uint;
                    }
                }
                _ => return IA_XHEAAC_DEC_API_FATAL_INVALID_EXECUTE_TYPE as IA_ERRORCODE,
            }
        }
        IA_API_CMD_GET_CURIDX_INPUT_BUF => {
            *pui_value = (*(*p_obj_exhaacplus_dec).p_state_aac).i_bytes_consumed
                as core::ffi::c_uint;
        }
        IA_API_CMD_SET_INPUT_BYTES => {
            (*(*p_obj_exhaacplus_dec).p_state_aac).ui_in_bytes = *pui_value as UWORD32;
        }
        IA_API_CMD_GET_OUTPUT_BYTES => {
            if 1 as core::ffi::c_int == i_idx {
                *pui_value = (*(*p_obj_exhaacplus_dec).p_state_aac).ui_mps_out_bytes
                    as core::ffi::c_uint;
            } else {
                *pui_value = (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes
                    as core::ffi::c_uint;
            }
            if (*(*p_obj_exhaacplus_dec).p_state_aac).audio_object_type
                as core::ffi::c_uint == AOT_USAC as core::ffi::c_int as core::ffi::c_uint
            {
                let mut ptr_audio_specific_config_7: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ia_audio_specific_config as *mut ia_audio_specific_config_struct;
                let mut preroll_counter_1: WORD32 = (*ptr_audio_specific_config_7)
                    .str_usac_config
                    .str_usac_dec_config
                    .preroll_counter;
                *pui_value = (*ptr_audio_specific_config_7)
                    .str_usac_config
                    .str_usac_dec_config
                    .preroll_bytes[preroll_counter_1 as usize] as core::ffi::c_uint;
                preroll_counter_1 += 1;
                if preroll_counter_1 > MAX_AUDIO_PREROLLS + 1 as core::ffi::c_int {
                    return IA_FATAL_ERROR as IA_ERRORCODE;
                }
                (*ptr_audio_specific_config_7)
                    .str_usac_config
                    .str_usac_dec_config
                    .preroll_counter = preroll_counter_1;
            }
        }
        IA_API_CMD_INPUT_OVER => {
            (*(*p_obj_exhaacplus_dec).p_state_aac).ui_input_over = 1 as UWORD32;
        }
        _ => return IA_XHEAAC_DEC_API_FATAL_INVALID_CMD as IA_ERRORCODE,
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decoder_2_ga_hdr(
    mut p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
) -> WORD32 {
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    if (*p_obj_exhaacplus_dec).aac_config.ui_flush_cmd == 0 as core::ffi::c_int {
        (*p_obj_exhaacplus_dec).aac_config.ui_pcm_wdsz = 16 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.flag_downmix = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.flag_08khz_out = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.flag_16khz_out = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.down_sample_flag = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.header_dec_done = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.frame_status = 1 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_mp4_flag = 1 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_disable_sync = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_auto_sbr_upsample = 1 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_samp_freq = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_channel_mode = 3 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_sbr_mode = 2 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_pce_found_in_hdr = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.loas_present = 0 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_boost = 0 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_cut = 0 as core::ffi::c_int as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_mode_cut = 0 as UWORD8;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_mode_boost = 0 as UWORD8;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_target_level = 108 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_set = 0 as core::ffi::c_int as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_flush_cmd = 1 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_max_channels = 6 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_coupling_channel = 0 as core::ffi::c_int
            as WORD;
        (*p_obj_exhaacplus_dec).aac_config.downmix = 0 as core::ffi::c_int as WORD;
        let mut pstr_aac_tables: *mut ia_aac_dec_tables_struct = &mut (*p_obj_exhaacplus_dec)
            .aac_tables;
        (*pstr_aac_tables).pstr_huffmann_tables = &ixheaacd_aac_huffmann_tables
            as *const ia_aac_dec_huffman_tables_struct
            as *mut ia_aac_dec_huffman_tables_struct;
        (*pstr_aac_tables).pstr_block_tables = &ixheaacd_aac_block_tables
            as *const ia_aac_dec_block_tables_struct
            as *mut ia_aac_dec_block_tables_struct;
        (*pstr_aac_tables).pstr_imdct_tables = &ixheaacd_imdct_tables
            as *const ia_aac_dec_imdct_tables_struct
            as *mut ia_aac_dec_imdct_tables_struct;
        ixheaacd_huff_tables_create(pstr_aac_tables);
        ixheaacd_init_sbr_tables(&mut (*p_obj_exhaacplus_dec).str_sbr_tables);
        (*p_obj_exhaacplus_dec).common_tables = &ixheaacd_str_fft_n_transcendent_tables
            as *const ixheaacd_misc_tables as *mut ixheaacd_misc_tables;
        (*p_obj_exhaacplus_dec).aac_config.ui_qmf_bands = 64 as UWORD32;
        (*(*p_obj_exhaacplus_dec).p_state_aac).ui_init_done = 0 as UWORD32;
        err_code = ixheaacd_dec_init(p_obj_exhaacplus_dec) as IA_ERRORCODE;
    } else {
        (*p_obj_exhaacplus_dec).aac_config.ui_flush_cmd = 0 as core::ffi::c_int
            as WORD32;
        err_code = ixheaacd_dec_init(p_obj_exhaacplus_dec) as IA_ERRORCODE;
    }
    if err_code != 0
        && (*(*p_obj_exhaacplus_dec).p_state_aac).s_adts_hdr_present as core::ffi::c_int
            != 0
    {
        (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done = 0 as UWORD32;
    }
    if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 && err_code != 0 {
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return IA_XHEAAC_DEC_INIT_FATAL_EC_INIT_FAIL as WORD32
        } else {
            return IA_XHEAAC_DEC_INIT_NONFATAL_EC_INIT_FAIL
        }
    } else {
        return err_code as WORD32
    };
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_decoder_flush_api(
    mut p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
) -> WORD32 {
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut header_temp_ptr: *mut UWORD8 = 0 as *mut UWORD8;
    let mut header_length: WORD32 = 0;
    if (*p_obj_exhaacplus_dec).aac_config.ui_flush_cmd == 0 as core::ffi::c_int {
        header_temp_ptr = (*(*p_obj_exhaacplus_dec).p_state_aac).header_ptr;
        header_length = (*(*p_obj_exhaacplus_dec).p_state_aac).header_length;
        memset(
            (*p_obj_exhaacplus_dec).p_state_aac as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<ia_aac_dec_state_struct>() as size_t)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
        );
        let mut p_temp: pUWORD8 = (*p_obj_exhaacplus_dec).p_state_aac as pUWORD8;
        let mut meminfo: *mut UWORD32 = (*p_obj_exhaacplus_dec).p_mem_info_aac
            as *mut UWORD32;
        let mut pers_size: UWORD32 = *meminfo.offset(0 as core::ffi::c_int as isize);
        p_temp = p_temp
            .offset(pers_size as isize)
            .offset(
                -(((::core::mem::size_of::<ia_dec_data_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (::core::mem::size_of::<ia_audio_specific_config_struct>()
                            as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (8300 as core::ffi::c_int
                            + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as usize,
                    ) as isize),
            );
        (*(*p_obj_exhaacplus_dec).p_state_aac).pstr_dec_data = p_temp
            as *mut core::ffi::c_void;
        (*(*p_obj_exhaacplus_dec).p_state_aac).ia_audio_specific_config = p_temp
            .offset(
                ((::core::mem::size_of::<ia_dec_data_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as isize,
            ) as *mut core::ffi::c_void;
        (*(*p_obj_exhaacplus_dec).p_state_aac).header_ptr = p_temp
            .offset(
                ((::core::mem::size_of::<ia_dec_data_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as isize,
            )
            .offset(
                ((::core::mem::size_of::<ia_audio_specific_config_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as isize,
            ) as *mut UWORD8;
        memset(
            &mut (*p_obj_exhaacplus_dec).aac_config as *mut ia_aac_dec_config_struct
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<ia_aac_dec_config_struct>() as size_t,
        );
        (*p_obj_exhaacplus_dec).aac_config.ui_pcm_wdsz = 16 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.flag_downmix = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.flag_08khz_out = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.flag_16khz_out = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.down_sample_flag = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.header_dec_done = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.frame_status = 1 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_mp4_flag = 1 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_disable_sync = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_auto_sbr_upsample = 1 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_samp_freq = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_channel_mode = 3 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_sbr_mode = 2 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_pce_found_in_hdr = 0 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.loas_present = 0 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_boost = 0 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_cut = 0 as core::ffi::c_int as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_mode_cut = 0 as UWORD8;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_mode_boost = 0 as UWORD8;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_target_level = 108 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_drc_set = 0 as core::ffi::c_int as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_flush_cmd = 1 as core::ffi::c_int
            as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_max_channels = 6 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_coupling_channel = 0 as core::ffi::c_int
            as WORD;
        (*p_obj_exhaacplus_dec).aac_config.downmix = 0 as core::ffi::c_int as WORD;
        (*(*p_obj_exhaacplus_dec).p_state_aac).peak_lim_init = 0 as UWORD8;
        let mut pstr_aac_tables: *mut ia_aac_dec_tables_struct = &mut (*p_obj_exhaacplus_dec)
            .aac_tables;
        (*pstr_aac_tables).pstr_huffmann_tables = &ixheaacd_aac_huffmann_tables
            as *const ia_aac_dec_huffman_tables_struct
            as *mut ia_aac_dec_huffman_tables_struct;
        (*pstr_aac_tables).pstr_block_tables = &ixheaacd_aac_block_tables
            as *const ia_aac_dec_block_tables_struct
            as *mut ia_aac_dec_block_tables_struct;
        (*pstr_aac_tables).pstr_imdct_tables = &ixheaacd_imdct_tables
            as *const ia_aac_dec_imdct_tables_struct
            as *mut ia_aac_dec_imdct_tables_struct;
        ixheaacd_huff_tables_create(pstr_aac_tables);
        ixheaacd_init_sbr_tables(&mut (*p_obj_exhaacplus_dec).str_sbr_tables);
        (*p_obj_exhaacplus_dec).common_tables = &ixheaacd_str_fft_n_transcendent_tables
            as *const ixheaacd_misc_tables as *mut ixheaacd_misc_tables;
        (*p_obj_exhaacplus_dec).aac_config.ui_qmf_bands = 64 as UWORD32;
        (*(*p_obj_exhaacplus_dec).p_state_aac).header_ptr = header_temp_ptr;
        (*(*p_obj_exhaacplus_dec).p_state_aac).ui_in_bytes = header_length as UWORD32;
        (*(*p_obj_exhaacplus_dec).p_state_aac).header_length = header_length;
        err_code = ixheaacd_dec_init(p_obj_exhaacplus_dec) as IA_ERRORCODE;
    } else {
        (*p_obj_exhaacplus_dec).aac_config.ui_flush_cmd = 0 as core::ffi::c_int
            as WORD32;
        err_code = ixheaacd_dec_init(p_obj_exhaacplus_dec) as IA_ERRORCODE;
    }
    if err_code != 0
        && (*(*p_obj_exhaacplus_dec).p_state_aac).s_adts_hdr_present as core::ffi::c_int
            != 0
    {
        (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done = 0 as UWORD32;
    }
    if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 && err_code != 0 {
        if err_code as core::ffi::c_uint & IA_FATAL_ERROR != 0 {
            return IA_XHEAAC_DEC_INIT_FATAL_EC_INIT_FAIL as WORD32
        } else {
            return IA_XHEAAC_DEC_INIT_NONFATAL_EC_INIT_FAIL
        }
    } else {
        return err_code as WORD32
    };
}
#[inline]
unsafe extern "C" fn ixheaacd_persistent_buffer_sizes(
    mut num_channel: WORD32,
) -> WORD32 {
    let mut size_buffers: WORD32 = 0 as WORD32;
    let mut temp: WORD32 = 0;
    let mut max_channels: WORD32 = 0;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            (((4 as WORD32 * 512 as WORD32 * num_channel) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            (num_channel as usize)
                .wrapping_mul(
                    (ltp_buffer_size as core::ffi::c_int as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            (num_channel as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_aac_dec_channel_info>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    if num_channel > 2 as core::ffi::c_int {
        max_channels = MAX_BS_ELEMENT as WORD32;
    } else {
        max_channels = 2 as core::ffi::c_int as WORD32;
    }
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((max_channels as core::ffi::c_int * 2 as core::ffi::c_int) as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_aac_dec_sbr_bitstream_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            (num_channel as usize)
                .wrapping_mul(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD8>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            (num_channel as usize)
                .wrapping_mul(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD8>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as WORD32 * num_channel) as usize)
                .wrapping_mul(
                    ((320 as core::ffi::c_int
                        + 2 as core::ffi::c_int
                            * (64 as core::ffi::c_int / 2 as core::ffi::c_int)) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as WORD32 * num_channel) as usize)
                .wrapping_mul(
                    ((320 as core::ffi::c_int
                        + 2 as core::ffi::c_int
                            * (64 as core::ffi::c_int / 2 as core::ffi::c_int)) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as WORD32 * num_channel) as usize)
                .wrapping_mul(
                    ((1280 as core::ffi::c_int
                        + 2 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as WORD32 * num_channel) as usize)
                .wrapping_mul(
                    ((1280 as core::ffi::c_int
                        + 2 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((num_channel as core::ffi::c_int * 2 as core::ffi::c_int) as usize)
                .wrapping_mul(
                    ((6 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as WORD32 * LPC_ORDER * num_channel) as usize)
                .wrapping_mul(
                    ((64 as core::ffi::c_int / 2 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(
                    (2 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as WORD32 * LPC_ORDER * num_channel) as usize)
                .wrapping_mul(
                    ((64 as core::ffi::c_int / 2 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as core::ffi::c_int * num_channel as core::ffi::c_int
                * 3 as core::ffi::c_int) as usize)
                .wrapping_mul(
                    (56 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    temp = ((::core::mem::size_of::<ia_freq_band_data_struct>() as usize)
        .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        .wrapping_add(
            (::core::mem::size_of::<ia_sbr_prev_frame_data_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        )
        .wrapping_add(
            (::core::mem::size_of::<ia_sbr_channel_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        )
        .wrapping_add(
            (::core::mem::size_of::<ia_sbr_header_data_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        ) as WORD32;
    size_buffers += (2 as WORD32 * num_channel * temp) as core::ffi::c_int;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            (MAX_BS_ELEMENT as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<*mut ixheaac_drc_bs_data_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            (num_channel as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_ps_dec_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    let mut temp_size: WORD32 = 0 as WORD32;
    size_buffers = (size_buffers as core::ffi::c_ulong)
        .wrapping_add(
            ((num_channel as core::ffi::c_int * 2 as core::ffi::c_int) as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_sbr_frame_info_data_struct>() as usize)
                        .wrapping_add(
                            (56 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_mul(2 as usize),
                        )
                        .wrapping_add(8 as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    temp_size = (temp_size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_pvc_data_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    temp_size = (temp_size as core::ffi::c_ulong)
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_esbr_hbe_txposer_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    temp_size = (temp_size as core::ffi::c_ulong)
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(
                    (64 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                        .wrapping_add(
                            ((2 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<*mut FLOAT32>() as usize,
                                ),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 2 as core::ffi::c_int
                                * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((64 as core::ffi::c_int * 2 as core::ffi::c_int
                                * 64 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            ((1024 as core::ffi::c_int + 64 as core::ffi::c_int)
                                as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            (((1024 as core::ffi::c_int + 64 as core::ffi::c_int)
                                * 2 as core::ffi::c_int) as usize)
                                .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize),
                        )
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    temp_size = (temp_size as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as core::ffi::c_int * 2 as core::ffi::c_int) as usize)
                .wrapping_mul(
                    (78 as usize)
                        .wrapping_mul(::core::mem::size_of::<*mut FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    temp_size = (temp_size as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as core::ffi::c_int * 2 as core::ffi::c_int * MAX_QMF_BUF_LEN) as usize)
                .wrapping_mul(
                    (78 as usize)
                        .wrapping_mul(::core::mem::size_of::<FLOAT32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    temp_size = (temp_size as core::ffi::c_ulong)
        .wrapping_add(
            ((2 as core::ffi::c_int * 2 as core::ffi::c_int * MAX_ENV_COLS) as usize)
                .wrapping_mul(
                    (78 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ) as core::ffi::c_ulong,
        ) as WORD32 as WORD32;
    size_buffers += num_channel * temp_size;
    return size_buffers;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fill_aac_mem_tables(
    mut p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
) -> VOID {
    let mut p_mem_info_aac: *mut ia_mem_info_struct = 0 as *mut ia_mem_info_struct;
    let mut num_channels: WORD32 = 0;
    let mut channels: WORD32 = 0;
    let mut buffer_size: WORD32 = 0;
    if (*p_obj_exhaacplus_dec).aac_config.ui_max_channels > 2 as UWORD32 {
        num_channels = ((*p_obj_exhaacplus_dec).aac_config.ui_max_channels)
            .wrapping_add(1 as UWORD32) as WORD32;
    } else {
        num_channels = (*p_obj_exhaacplus_dec).aac_config.ui_max_channels as WORD32;
    }
    channels = num_channels;
    buffer_size = ixheaacd_persistent_buffer_sizes(num_channels);
    p_mem_info_aac = &mut *((*p_obj_exhaacplus_dec).p_mem_info_aac)
        .offset(IA_ENHAACPLUS_DEC_PERSIST_IDX as isize) as *mut ia_mem_info_struct;
    (*p_mem_info_aac).ui_size = ((::core::mem::size_of::<ia_aac_dec_state_struct>()
        as usize)
        .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        .wrapping_add(
            (channels as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_aac_persistent_struct>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ),
        )
        .wrapping_add(buffer_size as usize)
        .wrapping_add((channels * ixheaacd_getsize_sbr_persistent()) as usize)
        .wrapping_add((channels as core::ffi::c_int * 16 as core::ffi::c_int) as usize)
        .wrapping_add(ixheaacd_mps_persistent_buffer_sizes() as usize) as UWORD32;
    (*p_mem_info_aac).ui_size = ((*p_mem_info_aac).ui_size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_dec_data_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as UWORD32 as UWORD32;
    (*p_mem_info_aac).ui_size = ((*p_mem_info_aac).ui_size as core::ffi::c_ulong)
        .wrapping_add(
            ((::core::mem::size_of::<ia_audio_specific_config_struct>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                as core::ffi::c_ulong,
        ) as UWORD32 as UWORD32;
    (*p_mem_info_aac).ui_size = ((*p_mem_info_aac).ui_size)
        .wrapping_add(
            (8300 as core::ffi::c_int + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as UWORD32,
        );
    (*p_mem_info_aac).ui_alignment = 16 as UWORD32;
    (*p_mem_info_aac).ui_type = IA_MEMTYPE_PERSIST as UWORD32;
    p_mem_info_aac = &mut *((*p_obj_exhaacplus_dec).p_mem_info_aac)
        .offset(IA_ENHAACPLUS_DEC_SCRATCH_IDX as isize) as *mut ia_mem_info_struct;
    (*p_mem_info_aac).ui_size = (if (if (if (((2 as core::ffi::c_int
        * 2 as core::ffi::c_int * 1024 as core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
        .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>() as usize)
                        .wrapping_add(
                            (::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                        )
                        & !(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_sub(1 as usize),
                )
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                        .wrapping_add(
                            (::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                        )
                        & !(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_sub(1 as usize),
                ),
        )
        .wrapping_add(
            (1024 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        )
        .wrapping_add(
            (128 as usize)
                .wrapping_mul(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ),
        )
        .wrapping_add(
            (128 as usize)
                .wrapping_mul(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ),
        ) > 194048 as usize
    {
        (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
            as usize)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                            as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    ),
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
    } else {
        194048 as usize
    })
        > (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
            * 1024 as core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                            as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    ),
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            ) > 194048 as usize
        {
            (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
                as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
        } else {
            194048 as usize
        })
    {
        (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
            as usize)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                            as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    ),
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            ) > 194048 as usize
        {
            (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
                as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
        } else {
            194048 as usize
        })
    } else {
        (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
            as usize)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                            as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    ),
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            ) > 194048 as usize
        {
            (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
                as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
        } else {
            194048 as usize
        })
    })
        > (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
            * 1024 as core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                            as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    ),
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            ) > 194048 as usize
        {
            (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
                as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
        } else {
            194048 as usize
        })
    {
        if (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
            * 1024 as core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                            as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    ),
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            ) > 194048 as usize
        {
            (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
                as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
        } else {
            194048 as usize
        })
            > (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                * 1024 as core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                ) > 194048 as usize
            {
                (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
            } else {
                194048 as usize
            })
        {
            if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                * 1024 as core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                ) > 194048 as usize
            {
                (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
            } else {
                194048 as usize
            }
        } else if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
            * 1024 as core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                            as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    ),
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            ) > 194048 as usize
        {
            (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
                as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
        } else {
            194048 as usize
        }
    } else if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
        * 1024 as core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
        .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(
                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>() as usize)
                        .wrapping_add(
                            (::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                        )
                        & !(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_sub(1 as usize),
                )
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                        .wrapping_add(
                            (::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                        )
                        & !(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_sub(1 as usize),
                ),
        )
        .wrapping_add(
            (1024 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        )
        .wrapping_add(
            (1024 as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
        )
        .wrapping_add(
            (2 as usize)
                .wrapping_mul(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ),
        )
        .wrapping_add(
            (128 as usize)
                .wrapping_mul(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ),
        )
        .wrapping_add(
            (128 as usize)
                .wrapping_mul(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                ),
        ) > 194048 as usize
    {
        (((2 as core::ffi::c_int * 2 as core::ffi::c_int * 1024 as core::ffi::c_int)
            as usize)
            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
            .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                            as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<ia_pns_stereo_data_struct>() as usize)
                            .wrapping_add(
                                (::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                            )
                            & !(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_sub(1 as usize),
                    ),
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (1024 as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
            )
            .wrapping_add(
                (2 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
            .wrapping_add(
                (128 as usize)
                    .wrapping_mul(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ),
            )
    } else {
        194048 as usize
    }) as UWORD32;
    (*p_mem_info_aac).ui_alignment = 8 as UWORD32;
    (*p_mem_info_aac).ui_type = IA_MEMTYPE_SCRATCH as UWORD32;
    p_mem_info_aac = &mut *((*p_obj_exhaacplus_dec).p_mem_info_aac)
        .offset(IA_ENHAACPLUS_DEC_INPUT_IDX as isize) as *mut ia_mem_info_struct;
    (*p_mem_info_aac).ui_size = IA_MAX_INP_BUFFER_SIZE as UWORD32;
    (*p_mem_info_aac).ui_alignment = 8 as UWORD32;
    (*p_mem_info_aac).ui_type = IA_MEMTYPE_INPUT as UWORD32;
    p_mem_info_aac = &mut *((*p_obj_exhaacplus_dec).p_mem_info_aac)
        .offset(IA_ENHAACPLUS_DEC_OUTPUT_IDX as isize) as *mut ia_mem_info_struct;
    (*p_mem_info_aac).ui_size = IA_ENHAACPLUS_DEC_OUT_BUF_SIZE as UWORD32;
    (*p_mem_info_aac).ui_alignment = 8 as UWORD32;
    (*p_mem_info_aac).ui_type = IA_MEMTYPE_OUTPUT as UWORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_table_api(
    mut p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
    mut i_cmd: WORD32,
    mut i_idx: WORD32,
    mut pv_value: pVOID,
) -> WORD32 {
    let mut pstr_mps_state: *mut ia_heaac_mps_state_struct = &mut (*(*p_obj_exhaacplus_dec)
        .p_state_aac)
        .heaac_mps_handle;
    let mut pui_value: pUWORD32 = pv_value as pUWORD32;
    let mut p_pui_value: *mut pUWORD32 = pv_value as *mut pUWORD32;
    let mut ui_get_vals: [size_t; 5] = [0; 5];
    let mut table_ptrs: [*mut pVOID; 21] = [0 as *mut pVOID; 21];
    let mut table_sizes: [UWORD32; 21] = [
        ::core::mem::size_of::<ia_aac_dec_huffman_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_aac_dec_block_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_aac_dec_imdct_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ixheaacd_misc_tables>() as UWORD32,
        ::core::mem::size_of::<ia_env_calc_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_qmf_dec_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_env_extr_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_ps_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_qmf_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_common_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_hybrid_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_m1_m2_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_decorr_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_tp_process_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_mdct2qmf_table_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_tonality_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_bitdec_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_blind_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_mdct2qmf_tables_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_mdct2qmf_cos_table_struct>() as UWORD32,
        ::core::mem::size_of::<ia_mps_dec_residual_aac_tables_struct>() as UWORD32,
    ];
    table_ptrs[0 as core::ffi::c_int as usize] = &mut (*p_obj_exhaacplus_dec)
        .aac_tables
        .pstr_huffmann_tables as *mut *mut ia_aac_dec_huffman_tables_struct
        as *mut pVOID;
    table_ptrs[1 as core::ffi::c_int as usize] = &mut (*p_obj_exhaacplus_dec)
        .aac_tables
        .pstr_block_tables as *mut *mut ia_aac_dec_block_tables_struct as *mut pVOID;
    table_ptrs[2 as core::ffi::c_int as usize] = &mut (*p_obj_exhaacplus_dec)
        .aac_tables
        .pstr_imdct_tables as *mut *mut ia_aac_dec_imdct_tables_struct as *mut pVOID;
    table_ptrs[3 as core::ffi::c_int as usize] = &mut (*p_obj_exhaacplus_dec)
        .common_tables as *mut *mut ixheaacd_misc_tables as *mut pVOID;
    table_ptrs[4 as core::ffi::c_int as usize] = &mut (*p_obj_exhaacplus_dec)
        .str_sbr_tables
        .env_calc_tables_ptr as *mut *mut ia_env_calc_tables_struct as *mut pVOID;
    table_ptrs[5 as core::ffi::c_int as usize] = &mut (*p_obj_exhaacplus_dec)
        .str_sbr_tables
        .qmf_dec_tables_ptr as *mut *mut ia_qmf_dec_tables_struct as *mut pVOID;
    table_ptrs[6 as core::ffi::c_int as usize] = &mut (*p_obj_exhaacplus_dec)
        .str_sbr_tables
        .env_extr_tables_ptr as *mut *mut ia_env_extr_tables_struct as *mut pVOID;
    table_ptrs[7 as core::ffi::c_int as usize] = &mut (*p_obj_exhaacplus_dec)
        .str_sbr_tables
        .ps_tables_ptr as *mut *mut ia_ps_tables_struct as *mut pVOID;
    table_ptrs[8 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .qmf_table_ptr as *mut *mut ia_mps_dec_qmf_tables_struct as *mut pVOID;
    table_ptrs[9 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .common_table_ptr as *mut *mut ia_mps_dec_common_tables_struct as *mut pVOID;
    table_ptrs[10 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .hybrid_table_ptr as *mut *mut ia_mps_dec_hybrid_tables_struct as *mut pVOID;
    table_ptrs[11 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .m1_m2_table_ptr as *mut *mut ia_mps_dec_m1_m2_tables_struct as *mut pVOID;
    table_ptrs[12 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .decor_table_ptr as *mut *mut ia_mps_dec_decorr_tables_struct as *mut pVOID;
    table_ptrs[13 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .tp_process_table_ptr as *mut *mut ia_mps_dec_tp_process_tables_struct
        as *mut pVOID;
    table_ptrs[14 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .mdct2qmf_table_ptr as *mut *mut ia_mps_dec_mdct2qmf_table_struct as *mut pVOID;
    table_ptrs[15 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .tonality_table_ptr as *mut *mut ia_mps_dec_tonality_tables_struct as *mut pVOID;
    table_ptrs[16 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .bitdec_table_ptr as *mut *mut ia_mps_dec_bitdec_tables_struct as *mut pVOID;
    table_ptrs[17 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .blind_table_ptr as *mut *mut ia_mps_dec_blind_tables_struct as *mut pVOID;
    table_ptrs[18 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .mdct2qmfcos_table_ptr as *mut *mut ia_mps_dec_mdct2qmf_tables_struct
        as *mut pVOID;
    table_ptrs[19 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .mdct2qmfcos_tab_ptr as *mut *mut ia_mps_dec_mdct2qmf_cos_table_struct
        as *mut pVOID;
    table_ptrs[20 as core::ffi::c_int as usize] = &mut (*pstr_mps_state)
        .ia_mps_dec_mps_table
        .aac_tab as *mut *mut core::ffi::c_void as *mut pVOID;
    if i_idx < 0 as core::ffi::c_int || i_idx >= NUM_AAC_TABLES + NUM_MPS_TABLES {
        return IA_XHEAAC_DEC_API_FATAL_INVALID_MEMTAB_INDEX as WORD32;
    }
    ui_get_vals[0 as core::ffi::c_int as usize] = table_sizes[i_idx as usize] as size_t;
    ui_get_vals[1 as core::ffi::c_int as usize] = 4 as size_t;
    ui_get_vals[4 as core::ffi::c_int as usize] = *table_ptrs[i_idx as usize] as size_t;
    if i_cmd == IA_API_CMD_SET_TABLE_PTR {
        if pv_value.is_null() {
            return 0xffff8002 as WORD32;
        }
        if pv_value as size_t & 3 as size_t != 0 {
            return IA_XHEAAC_DEC_API_FATAL_MEM_ALIGN as WORD32;
        }
        *table_ptrs[i_idx as usize] = pv_value;
        if i_idx == 0 as core::ffi::c_int {
            ixheaacd_huff_tables_create(&mut (*p_obj_exhaacplus_dec).aac_tables);
        }
    } else if i_cmd == IA_API_CMD_GET_TABLE_PTR {
        *p_pui_value = ui_get_vals[(i_cmd as core::ffi::c_int
            - IA_API_CMD_GET_TABLE_INFO_SIZE) as usize] as *mut UWORD32 as pUWORD32;
    } else {
        *pui_value = ui_get_vals[(i_cmd as core::ffi::c_int
            - IA_API_CMD_GET_TABLE_INFO_SIZE) as usize] as WORD32 as core::ffi::c_uint;
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_mps_payload(
    mut self_0: ia_handle_sbr_dec_inst_struct,
    mut p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
) -> VOID {
    let mut local_bit_buff: ia_bit_buf_struct = ia_bit_buf_struct {
        ptr_bit_buf_base: 0 as *mut UWORD8,
        ptr_bit_buf_end: 0 as *mut UWORD8,
        ptr_read_next: 0 as *mut UWORD8,
        bit_pos: 0,
        cnt_bits: 0,
        size: 0,
        adts_header_present: 0,
        crc_check: 0,
        protection_absent: 0,
        no_raw_data_blocks: 0,
        str_adts_crc_info: ia_adts_crc_info_struct {
            crc_active: 0,
            no_reg: 0,
            file_value: 0,
            crc_lookup: [0; 256],
            str_crc_reg_data: [ia_crc_reg_data_struct {
                active: 0,
                buf_size: 0,
                max_bits: 0,
                bit_cnt: 0,
                bit_buf_cnt: 0,
                str_bit_buf: ia_crc_bit_buf_struct {
                    ptr_bit_buf_base: 0 as *mut UWORD8,
                    ptr_bit_buf_end: 0 as *mut UWORD8,
                    ptr_read_next: 0 as *mut UWORD8,
                    bit_pos: 0,
                    cnt_bits: 0,
                    size: 0,
                },
            }; 7],
        },
        pstr_adts_crc_info: 0 as *mut ia_adts_crc_info_struct,
        initial_cnt_bits: 0,
        audio_mux_align: 0,
        bit_count: 0,
        valid_bits: 0,
        byte: 0,
        byte_ptr: 0 as *mut UWORD8,
        ptr_start: 0 as *mut UWORD8,
        write_bit_count: 0,
        max_size: 0,
        xaac_jmp_buf: 0 as *mut jmp_buf,
    };
    let mut it_bit_buff: *mut ia_bit_buf_struct = 0 as *mut ia_bit_buf_struct;
    if !((*self_0).ptr_mps_data).is_null() {
        ixheaacd_create_init_bit_buf(
            &mut local_bit_buff,
            (*self_0).ptr_mps_data as *mut UWORD8,
            ((*self_0).left_mps_bits >> 3 as core::ffi::c_int) + 1 as WORD32,
        );
    }
    local_bit_buff.xaac_jmp_buf = &mut (*(*p_obj_exhaacplus_dec).p_state_aac)
        .xaac_jmp_buf;
    it_bit_buff = &mut local_bit_buff;
    (*it_bit_buff).bit_pos = (*self_0).mps_bits_pos;
    (*it_bit_buff).cnt_bits = (*self_0).left_mps_bits;
    while (*self_0).left_mps_bits >= 8 as core::ffi::c_int {
        ixheaacd_extension_payload(
            it_bit_buff as *mut ia_bit_buf_struct,
            &mut (*self_0).left_mps_bits,
            &mut (*(*p_obj_exhaacplus_dec).p_state_aac).mps_dec_handle,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_init(
    mut p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
) -> WORD32 {
    let mut frame_status: FLAG = 1 as FLAG;
    let mut frame_size_1: WORD32 = 0;
    let mut sample_rate_1: WORD32 = 0;
    let mut num_channels_1: WORD16 = 0;
    let mut ps_detected: WORD32 = 0 as WORD32;
    let mut in_buffer: *mut UWORD8 = 0 as *mut UWORD8;
    let mut time_data: *mut WORD16 = 0 as *mut WORD16;
    let mut ch_idx: WORD = 0;
    let mut sbr_present_flag: WORD = 0 as WORD;
    let mut mps_buffer: *mut UWORD8 = 0 as *mut UWORD8;
    let mut p_state_enhaacplus_dec: *mut ia_aac_dec_state_struct = 0
        as *mut ia_aac_dec_state_struct;
    let mut error_code: WORD32 = IA_NO_ERROR;
    let mut persistent_used: WORD32 = 0 as WORD32;
    let mut err_code: IA_ERRORCODE = IA_NO_ERROR;
    let mut aac_persistent_mem: *mut ia_aac_persistent_struct = 0
        as *mut ia_aac_persistent_struct;
    let mut sbr_persistent_mem: *mut ia_sbr_pers_struct = 0 as *mut ia_sbr_pers_struct;
    let mut ret_val: WORD32 = 0;
    (*p_obj_exhaacplus_dec).p_state_aac = *((*p_obj_exhaacplus_dec).pp_mem_aac)
        .offset(IA_ENHAACPLUS_DEC_PERSIST_IDX as isize) as *mut ia_aac_dec_state_struct;
    if (*(*p_obj_exhaacplus_dec).p_state_aac).ui_init_done != 0 {
        return IA_NO_ERROR;
    }
    (*(*p_obj_exhaacplus_dec).p_state_aac).preroll_config_present = 0 as UWORD8;
    if !((*p_obj_exhaacplus_dec).p_state_aac).is_null() {
        ret_val = _setjmp(
            ((*(*p_obj_exhaacplus_dec).p_state_aac).xaac_jmp_buf).as_mut_ptr(),
        ) as WORD32;
        if ret_val != 0 as core::ffi::c_int {
            (*(*p_obj_exhaacplus_dec).p_state_aac).i_bytes_consumed = (*(*p_obj_exhaacplus_dec)
                .p_state_aac)
                .ui_in_bytes as WORD32;
            (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = 0 as UWORD32;
            return IA_NO_ERROR;
        }
    }
    time_data = *((*p_obj_exhaacplus_dec).pp_mem_aac)
        .offset(IA_ENHAACPLUS_DEC_OUTPUT_IDX as isize) as *mut WORD16;
    if (*p_obj_exhaacplus_dec).aac_config.ui_flush_cmd == 0 as core::ffi::c_int {
        in_buffer = *((*p_obj_exhaacplus_dec).pp_mem_aac)
            .offset(IA_ENHAACPLUS_DEC_INPUT_IDX as isize) as *mut UWORD8;
    } else {
        in_buffer = (*(*p_obj_exhaacplus_dec).p_state_aac).header_ptr;
    }
    p_state_enhaacplus_dec = (*p_obj_exhaacplus_dec).p_state_aac;
    (*p_state_enhaacplus_dec).aac_scratch_mem_v = *((*p_obj_exhaacplus_dec).pp_mem_aac)
        .offset(IA_ENHAACPLUS_DEC_SCRATCH_IDX as isize) as *mut core::ffi::c_void;
    (*(*p_obj_exhaacplus_dec).p_state_aac).huffman_code_book_scl = ((*(*p_obj_exhaacplus_dec)
        .aac_tables
        .pstr_huffmann_tables)
        .huffman_code_book_scl)
        .as_mut_ptr();
    mps_buffer = ((*(*p_obj_exhaacplus_dec).p_state_aac).mps_buffer).as_mut_ptr();
    (*p_state_enhaacplus_dec).mps_header = -(1 as core::ffi::c_int) as WORD32;
    (*(*p_obj_exhaacplus_dec).p_state_aac).huffman_code_book_scl_index = ((*(*p_obj_exhaacplus_dec)
        .aac_tables
        .pstr_huffmann_tables)
        .huffman_code_book_scl_index)
        .as_mut_ptr();
    (*p_state_enhaacplus_dec).pstr_aac_tables = &mut (*p_obj_exhaacplus_dec).aac_tables;
    if (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done == 0 as UWORD32 {
        (*p_obj_exhaacplus_dec).aac_config.header_dec_done = 0 as UWORD32;
        (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.ldmps_present_flag = 0
            as UINT32;
    }
    if (*p_obj_exhaacplus_dec).aac_config.header_dec_done == 0 as UWORD32 {
        let mut channels: WORD32 = 0;
        let mut total_persistent_used: UWORD32 = 0 as UWORD32;
        (*(*p_obj_exhaacplus_dec).p_state_aac).p_config = &mut (*p_obj_exhaacplus_dec)
            .aac_config;
        (*(*p_obj_exhaacplus_dec).p_state_aac).pstr_stream_sbr = ((*p_obj_exhaacplus_dec)
            .p_state_aac as pWORD8)
            .offset(
                ((::core::mem::size_of::<ia_aac_dec_state_struct>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    as isize,
            ) as pVOID as *mut [ia_aac_dec_sbr_bitstream_struct; 2];
        if (*p_obj_exhaacplus_dec).aac_config.ui_max_channels > 2 as UWORD32 {
            (*p_state_enhaacplus_dec).aac_persistent_mem_v = ((*(*p_obj_exhaacplus_dec)
                .p_state_aac)
                .pstr_stream_sbr as pWORD8)
                .offset(
                    (((8 as core::ffi::c_int + MAX_CC_CHANNEL_NUM)
                        * 2 as core::ffi::c_int) as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_sbr_bitstream_struct>()
                                as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) as isize,
                ) as pVOID as *mut core::ffi::c_void;
            memset(
                (*(*p_obj_exhaacplus_dec).p_state_aac).pstr_stream_sbr
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                (((8 as core::ffi::c_int + MAX_CC_CHANNEL_NUM) * 2 as core::ffi::c_int)
                    as size_t)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_sbr_bitstream_struct>()
                            as size_t)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ),
            );
        } else {
            (*p_state_enhaacplus_dec).aac_persistent_mem_v = ((*(*p_obj_exhaacplus_dec)
                .p_state_aac)
                .pstr_stream_sbr as pWORD8)
                .offset(
                    ((2 as core::ffi::c_int * 2 as core::ffi::c_int) as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_sbr_bitstream_struct>()
                                as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) as isize,
                ) as pVOID as *mut core::ffi::c_void;
            memset(
                (*(*p_obj_exhaacplus_dec).p_state_aac).pstr_stream_sbr
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ((2 as core::ffi::c_int * 2 as core::ffi::c_int) as size_t)
                    .wrapping_mul(
                        (::core::mem::size_of::<ia_aac_dec_sbr_bitstream_struct>()
                            as size_t)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as size_t,
                    ),
            );
        }
        if 1 as UWORD32 == (*p_obj_exhaacplus_dec).aac_config.ui_max_channels {
            channels = 1 as core::ffi::c_int as WORD32;
        } else {
            channels = 2 as core::ffi::c_int as WORD32;
        }
        persistent_used = ixheaacd_set_aac_persistent_buffers(
            (*p_state_enhaacplus_dec).aac_persistent_mem_v,
            channels,
        );
        (*p_state_enhaacplus_dec).sbr_persistent_mem_v = ((*p_state_enhaacplus_dec)
            .aac_persistent_mem_v as pWORD8)
            .offset(
                (persistent_used as core::ffi::c_int
                    + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
            ) as pVOID as *mut core::ffi::c_void;
        total_persistent_used = total_persistent_used
            .wrapping_add(
                (persistent_used as core::ffi::c_int
                    + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as UWORD32,
            );
        persistent_used = ixheaacd_getsize_sbr_persistent();
        ixheaacd_set_sbr_persistent_buffers(
            (*p_state_enhaacplus_dec).sbr_persistent_mem_v,
            &mut persistent_used,
            channels,
            1 as WORD32,
        );
        (*p_state_enhaacplus_dec).heaac_mps_handle.mps_persistent_mem_v = ((*p_state_enhaacplus_dec)
            .sbr_persistent_mem_v as pWORD8)
            .offset(
                (persistent_used as core::ffi::c_int
                    + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as isize,
            ) as pVOID as *mut core::ffi::c_void;
        total_persistent_used = total_persistent_used
            .wrapping_add(
                (persistent_used as core::ffi::c_int
                    + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as UWORD32,
            );
        persistent_used = ixheaacd_getsize_mps_persistent();
        ixheaacd_set_mps_persistent_buffers(
            &mut (*p_state_enhaacplus_dec).heaac_mps_handle,
            &mut persistent_used,
            channels,
            (*p_state_enhaacplus_dec).heaac_mps_handle.mps_persistent_mem_v,
        );
        total_persistent_used = total_persistent_used
            .wrapping_add(
                (persistent_used as core::ffi::c_int
                    + (8 as core::ffi::c_int - 1 as core::ffi::c_int)
                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int)) as UWORD32,
            );
        if total_persistent_used
            > (*((*p_obj_exhaacplus_dec).p_mem_info_aac)
                .offset(IA_ENHAACPLUS_DEC_PERSIST_IDX as isize))
                .ui_size
        {
            return IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL as WORD32;
        }
        aac_persistent_mem = (*p_state_enhaacplus_dec).aac_persistent_mem_v
            as *mut ia_aac_persistent_struct;
        if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
            == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
            || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
        {
            (*p_state_enhaacplus_dec).frame_len_flag = (*p_obj_exhaacplus_dec)
                .aac_config
                .framesize_480 as WORD32;
        }
        (*p_state_enhaacplus_dec).ptr_overlap_buf = (*aac_persistent_mem).overlap_buffer;
        (*p_state_enhaacplus_dec).bit_count = 0 as core::ffi::c_int as WORD32;
        (*p_state_enhaacplus_dec).ec_enable = (*p_obj_exhaacplus_dec)
            .aac_config
            .ui_err_conceal;
        (*p_state_enhaacplus_dec).sync_status = 0 as core::ffi::c_int as WORD32;
        (*p_state_enhaacplus_dec).bs_format = ADTS_BSFORMAT as WORD32;
        (*p_state_enhaacplus_dec).latm_initialized = 0 as core::ffi::c_int as WORD32;
        (*p_state_enhaacplus_dec).frame_size = 0 as core::ffi::c_int as WORD32;
        memset(
            &mut (*p_state_enhaacplus_dec).latm_struct_element
                as *mut ixheaacd_latm_struct as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<ixheaacd_latm_struct>() as size_t,
        );
        memset(
            &mut (*p_state_enhaacplus_dec).b_n_raw_data_blk as *mut WORD16
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<WORD32>() as size_t)
                .wrapping_mul((9 as core::ffi::c_int + MAX_BS_ELEMENT) as size_t),
        );
        (*p_state_enhaacplus_dec).sbr_present_flag = 0 as core::ffi::c_int as WORD32;
        ch_idx = 0 as core::ffi::c_int as WORD;
        while ch_idx < MAX_BS_ELEMENT {
            (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize] = 0
                as ia_handle_sbr_dec_inst_struct;
            ch_idx += 1;
        }
        memset(
            &mut (*p_state_enhaacplus_dec).ind_cc_info as *mut ia_enhaacplus_dec_ind_cc
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<ia_enhaacplus_dec_ind_cc>() as size_t,
        );
        (*p_state_enhaacplus_dec).last_frame_ok = 1 as core::ffi::c_int as WORD32;
        (*p_obj_exhaacplus_dec).aac_config.header_dec_done = 1 as UWORD32;
        (*aac_persistent_mem).str_aac_decoder.pstr_aac_tables = &mut (*p_obj_exhaacplus_dec)
            .aac_tables;
        (*aac_persistent_mem).str_aac_decoder.pstr_common_tables = (*p_obj_exhaacplus_dec)
            .common_tables;
        (*(*p_obj_exhaacplus_dec).p_state_aac).sbr_persistent_mem_u = (*(*p_obj_exhaacplus_dec)
            .p_state_aac)
            .sbr_persistent_mem_v;
        (*(*p_obj_exhaacplus_dec).p_state_aac).sbr_scratch_mem_u = (*(*p_obj_exhaacplus_dec)
            .p_state_aac)
            .aac_scratch_mem_v;
        ixheaacd_set_sbr_persistent_table_pointer(
            (*(*p_obj_exhaacplus_dec).p_state_aac).sbr_persistent_mem_v,
            &mut (*p_obj_exhaacplus_dec).str_sbr_tables,
            (*p_obj_exhaacplus_dec).common_tables,
        );
        ixheaacd_set_scratch_buffers(
            &mut (*(*p_obj_exhaacplus_dec).p_state_aac).heaac_mps_handle,
            (*p_state_enhaacplus_dec).aac_scratch_mem_v,
        );
    }
    if (*(*p_obj_exhaacplus_dec).p_state_aac).ui_input_over == 1 as UWORD32 {
        return IA_XHEAAC_DEC_INIT_FATAL_EO_INPUT_REACHED as WORD32;
    }
    if (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done == 0 as UWORD32 {
        if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
            == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
            || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
        {
            (*p_state_enhaacplus_dec).frame_len_flag = (*p_obj_exhaacplus_dec)
                .aac_config
                .framesize_480 as WORD32;
        }
        aac_persistent_mem = (*p_state_enhaacplus_dec).aac_persistent_mem_v
            as *mut ia_aac_persistent_struct;
        sbr_persistent_mem = (*p_state_enhaacplus_dec).sbr_persistent_mem_v
            as *mut ia_sbr_pers_struct;
        if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
            (*(*p_obj_exhaacplus_dec).p_state_aac).first_frame = 1 as core::ffi::c_int
                as WORD32;
        }
        if (*p_obj_exhaacplus_dec).aac_config.ui_samp_freq == 0 as UWORD32 {
            let mut header_bytes_consumed: WORD32 = 0;
            let mut return_val: WORD32 = 0;
            if (*p_state_enhaacplus_dec).ui_in_bytes == 0 as UWORD32 {
                (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int
                    as WORD32;
                return IA_NO_ERROR;
            }
            if 1 as UWORD32 == (*p_obj_exhaacplus_dec).aac_config.ui_frame_size {
                (*p_state_enhaacplus_dec).frame_len_flag = 1 as core::ffi::c_int
                    as WORD32;
                (*p_state_enhaacplus_dec).frame_length = 960 as core::ffi::c_int
                    as WORD32;
            } else {
                (*p_state_enhaacplus_dec).frame_len_flag = 0 as core::ffi::c_int
                    as WORD32;
                (*p_state_enhaacplus_dec).frame_length = 1024 as core::ffi::c_int
                    as WORD32;
            }
            (*p_state_enhaacplus_dec).ui_init_done = 0 as UWORD32;
            memset(
                &mut (*p_state_enhaacplus_dec).eld_specific_config
                    as *mut ia_eld_specific_config_struct as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<ia_eld_specific_config_struct>() as size_t,
            );
            return_val = ixheaacd_aac_headerdecode(
                p_obj_exhaacplus_dec,
                in_buffer,
                &mut header_bytes_consumed,
                (*(*aac_persistent_mem).str_aac_decoder.pstr_aac_tables)
                    .pstr_huffmann_tables,
            );
            if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
                || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
                || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    == AOT_ER_AAC_LC as core::ffi::c_int as core::ffi::c_uint
            {
                *(*sbr_persistent_mem)
                    .str_sbr_dec_inst
                    .pstr_sbr_header[0 as core::ffi::c_int as usize] = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .str_sbr_config;
                *(*sbr_persistent_mem)
                    .str_sbr_dec_inst
                    .pstr_sbr_header[1 as core::ffi::c_int as usize] = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .str_sbr_config;
            } else {
                memset(
                    &mut (*p_state_enhaacplus_dec).eld_specific_config
                        as *mut ia_eld_specific_config_struct as *mut core::ffi::c_void,
                    0 as core::ffi::c_int,
                    ::core::mem::size_of::<ia_eld_specific_config_struct>() as size_t,
                );
            }
            if return_val < 0 as core::ffi::c_int {
                if return_val == IA_XHEAAC_DEC_INIT_FATAL_STREAM_CHAN_GT_MAX as WORD32 {
                    (*p_state_enhaacplus_dec).i_bytes_consumed = header_bytes_consumed;
                    return return_val;
                }
                (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                    as WORD32;
                return return_val;
            }
            if return_val == IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES {
                (*p_state_enhaacplus_dec).i_bytes_consumed = header_bytes_consumed;
                return return_val;
            }
            (*p_state_enhaacplus_dec).i_bytes_consumed = header_bytes_consumed;
            if return_val == 0 as core::ffi::c_int
                && (*(*p_obj_exhaacplus_dec).p_state_aac).audio_object_type
                    as core::ffi::c_uint
                    == AOT_USAC as core::ffi::c_int as core::ffi::c_uint
            {
                let mut pcm_size: WORD32 = (*p_obj_exhaacplus_dec).aac_config.ui_pcm_wdsz
                    as WORD32;
                let mut inbuffer: *mut WORD8 = *((*p_obj_exhaacplus_dec).pp_mem_aac)
                    .offset(IA_ENHAACPLUS_DEC_INPUT_IDX as isize) as *mut WORD8;
                let mut outbuffer: *mut WORD8 = *((*p_obj_exhaacplus_dec).pp_mem_aac)
                    .offset(IA_ENHAACPLUS_DEC_OUTPUT_IDX as isize) as *mut WORD8;
                let mut out_bytes: WORD32 = 0 as WORD32;
                let mut frames_done: WORD32 = (*(*p_obj_exhaacplus_dec).p_state_aac)
                    .frame_counter;
                (*(*p_obj_exhaacplus_dec).p_state_aac).decode_create_done = 0
                    as core::ffi::c_int as WORD32;
                if (*(*p_obj_exhaacplus_dec).p_state_aac).ui_input_over == 0 as UWORD32 {
                    error_code = ixheaacd_dec_main(
                        p_obj_exhaacplus_dec as *mut core::ffi::c_void,
                        inbuffer,
                        outbuffer,
                        &mut out_bytes,
                        frames_done,
                        pcm_size,
                        &mut (*(*p_obj_exhaacplus_dec).p_state_aac).num_of_output_ch,
                    );
                    if error_code != 0 {
                        return error_code;
                    }
                    (*(*p_obj_exhaacplus_dec).p_state_aac).frame_counter += 1;
                } else {
                    out_bytes = 0 as core::ffi::c_int as WORD32;
                }
                (*p_obj_exhaacplus_dec).aac_config.ui_n_channels = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .num_of_output_ch as UWORD32;
                if return_val == 0 as core::ffi::c_int {
                    (*(*p_obj_exhaacplus_dec).p_state_aac).ui_init_done = 1 as UWORD32;
                }
                return return_val;
            }
            if return_val == 0 as core::ffi::c_int {
                (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done = 1 as UWORD32;
                if (*p_obj_exhaacplus_dec).aac_config.ui_flush_cmd
                    == 0 as core::ffi::c_int
                {
                    memcpy(
                        (*p_state_enhaacplus_dec).header_ptr as *mut core::ffi::c_void,
                        in_buffer as *const core::ffi::c_void,
                        (header_bytes_consumed as size_t)
                            .wrapping_mul(::core::mem::size_of::<UWORD8>() as size_t),
                    );
                    (*p_state_enhaacplus_dec).header_length = header_bytes_consumed;
                }
            }
            if (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done != 1 as UWORD32 {
                return IA_XHEAAC_DEC_INIT_NONFATAL_HEADER_NOT_AT_START;
            }
            if (*p_state_enhaacplus_dec).dwnsmp_signal == 1 as core::ffi::c_int
                && (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
            {
                (*p_obj_exhaacplus_dec).aac_config.down_sample_flag = 1 as UWORD32;
            }
            if (*p_state_enhaacplus_dec).sampling_rate
                == (*p_state_enhaacplus_dec).extension_samp_rate
            {
                (*p_obj_exhaacplus_dec).aac_config.down_sample_flag = 1 as UWORD32;
            }
        } else {
            (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done = 1 as UWORD32;
            (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int as WORD32;
            (*p_state_enhaacplus_dec).sampling_rate = (*p_obj_exhaacplus_dec)
                .aac_config
                .ui_samp_freq;
            if 1 as UWORD32 == (*p_obj_exhaacplus_dec).aac_config.ui_frame_size {
                (*p_state_enhaacplus_dec).frame_len_flag = 1 as core::ffi::c_int
                    as WORD32;
                (*p_state_enhaacplus_dec).frame_length = 960 as core::ffi::c_int
                    as WORD32;
            } else {
                (*p_state_enhaacplus_dec).frame_len_flag = 0 as core::ffi::c_int
                    as WORD32;
                (*p_state_enhaacplus_dec).frame_length = 1024 as core::ffi::c_int
                    as WORD32;
            }
        }
        (*p_state_enhaacplus_dec).pstr_bit_buf = ixheaacd_create_bit_buf(
            &mut (*p_state_enhaacplus_dec).str_bit_buf,
            in_buffer,
            (*((*p_obj_exhaacplus_dec).p_mem_info_aac)
                .offset(IA_ENHAACPLUS_DEC_INPUT_IDX as isize))
                .ui_size as WORD32,
        ) as *mut ia_bit_buf_struct;
        (*(*p_state_enhaacplus_dec).pstr_bit_buf).xaac_jmp_buf = &mut (*p_state_enhaacplus_dec)
            .xaac_jmp_buf;
        (*p_state_enhaacplus_dec).ptr_bit_stream = (*p_state_enhaacplus_dec).pstr_bit_buf
            as *mut ia_bit_buf_struct;
        if (*p_state_enhaacplus_dec).s_adts_hdr_present != 0 {
            if (*p_obj_exhaacplus_dec).aac_config.ld_decoder == 1 as core::ffi::c_int {
                (*p_state_enhaacplus_dec).audio_object_type = AOT_ER_AAC_LD;
            }
        }
        if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
            == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
            || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
        {
            if (*p_state_enhaacplus_dec).s_adts_hdr_present != 0 {
                if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
                {
                    (*p_state_enhaacplus_dec).eld_specific_config.ld_sbr_samp_rate = 1
                        as core::ffi::c_int as WORD32;
                    (*p_state_enhaacplus_dec).eld_specific_config.ld_sbr_crc_flag = 0
                        as core::ffi::c_int as WORD32;
                    (*p_state_enhaacplus_dec).eld_specific_config.ld_sbr_flag_present = 0
                        as core::ffi::c_int as WORD32;
                    if (*p_obj_exhaacplus_dec).aac_config.eld_sbr_present
                        == 1 as core::ffi::c_int
                    {
                        (*p_state_enhaacplus_dec)
                            .eld_specific_config
                            .ld_sbr_flag_present = 1 as core::ffi::c_int as WORD32;
                    }
                }
                if (*p_obj_exhaacplus_dec).aac_config.framesize_480 != 0 {
                    (*p_state_enhaacplus_dec).frame_length = 480 as core::ffi::c_int
                        as WORD32;
                } else {
                    (*p_state_enhaacplus_dec).frame_length = 512 as core::ffi::c_int
                        as WORD32;
                }
            }
        }
        ch_idx = 0 as core::ffi::c_int as WORD;
        while ch_idx < MAX_BS_ELEMENT {
            let mut channels_0: WORD32 = 0;
            channels_0 = 2 as core::ffi::c_int as WORD32;
            (*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize] = ixheaacd_aac_decoder_init(
                p_state_enhaacplus_dec,
                (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr(),
                channels_0 as WORD,
                (*p_state_enhaacplus_dec).aac_persistent_mem_v,
                (*p_state_enhaacplus_dec).frame_length,
            );
            if ((*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize]).is_null() {
                (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                    as WORD32;
                return IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL as WORD32;
            }
            ch_idx += 1;
        }
        (*p_state_enhaacplus_dec).pstr_drc_dec = &mut (*p_state_enhaacplus_dec)
            .str_drc_dec_info;
        ixheaacd_drc_dec_create(
            (*p_state_enhaacplus_dec).pstr_drc_dec,
            127 as WORD16,
            127 as WORD16,
        );
        (*(*p_state_enhaacplus_dec).pstr_drc_dec).cut_factor = (*p_obj_exhaacplus_dec)
            .aac_config
            .ui_drc_cut;
        (*(*p_state_enhaacplus_dec).pstr_drc_dec).boost_factor = (*p_obj_exhaacplus_dec)
            .aac_config
            .ui_drc_boost;
        (*(*p_state_enhaacplus_dec).pstr_drc_dec).target_ref_level = (*p_obj_exhaacplus_dec)
            .aac_config
            .ui_drc_target_level;
        (*(*p_state_enhaacplus_dec).pstr_drc_dec).prog_ref_level = (*p_obj_exhaacplus_dec)
            .aac_config
            .ui_drc_target_level;
        if 1 as core::ffi::c_int == (*p_obj_exhaacplus_dec).aac_config.ui_drc_set {
            if (*p_obj_exhaacplus_dec).aac_config.ui_drc_heavy_comp
                == 1 as core::ffi::c_int
            {
                (*(*p_state_enhaacplus_dec).pstr_drc_dec).drc_on = 1 as core::ffi::c_int
                    as FLAG;
                (*(*p_state_enhaacplus_dec).pstr_drc_dec).heavy_mode = 1
                    as core::ffi::c_int as WORD32;
            } else {
                (*(*p_state_enhaacplus_dec).pstr_drc_dec).heavy_mode = 0
                    as core::ffi::c_int as WORD32;
                if (*(*p_state_enhaacplus_dec).pstr_drc_dec).target_ref_level
                    > 127 as core::ffi::c_int
                {
                    (*(*p_state_enhaacplus_dec).pstr_drc_dec).target_ref_level = 127
                        as core::ffi::c_int as WORD32;
                }
                if (*(*p_state_enhaacplus_dec).pstr_drc_dec).target_ref_level
                    < 0 as core::ffi::c_int
                {
                    if (*(*p_state_enhaacplus_dec).pstr_drc_dec).cut_factor
                        > 0 as core::ffi::c_int
                        || (*(*p_state_enhaacplus_dec).pstr_drc_dec).boost_factor
                            > 0 as core::ffi::c_int
                    {
                        (*(*p_state_enhaacplus_dec).pstr_drc_dec).drc_on = 1
                            as core::ffi::c_int as FLAG;
                    } else {
                        (*(*p_state_enhaacplus_dec).pstr_drc_dec).drc_on = 0
                            as core::ffi::c_int as FLAG;
                    }
                    (*(*p_state_enhaacplus_dec).pstr_drc_dec).drc_dig_norm = 0
                        as core::ffi::c_int as FLAG;
                    (*(*p_state_enhaacplus_dec).pstr_drc_dec).target_ref_level = 108
                        as core::ffi::c_int as WORD32;
                } else {
                    (*(*p_state_enhaacplus_dec).pstr_drc_dec).drc_on = 1
                        as core::ffi::c_int as FLAG;
                    (*(*p_state_enhaacplus_dec).pstr_drc_dec).drc_dig_norm = 1
                        as core::ffi::c_int as FLAG;
                }
            }
        }
    } else {
        let mut temp_bit_buff: ia_bit_buf_struct = {
            let mut init = ia_bit_buf_struct {
                ptr_bit_buf_base: 0 as *mut UWORD8,
                ptr_bit_buf_end: 0 as *mut UWORD8,
                ptr_read_next: 0 as *mut UWORD8,
                bit_pos: 0,
                cnt_bits: 0,
                size: 0,
                adts_header_present: 0,
                crc_check: 0,
                protection_absent: 0,
                no_raw_data_blocks: 0,
                str_adts_crc_info: ia_adts_crc_info_struct {
                    crc_active: 0,
                    no_reg: 0,
                    file_value: 0,
                    crc_lookup: [0; 256],
                    str_crc_reg_data: [ia_crc_reg_data_struct {
                        active: 0,
                        buf_size: 0,
                        max_bits: 0,
                        bit_cnt: 0,
                        bit_buf_cnt: 0,
                        str_bit_buf: ia_crc_bit_buf_struct {
                            ptr_bit_buf_base: 0 as *mut UWORD8,
                            ptr_bit_buf_end: 0 as *mut UWORD8,
                            ptr_read_next: 0 as *mut UWORD8,
                            bit_pos: 0,
                            cnt_bits: 0,
                            size: 0,
                        },
                    }; 7],
                },
                pstr_adts_crc_info: 0 as *mut ia_adts_crc_info_struct,
                initial_cnt_bits: 0,
                audio_mux_align: 0,
                bit_count: 0,
                valid_bits: 0,
                byte: 0,
                byte_ptr: 0 as *mut UWORD8,
                ptr_start: 0 as *mut UWORD8,
                write_bit_count: 0,
                max_size: 0,
                xaac_jmp_buf: 0 as *mut jmp_buf,
            };
            init
        };
        let mut adts: ia_adts_header_struct = ia_adts_header_struct {
            sync_word: 0,
            id: 0,
            layer: 0,
            protection_absent: 0,
            profile: 0,
            samp_freq_index: 0,
            channel_configuration: 0,
            aac_frame_length: 0,
            no_raw_data_blocks: 0,
            crc_check: 0,
        };
        let mut it_bit_buff: *mut ia_bit_buf_struct = 0 as *mut ia_bit_buf_struct;
        let mut frame_size_2: WORD16 = 0 as WORD16;
        let mut sample_rate_2: WORD32 = 0 as WORD32;
        let mut sample_rate: WORD32 = 0 as WORD32;
        let mut type_0: WORD = 0;
        let mut i: WORD = 0;
        let mut elements_number: WORD = 0;
        if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
            == AOT_USAC as core::ffi::c_int as core::ffi::c_uint
        {
            return IA_FATAL_ERROR as WORD32;
        }
        memset(
            &mut adts as *mut ia_adts_header_struct as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<ia_adts_header_struct>() as size_t,
        );
        i = 0 as core::ffi::c_int as WORD;
        while i < MAX_BS_ELEMENT + 1 as core::ffi::c_int {
            (*p_obj_exhaacplus_dec).aac_config.element_type[i as usize] = -(1
                as core::ffi::c_int) as WORD32;
            i += 1;
        }
        it_bit_buff = (*p_state_enhaacplus_dec).pstr_bit_buf;
        (*p_obj_exhaacplus_dec).aac_config.ui_sbr_mode = 0 as UWORD32;
        (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = 0 as UWORD32;
        (*p_state_enhaacplus_dec).ui_mps_out_bytes = 0 as core::ffi::c_int as WORD32;
        if (*p_state_enhaacplus_dec).ui_in_bytes == 0 as UWORD32 {
            (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int as WORD32;
            return IA_NO_ERROR;
        }
        if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
            == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
            || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
        {
            if (*p_obj_exhaacplus_dec).aac_config.ui_mp4_flag != 0 {
                (*p_state_enhaacplus_dec).frame_size = (*p_state_enhaacplus_dec)
                    .ui_in_bytes as WORD32;
            }
        }
        ixheaacd_create_init_bit_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            in_buffer,
            (*p_state_enhaacplus_dec).ui_in_bytes as WORD32,
        );
        (*(*p_state_enhaacplus_dec).pstr_bit_buf).xaac_jmp_buf = &mut (*p_state_enhaacplus_dec)
            .xaac_jmp_buf;
        (*it_bit_buff).adts_header_present = (*p_state_enhaacplus_dec).s_adts_hdr_present
            as WORD32;
        (*it_bit_buff).no_raw_data_blocks = (*p_state_enhaacplus_dec).b_n_raw_data_blk
            as WORD8;
        (*it_bit_buff).protection_absent = (*p_state_enhaacplus_dec).protection_absent;
        memcpy(
            &mut temp_bit_buff as *mut ia_bit_buf_struct as *mut core::ffi::c_void,
            it_bit_buff as *const core::ffi::c_void,
            ::core::mem::size_of::<ia_bit_buf_struct>() as size_t,
        );
        if (*p_obj_exhaacplus_dec).aac_config.ui_max_channels > 2 as UWORD32 {
            elements_number = MAX_BS_ELEMENT as WORD;
        } else {
            elements_number = 2 as core::ffi::c_int as WORD;
        }
        i = 0 as core::ffi::c_int as WORD;
        while i < elements_number {
            (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                .offset(i as isize))[0 as core::ffi::c_int as usize]
                .no_elements = 0 as WORD16;
            i += 1;
        }
        (*it_bit_buff).initial_cnt_bits = (*it_bit_buff).cnt_bits;
        ixheaacd_byte_align(
            (*p_state_enhaacplus_dec).ptr_bit_stream,
            &mut (**((*p_state_enhaacplus_dec).pstr_aac_dec_info)
                .as_mut_ptr()
                .offset(0 as core::ffi::c_int as isize))
                .byte_align_bits,
        );
        if (*p_state_enhaacplus_dec).s_adts_hdr_present != 0 {
            let mut error: WORD32 = 0;
            if (*p_state_enhaacplus_dec).b_n_raw_data_blk as core::ffi::c_int
                == 0 as core::ffi::c_int
            {
                error = ixheaacd_readifadts(
                    p_state_enhaacplus_dec,
                    it_bit_buff,
                    &mut adts,
                );
                if error != 0 {
                    return error;
                }
                (*p_state_enhaacplus_dec).protection_absent = adts.protection_absent
                    as WORD8;
                if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
                    || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                        == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
                {
                    (*p_state_enhaacplus_dec).frame_size = adts.aac_frame_length
                        as WORD32;
                    if (*p_obj_exhaacplus_dec).aac_config.framesize_480 != 0 {
                        (*p_state_enhaacplus_dec).frame_length = 480 as core::ffi::c_int
                            as WORD32;
                    } else {
                        (*p_state_enhaacplus_dec).frame_length = 512 as core::ffi::c_int
                            as WORD32;
                    }
                }
            }
        }
        if (*p_state_enhaacplus_dec).bs_format == LOAS_BSFORMAT {
            let mut result: WORD32 = 0;
            let mut sync: WORD32 = 0;
            let mut cnt_bits: WORD32 = 0;
            sync = ixheaacd_read_bits_buf(
                it_bit_buff as *mut ia_bit_buf_struct,
                11 as WORD,
            );
            cnt_bits = (*it_bit_buff).cnt_bits;
            if (*it_bit_buff).cnt_bits <= 24 as core::ffi::c_int {
                return IA_XHEAAC_DEC_INIT_NONFATAL_INSUFFICIENT_INPUT_BYTES;
            }
            while sync != 0x2b7 as core::ffi::c_int {
                sync = (sync & 0x3ff as WORD32) << 1 as core::ffi::c_int
                    | ixheaacd_read_bits_buf(
                        it_bit_buff as *mut ia_bit_buf_struct,
                        1 as WORD,
                    );
                if (*it_bit_buff).cnt_bits < 11 as core::ffi::c_int {
                    ixheaacd_read_bidirection(
                        it_bit_buff as *mut ia_bit_buf_struct,
                        -(11 as WORD32),
                    );
                    (*p_state_enhaacplus_dec).i_bytes_consumed = ((cnt_bits
                        as core::ffi::c_int
                        - (*it_bit_buff).cnt_bits as core::ffi::c_int)
                        / 8 as core::ffi::c_int) as WORD32;
                    return 0x1000 as WORD32;
                }
            }
            (*it_bit_buff).audio_mux_align = ((*it_bit_buff).cnt_bits as core::ffi::c_int
                - 13 as core::ffi::c_int) as WORD32;
            if sync == 0x2b7 as core::ffi::c_int {
                result = ixheaacd_latm_audio_mux_element(
                    it_bit_buff,
                    &mut (*p_state_enhaacplus_dec).latm_struct_element,
                    p_state_enhaacplus_dec,
                    &mut *((*(*p_obj_exhaacplus_dec).aac_tables.pstr_huffmann_tables)
                        .str_sample_rate_info)
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize)
                        as *mut ia_sampling_rate_info_struct,
                );
                if result < 0 as core::ffi::c_int {
                    return result;
                }
            }
        }
        (*(*p_state_enhaacplus_dec).pstr_aac_dec_info[0 as core::ffi::c_int as usize])
            .byte_align_bits = (*it_bit_buff).cnt_bits;
        type_0 = -(1 as core::ffi::c_int) as WORD;
        ch_idx = 0 as core::ffi::c_int as WORD;
        while type_0 != 7 as core::ffi::c_int {
            let mut aac_scratch_struct: ia_aac_dec_scratch_struct = ia_aac_dec_scratch_struct {
                base_scr_8k: 0 as *mut core::ffi::c_void,
                extra_scr_4k: [0 as *mut core::ffi::c_void; 4],
                in_data: 0 as *mut WORD32,
                out_data: 0 as *mut WORD32,
            };
            memset(
                &mut aac_scratch_struct as *mut ia_aac_dec_scratch_struct
                    as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<ia_aac_dec_scratch_struct>() as size_t,
            );
            if ch_idx >= elements_number {
                (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                    as WORD32;
                return IA_XHEAAC_DEC_INIT_FATAL_STREAM_CHAN_GT_MAX as WORD32;
            }
            ixheaacd_allocate_aac_scr(
                &mut aac_scratch_struct,
                (*p_state_enhaacplus_dec).aac_scratch_mem_v,
                time_data as *mut core::ffi::c_void,
                1 as WORD,
                (*p_obj_exhaacplus_dec).aac_config.ui_max_channels as WORD,
                (*p_state_enhaacplus_dec).audio_object_type as WORD32,
            );
            (*(*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize])
                .p_ind_channel_info = &mut (*p_state_enhaacplus_dec).ind_cc_info
                as *mut ia_enhaacplus_dec_ind_cc as *mut core::ffi::c_void;
            if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                (*p_obj_exhaacplus_dec).aac_config.first_frame = 1 as core::ffi::c_int
                    as FLAG;
            }
            error_code = ixheaacd_aacdec_decodeframe(
                p_obj_exhaacplus_dec,
                &mut aac_scratch_struct,
                time_data as *mut core::ffi::c_void,
                frame_status,
                &mut type_0,
                &mut ch_idx,
                1 as WORD,
                2 as WORD,
                ((*p_obj_exhaacplus_dec).aac_config.element_instance_order).as_mut_ptr(),
                0 as WORD,
                1 as WORD,
                0 as WORD,
                (*p_obj_exhaacplus_dec).aac_config.ui_max_channels as WORD,
                2 as WORD32,
                (*(*p_obj_exhaacplus_dec).p_state_aac).frame_length,
                (*(*p_obj_exhaacplus_dec).p_state_aac).frame_size,
                (*p_state_enhaacplus_dec).pstr_drc_dec,
                (*p_state_enhaacplus_dec).audio_object_type as WORD32,
                (*p_state_enhaacplus_dec).ch_config as WORD32,
                (*p_state_enhaacplus_dec).eld_specific_config,
                (*p_state_enhaacplus_dec).s_adts_hdr_present,
                &mut (*p_state_enhaacplus_dec).drc_dummy,
                (*p_state_enhaacplus_dec).ldmps_present,
                &mut (*p_state_enhaacplus_dec).slot_pos,
                mps_buffer,
                &mut (*p_state_enhaacplus_dec).mps_header,
                &mut (*p_state_enhaacplus_dec).ui_mps_out_bytes,
                1 as WORD32,
                (*p_obj_exhaacplus_dec).aac_config.first_frame as WORD32,
            );
            if (*(*p_state_enhaacplus_dec).pstr_drc_dec).drc_element_found
                as core::ffi::c_int == 1 as core::ffi::c_int
            {
                if (*p_obj_exhaacplus_dec).aac_config.i_loud_ref_level
                    < 0 as core::ffi::c_int
                {
                    (*p_obj_exhaacplus_dec).aac_config.output_level = (*(*p_state_enhaacplus_dec)
                        .pstr_drc_dec)
                        .prog_ref_level;
                } else {
                    (*p_obj_exhaacplus_dec).aac_config.output_level = (*p_obj_exhaacplus_dec)
                        .aac_config
                        .i_loud_ref_level;
                }
            }
            memset(
                &mut (**((**((*(*p_obj_exhaacplus_dec).p_state_aac).pstr_aac_dec_info)
                    .as_mut_ptr()
                    .offset(ch_idx as isize))
                    .pstr_aac_dec_ch_info)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .str_ics_info
                    .ltp as *mut ltp_info as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<ltp_info>() as size_t,
            );
            memset(
                &mut (**((**((*(*p_obj_exhaacplus_dec).p_state_aac).pstr_aac_dec_info)
                    .as_mut_ptr()
                    .offset(ch_idx as isize))
                    .pstr_aac_dec_ch_info)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize))
                    .str_ics_info
                    .ltp2 as *mut ltp_info as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<ltp_info>() as size_t,
            );
            memset(
                &mut (**((**((*(*p_obj_exhaacplus_dec).p_state_aac).pstr_aac_dec_info)
                    .as_mut_ptr()
                    .offset(ch_idx as isize))
                    .pstr_aac_dec_ch_info)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .str_ics_info
                    .ltp as *mut ltp_info as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<ltp_info>() as size_t,
            );
            memset(
                &mut (**((**((*(*p_obj_exhaacplus_dec).p_state_aac).pstr_aac_dec_info)
                    .as_mut_ptr()
                    .offset(ch_idx as isize))
                    .pstr_aac_dec_ch_info)
                    .as_mut_ptr()
                    .offset(1 as core::ffi::c_int as isize))
                    .str_ics_info
                    .ltp2 as *mut ltp_info as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<ltp_info>() as size_t,
            );
            frame_size_1 = (*p_state_enhaacplus_dec).frame_length;
            sample_rate_1 = (*(*p_state_enhaacplus_dec)
                .pstr_aac_dec_info[ch_idx as usize])
                .sampling_rate;
            num_channels_1 = (*(*p_state_enhaacplus_dec)
                .pstr_aac_dec_info[ch_idx as usize])
                .channels;
            if (*p_obj_exhaacplus_dec).aac_config.ui_max_channels <= 2 as UWORD32
                && (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                    == 2 as core::ffi::c_int
            {
                (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                    as WORD32;
                return IA_XHEAAC_DEC_INIT_FATAL_UNIMPLEMENTED_CCE as WORD32;
            }
            if (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                .no_elements != 0
            {
                sbr_present_flag = 1 as core::ffi::c_int as WORD;
                (*p_obj_exhaacplus_dec).aac_config.ui_sbr_mode = 1 as UWORD32;
            }
            if error_code != 0 {
                if (*p_state_enhaacplus_dec).ui_input_over != 0 {
                    return IA_XHEAAC_DEC_INIT_FATAL_EO_INPUT_REACHED as WORD32;
                }
                ixheaacd_updatebytesconsumed(p_state_enhaacplus_dec, it_bit_buff);
                return error_code;
            }
            if (*p_state_enhaacplus_dec).s_adts_hdr_present != 0 {
                if adts.no_raw_data_blocks != 0 as core::ffi::c_int {
                    if adts.protection_absent == 0 as core::ffi::c_int
                        && (*it_bit_buff).cnt_bits >= 16 as core::ffi::c_int
                    {
                        adts.crc_check = ixheaacd_read_bits_buf(
                            it_bit_buff as *mut ia_bit_buf_struct,
                            16 as WORD,
                        );
                    }
                }
                (*p_state_enhaacplus_dec).b_n_raw_data_blk -= 1;
            }
            sample_rate_2 = sample_rate_1;
            frame_size_2 = frame_size_1 as WORD16;
            if ((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null()
                && (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize]
                    .no_elements as core::ffi::c_int != 0
            {
                let mut harmonic_sbr_flag: WORD32 = 0 as WORD32;
                if (*p_obj_exhaacplus_dec).aac_config.flag_16khz_out == 1 as UWORD32
                    && sample_rate_1 == 8000 as core::ffi::c_int
                {
                    (*p_obj_exhaacplus_dec).aac_config.flag_16khz_out = 0 as UWORD32;
                }
                (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize] = ixheaacd_init_sbr(
                    sample_rate_1,
                    frame_size_1,
                    &mut (*p_obj_exhaacplus_dec).aac_config.down_sample_flag
                        as *mut UWORD32 as *mut FLAG,
                    (*p_state_enhaacplus_dec).sbr_persistent_mem_v,
                    (*p_state_enhaacplus_dec).ptr_overlap_buf,
                    MAXNRSBRCHANNELS,
                    1 as core::ffi::c_int,
                    1 as WORD,
                    frame_size_1 as WORD * 2 as WORD,
                    &mut harmonic_sbr_flag,
                    NULL as *mut core::ffi::c_void,
                    (*p_state_enhaacplus_dec).str_sbr_config,
                    (*p_state_enhaacplus_dec).audio_object_type as WORD,
                    (*p_state_enhaacplus_dec)
                        .mps_dec_handle
                        .ldmps_config
                        .ldmps_present_flag as WORD32,
                    (*p_state_enhaacplus_dec)
                        .mps_dec_handle
                        .ldmps_config
                        .no_ldsbr_present,
                );
                if !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                    .is_null()
                {
                    (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                        .xaac_jmp_buf = &mut (*p_state_enhaacplus_dec).xaac_jmp_buf;
                }
            }
            if !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null()
                && (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize]
                    .no_elements as core::ffi::c_int != 0
            {
                let mut sbr_scratch_struct: ia_sbr_scr_struct = ia_sbr_scr_struct {
                    ptr_work_buf_core: 0 as *mut core::ffi::c_void,
                };
                let mut num_channels_1_t: WORD16 = num_channels_1;
                ixheaacd_allocate_sbr_scr(
                    &mut sbr_scratch_struct,
                    (*p_state_enhaacplus_dec).aac_scratch_mem_v,
                    time_data as *mut core::ffi::c_void,
                    0 as WORD32,
                    0 as *mut WORD8,
                    0 as UWORD8,
                    0 as UWORD8,
                );
                let mut audio_object_type: WORD32 = (*p_state_enhaacplus_dec)
                    .audio_object_type as WORD32;
                if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
                    && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
                {
                    let mut i_0: WORD32 = 0 as WORD32;
                    let mut pstr_dec_data: *mut ia_dec_data_struct = (*p_state_enhaacplus_dec)
                        .pstr_dec_data as *mut ia_dec_data_struct;
                    if num_channels_1 as core::ffi::c_int == 1 as core::ffi::c_int {
                        while i_0 < 1024 as core::ffi::c_int {
                            (*pstr_dec_data)
                                .str_usac_data
                                .time_sample_vector[0 as core::ffi::c_int
                                as usize][i_0 as usize] = *time_data.offset(i_0 as isize)
                                as FLOAT32;
                            i_0 += 1;
                        }
                        (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                            .time_sample_buf[0 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data)
                            .str_usac_data
                            .time_sample_vector)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                        (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                            .time_sample_buf[1 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data)
                            .str_usac_data
                            .time_sample_vector)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                    } else if num_channels_1 as core::ffi::c_int == 2 as core::ffi::c_int
                    {
                        while i_0 < 1024 as core::ffi::c_int {
                            (*pstr_dec_data)
                                .str_usac_data
                                .time_sample_vector[0 as core::ffi::c_int
                                as usize][i_0 as usize] = *time_data
                                .offset(
                                    (2 as core::ffi::c_int * i_0 as core::ffi::c_int
                                        + 0 as core::ffi::c_int) as isize,
                                ) as FLOAT32;
                            (*pstr_dec_data)
                                .str_usac_data
                                .time_sample_vector[1 as core::ffi::c_int
                                as usize][i_0 as usize] = *time_data
                                .offset(
                                    (2 as core::ffi::c_int * i_0 as core::ffi::c_int
                                        + 1 as core::ffi::c_int) as isize,
                                ) as FLOAT32;
                            i_0 += 1;
                        }
                        (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                            .time_sample_buf[0 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data)
                            .str_usac_data
                            .time_sample_vector)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                        (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                            .time_sample_buf[1 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data)
                            .str_usac_data
                            .time_sample_vector)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                    }
                }
                (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).ec_flag = (*p_obj_exhaacplus_dec)
                    .aac_config
                    .ui_err_conceal as FLAG;
                (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).esbr_hq = (*p_obj_exhaacplus_dec)
                    .aac_config
                    .ui_hq_esbr as FLAG;
                (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).enh_sbr = (*p_obj_exhaacplus_dec)
                    .aac_config
                    .ui_enh_sbr as FLAG;
                (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                    .enh_sbr_ps = (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr_ps
                    as FLAG;
                if !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                    .is_null()
                {
                    (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                        .xaac_jmp_buf = &mut (*p_state_enhaacplus_dec).xaac_jmp_buf;
                }
                if ixheaacd_applysbr(
                    (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize],
                    &mut *(*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                        .offset(0 as core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize),
                    time_data,
                    &mut num_channels_1,
                    frame_status,
                    (*p_obj_exhaacplus_dec).aac_config.down_sample_flag as FLAG,
                    0 as FLAG,
                    &mut sbr_scratch_struct,
                    1 as WORD32,
                    1 as WORD32,
                    0 as WORD32,
                    0 as *mut ia_bit_buf_struct,
                    0 as *mut ia_drc_dec_struct,
                    (*p_state_enhaacplus_dec).eld_specific_config.ld_sbr_flag_present
                        as WORD,
                    (*p_state_enhaacplus_dec).audio_object_type as WORD,
                    1 as WORD32,
                    (*p_state_enhaacplus_dec).ldmps_present,
                    frame_size_1,
                    (*p_state_enhaacplus_dec).heaac_mps_handle.heaac_mps_present,
                    (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal,
                    (*p_obj_exhaacplus_dec).aac_config.first_frame,
                ) != SBRDEC_OK
                {
                    (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize] = 0
                        as ia_handle_sbr_dec_inst_struct;
                    return -(1 as WORD32);
                } else {
                    if (*p_obj_exhaacplus_dec).aac_config.down_sample_flag == 0 {
                        sample_rate_1 *= 2 as core::ffi::c_int;
                    }
                    if (*p_state_enhaacplus_dec).eld_specific_config.ld_sbr_flag_present
                        == 1 as core::ffi::c_int
                    {
                        (*(*p_obj_exhaacplus_dec).p_state_aac)
                            .mps_dec_handle
                            .pre_mix_req = 1 as core::ffi::c_int as WORD32;
                        ixheaacd_mps_payload(
                            (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize],
                            p_obj_exhaacplus_dec,
                        );
                    }
                }
                let mut audio_object_type_0: WORD32 = (*p_state_enhaacplus_dec)
                    .audio_object_type as WORD32;
                if audio_object_type_0 != AOT_ER_AAC_ELD as core::ffi::c_int
                    && audio_object_type_0 != AOT_ER_AAC_LD as core::ffi::c_int
                {
                    let mut out_bytes_0: WORD32 = 0 as WORD32;
                    let mut pstr_dec_data_0: *mut ia_dec_data_struct = (*p_state_enhaacplus_dec)
                        .pstr_dec_data as *mut ia_dec_data_struct;
                    ixheaacd_samples_sat(
                        time_data as *mut WORD8,
                        2048 as WORD32,
                        16 as WORD32,
                        ((*pstr_dec_data_0).str_usac_data.time_sample_vector)
                            .as_mut_ptr() as *mut [FLOAT32; 4096],
                        &mut out_bytes_0,
                        1 as WORD32,
                    );
                }
                if (*p_obj_exhaacplus_dec).aac_config.flag_downmix != 0 {
                    num_channels_1 = 1 as WORD16;
                }
                if num_channels_1_t as core::ffi::c_int == 1 as core::ffi::c_int
                    && num_channels_1 as core::ffi::c_int == 2 as core::ffi::c_int
                {
                    ps_detected = 1 as core::ffi::c_int as WORD32;
                }
            } else {
                (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.no_ldsbr_present = 1
                    as core::ffi::c_int as WORD32;
            }
            (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int as WORD32;
            (*p_state_enhaacplus_dec).pstr_bit_buf = it_bit_buff;
            (*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize] = ixheaacd_aac_decoder_init(
                p_state_enhaacplus_dec,
                (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                    .offset(0 as core::ffi::c_int as isize))
                    .as_mut_ptr(),
                2 as WORD,
                (*p_state_enhaacplus_dec).aac_persistent_mem_v,
                (*p_state_enhaacplus_dec).frame_length,
            );
            if ((*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize]).is_null() {
                (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                    as WORD32;
                return IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL as WORD32;
            }
            if !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null() {
                let mut harmonic_sbr_flag_0: WORD32 = 0 as WORD32;
                (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize] = ixheaacd_init_sbr(
                    sample_rate_2,
                    frame_size_2 as WORD32,
                    &mut (*p_obj_exhaacplus_dec).aac_config.down_sample_flag
                        as *mut UWORD32 as *mut FLAG,
                    (*p_state_enhaacplus_dec).sbr_persistent_mem_v,
                    (*p_state_enhaacplus_dec).ptr_overlap_buf,
                    MAXNRSBRCHANNELS,
                    1 as WORD,
                    1 as WORD,
                    frame_size_2 as WORD * 2 as WORD,
                    &mut harmonic_sbr_flag_0,
                    NULL as *mut core::ffi::c_void,
                    (*p_state_enhaacplus_dec).str_sbr_config,
                    (*p_state_enhaacplus_dec).audio_object_type as WORD,
                    (*p_state_enhaacplus_dec)
                        .mps_dec_handle
                        .ldmps_config
                        .ldmps_present_flag as WORD32,
                    (*p_state_enhaacplus_dec)
                        .mps_dec_handle
                        .ldmps_config
                        .no_ldsbr_present,
                );
            }
            if !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null() {
                (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                    .xaac_jmp_buf = &mut (*p_state_enhaacplus_dec).xaac_jmp_buf;
            }
            if sample_rate < sample_rate_1 {
                sample_rate = sample_rate_1;
            }
            ch_idx += 1;
            if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                >= ER_OBJECT_START as core::ffi::c_uint
                && ((*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
                    || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                        == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
                    || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                        == AOT_ER_AAC_LC as core::ffi::c_int as core::ffi::c_uint)
            {
                break;
            }
        }
        let mut ptr_adts_crc_info: *mut ia_adts_crc_info_struct = (*(*p_state_enhaacplus_dec)
            .ptr_bit_stream)
            .pstr_adts_crc_info;
        if (*ptr_adts_crc_info).crc_active as core::ffi::c_int == 1 as core::ffi::c_int {
            error_code = ixheaacd_adts_crc_check_crc(ptr_adts_crc_info);
            if error_code != 0 {
                return error_code;
            }
        }
        let mut temp: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
        let mut prev_persistent_used_t: WORD = 0;
        let mut prev_sbrpersistent_used_t: WORD = 0;
        let mut ps_enable: WORD = 0;
        let mut ch_idx_err: WORD = 0 as WORD;
        let mut persistent_used_t: WORD = 0 as WORD;
        let mut channel_check: WORD = 0 as WORD;
        let mut cc_channel_check: WORD = 0 as WORD;
        let mut max_ch_num: WORD = (*p_obj_exhaacplus_dec).aac_config.ui_max_channels
            as WORD;
        let mut harmonic_sbr_flag_1: WORD32 = 0 as WORD32;
        i = 0 as core::ffi::c_int as WORD;
        (*p_obj_exhaacplus_dec).aac_config.ui_n_channels = ch_idx as UWORD32;
        while (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx_err as usize]
            <= 3 as core::ffi::c_int
            && (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx_err as usize]
                >= 0 as core::ffi::c_int
        {
            ch_idx_err += 1;
        }
        if ch_idx_err == 0 as core::ffi::c_int {
            (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done = 0 as UWORD32;
            (*p_state_enhaacplus_dec).i_bytes_consumed = ((*it_bit_buff).ptr_read_next)
                .offset_from((*it_bit_buff).ptr_bit_buf_base) as core::ffi::c_long
                as WORD32;
            return IA_XHEAAC_DEC_INIT_NONFATAL_DECODE_FRAME_ERROR;
        }
        if ch_idx_err == 1 as core::ffi::c_int {
            ps_enable = 1 as core::ffi::c_int as WORD;
        } else {
            ps_enable = 0 as core::ffi::c_int as WORD;
        }
        while (*p_obj_exhaacplus_dec).aac_config.element_type[i as usize]
            >= 0 as core::ffi::c_int
            && (*p_obj_exhaacplus_dec).aac_config.element_type[i as usize]
                <= 3 as core::ffi::c_int
        {
            let mut channel: WORD32 = 0 as WORD32;
            match (*p_obj_exhaacplus_dec).aac_config.element_type[i as usize] {
                0 | 3 => {
                    channel = 1 as core::ffi::c_int as WORD32;
                }
                1 => {
                    channel = 2 as core::ffi::c_int as WORD32;
                }
                2 => {
                    if max_ch_num > 2 as core::ffi::c_int {
                        if (*p_obj_exhaacplus_dec)
                            .aac_config
                            .element_instance_order[i as usize]
                            != (*p_obj_exhaacplus_dec).aac_config.ui_coupling_channel
                        {
                            i += 1;
                            continue;
                        } else {
                            channel = 1 as core::ffi::c_int as WORD32;
                            cc_channel_check += 1;
                        }
                    } else {
                        i += 1;
                        continue;
                    }
                }
                _ => return -(1 as WORD32),
            }
            if cc_channel_check > MAX_CC_CHANNEL_NUM {
                return IA_XHEAAC_DEC_EXE_FATAL_UNIMPLEMENTED_CCE as WORD32;
            }
            if ps_enable == 1 as core::ffi::c_int {
                channel = 2 as core::ffi::c_int as WORD32;
            }
            if (*p_obj_exhaacplus_dec).aac_config.element_type[i as usize]
                != 2 as core::ffi::c_int
            {
                channel_check += channel as core::ffi::c_int;
            }
            if channel_check > max_ch_num {
                (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                    as WORD32;
                return IA_XHEAAC_DEC_INIT_FATAL_STREAM_CHAN_GT_MAX as WORD32;
            }
            temp = (*p_state_enhaacplus_dec).aac_persistent_mem_v;
            prev_persistent_used_t = persistent_used_t;
            ixheaacd_allocate_mem_persistent(
                p_obj_exhaacplus_dec,
                p_state_enhaacplus_dec,
                channel as WORD,
                &mut persistent_used_t,
                &mut prev_sbrpersistent_used_t,
                ps_enable,
            );
            (*p_state_enhaacplus_dec).aac_persistent_mem_v = temp;
            (*p_state_enhaacplus_dec).last_frame_ok = 1 as core::ffi::c_int as WORD32;
            (*p_state_enhaacplus_dec).num_channel_last = 0 as WORD16;
            (*p_state_enhaacplus_dec).ui_init_done = 0 as UWORD32;
            (*p_state_enhaacplus_dec).ui_input_over = 0 as UWORD32;
            (*p_state_enhaacplus_dec).ptr_bit_stream = (*p_state_enhaacplus_dec)
                .pstr_bit_buf as *mut ia_bit_buf_struct;
            (*p_state_enhaacplus_dec).pstr_aac_dec_info[i as usize] = 0
                as *mut ia_aac_decoder_struct;
            (*p_state_enhaacplus_dec).pstr_aac_dec_info[i as usize] = ixheaacd_aac_decoder_init(
                p_state_enhaacplus_dec,
                (*((*p_state_enhaacplus_dec).pstr_stream_sbr).offset(i as isize))
                    .as_mut_ptr(),
                channel as WORD,
                ((*p_state_enhaacplus_dec).aac_persistent_mem_v as *mut WORD8)
                    .offset(prev_persistent_used_t as isize) as *mut core::ffi::c_void,
                (*p_state_enhaacplus_dec).frame_length,
            );
            if ((*p_state_enhaacplus_dec).pstr_aac_dec_info[i as usize]).is_null() {
                (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                    as WORD32;
                return IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL as WORD32;
            }
            (*p_state_enhaacplus_dec).str_sbr_dec_info[i as usize] = ixheaacd_init_sbr(
                sample_rate_2,
                frame_size_2 as WORD32,
                &mut (*p_obj_exhaacplus_dec).aac_config.down_sample_flag as *mut UWORD32
                    as *mut FLAG,
                (*p_state_enhaacplus_dec).sbr_persistent_mem_v,
                (*p_state_enhaacplus_dec).ptr_overlap_buf,
                channel as WORD,
                ps_enable,
                1 as WORD,
                frame_size_2 as WORD * 2 as WORD,
                &mut harmonic_sbr_flag_1,
                NULL as *mut core::ffi::c_void,
                (*p_state_enhaacplus_dec).str_sbr_config,
                (*p_state_enhaacplus_dec).audio_object_type as WORD,
                (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.ldmps_present_flag
                    as WORD32,
                (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.no_ldsbr_present,
            );
            if !((*p_state_enhaacplus_dec).str_sbr_dec_info[i as usize]).is_null() {
                (*(*p_state_enhaacplus_dec).str_sbr_dec_info[i as usize]).xaac_jmp_buf = &mut (*p_state_enhaacplus_dec)
                    .xaac_jmp_buf;
            }
            if sbr_present_flag != 0
                && ((*(*p_obj_exhaacplus_dec).p_state_aac).audio_object_type
                    as core::ffi::c_uint
                    == AOT_AAC_LC as core::ffi::c_int as core::ffi::c_uint
                    || (*(*p_obj_exhaacplus_dec).p_state_aac).audio_object_type
                        as core::ffi::c_uint
                        == AOT_SBR as core::ffi::c_int as core::ffi::c_uint
                    || (*(*p_obj_exhaacplus_dec).p_state_aac).audio_object_type
                        as core::ffi::c_uint
                        == AOT_PS as core::ffi::c_int as core::ffi::c_uint)
                || (*p_state_enhaacplus_dec)
                    .mps_dec_handle
                    .ldmps_config
                    .ldmps_present_flag == 1 as UINT32
                    && (*(*p_obj_exhaacplus_dec).p_state_aac).audio_object_type
                        as core::ffi::c_uint
                        == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
            {
                (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo = 1 as UWORD32;
            }
            copy_qmf_to_ldmps(
                &mut (*(*p_obj_exhaacplus_dec).p_state_aac).mps_dec_handle,
                (*p_state_enhaacplus_dec).sbr_persistent_mem_v,
            );
            if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                == AOT_AAC_LC as core::ffi::c_int as core::ffi::c_uint
                && (*p_state_enhaacplus_dec).ui_mps_out_bytes != 0 as core::ffi::c_int
            {
                (*p_state_enhaacplus_dec).heaac_mps_handle.heaac_mps_present = 1
                    as core::ffi::c_int as WORD32;
                if (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                    .offset(
                        0 as core::ffi::c_int as isize,
                    ))[0 as core::ffi::c_int as usize]
                    .no_elements != 0
                {
                    (*(*p_obj_exhaacplus_dec).p_state_aac)
                        .heaac_mps_handle
                        .mps_with_sbr = 1 as core::ffi::c_int as WORD32;
                }
                error_code = ixheaacd_aac_mps_init(
                    p_obj_exhaacplus_dec,
                    mps_buffer,
                    (*p_state_enhaacplus_dec).ui_mps_out_bytes,
                    sample_rate_1,
                ) as WORD32;
                if error_code != 0 {
                    return error_code;
                }
                (*p_obj_exhaacplus_dec).aac_config.ui_n_channels = (*p_state_enhaacplus_dec)
                    .heaac_mps_handle
                    .num_output_channels_at as UWORD32;
                if (*(*p_obj_exhaacplus_dec).p_state_aac).heaac_mps_handle.mps_with_sbr
                    == 1 as core::ffi::c_int
                {
                    (*p_obj_exhaacplus_dec).aac_config.ui_sbr_mode = 1 as UWORD32;
                }
                if (*p_obj_exhaacplus_dec)
                    .aac_config
                    .element_type[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                    as usize] >= 0 as core::ffi::c_int
                    && (*p_obj_exhaacplus_dec)
                        .aac_config
                        .element_type[(i as core::ffi::c_int + 1 as core::ffi::c_int)
                        as usize] <= 3 as core::ffi::c_int
                {
                    return IA_FATAL_ERROR as WORD32;
                }
                break;
            } else {
                i += 1;
            }
        }
        (*p_state_enhaacplus_dec).pers_mem_ptr = ((*p_state_enhaacplus_dec)
            .aac_persistent_mem_v as *mut WORD8)
            .offset(persistent_used_t as isize);
        (*p_obj_exhaacplus_dec).aac_config.i_channel_mask = ixheaacd_get_channel_mask(
            p_obj_exhaacplus_dec,
        ) as WORD32;
        num_channels_1 = 0 as WORD16;
        ch_idx = 0 as core::ffi::c_int as WORD;
        while (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
            >= 0 as core::ffi::c_int
            && (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                <= 3 as core::ffi::c_int
        {
            if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                == 0 as core::ffi::c_int
                || (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                    == 3 as core::ffi::c_int
            {
                num_channels_1 = (num_channels_1 as core::ffi::c_int
                    + 1 as core::ffi::c_int) as WORD16;
            }
            if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                == 1 as core::ffi::c_int
            {
                num_channels_1 = (num_channels_1 as core::ffi::c_int
                    + 2 as core::ffi::c_int) as WORD16;
            }
            ch_idx += 1;
        }
        if ch_idx == 2 as core::ffi::c_int
            && num_channels_1 as core::ffi::c_int == 2 as core::ffi::c_int
        {
            (*p_obj_exhaacplus_dec).aac_config.ui_channel_mode = 2 as UWORD32;
        }
        if ch_idx == 1 as core::ffi::c_int {
            if num_channels_1 as core::ffi::c_int == 1 as core::ffi::c_int {
                (*p_obj_exhaacplus_dec).aac_config.ui_channel_mode = 0 as UWORD32;
            }
            if num_channels_1 as core::ffi::c_int == 2 as core::ffi::c_int {
                (*p_obj_exhaacplus_dec).aac_config.ui_channel_mode = 1 as UWORD32;
            }
        }
        if ps_detected == 1 as core::ffi::c_int
            && num_channels_1 as core::ffi::c_int == 1 as core::ffi::c_int
        {
            num_channels_1 = 2 as WORD16;
        }
        if 1 as core::ffi::c_int == (*p_obj_exhaacplus_dec).aac_config.downmix {
            num_channels_1 = 2 as WORD16;
        }
        if (*p_obj_exhaacplus_dec).aac_config.flag_downmix == 1 as UWORD32 {
            num_channels_1 = 1 as WORD16;
        }
        if (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo == 1 as UWORD32
            && (ch_idx == 1 as core::ffi::c_int
                || num_channels_1 as core::ffi::c_int <= 2 as core::ffi::c_int)
        {
            num_channels_1 = 2 as WORD16;
        }
        (*p_obj_exhaacplus_dec).aac_config.ui_n_channels = num_channels_1 as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_samp_freq = sample_rate as UWORD32;
        (*p_state_enhaacplus_dec).ui_init_done = 1 as UWORD32;
        memcpy(
            it_bit_buff as *mut core::ffi::c_void,
            &mut temp_bit_buff as *mut ia_bit_buf_struct as *const core::ffi::c_void,
            ::core::mem::size_of::<ia_bit_buf_struct>() as size_t,
        );
        (*p_state_enhaacplus_dec).b_n_raw_data_blk = 0 as WORD16;
        if (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done == 1 as UWORD32 {
            (*(*p_obj_exhaacplus_dec).p_state_aac).header_dec_done = 0 as UWORD32;
        }
    }
    return err_code as WORD32;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fill_slot_order(
    mut p_state_enhaacplus_dec: *mut ia_aac_dec_state_struct,
    mut ch: WORD32,
    mut ptr_is_cpe: *mut WORD8,
    mut ptr_tag_select: *mut WORD8,
    mut ptr_idx_no: *mut WORD32,
) -> VOID {
    let mut i: WORD32 = 0;
    let mut idx_no: WORD32 = *ptr_idx_no;
    let mut p_slot_element: *mut WORD = ((*(*p_state_enhaacplus_dec).p_config)
        .slot_element)
        .as_mut_ptr() as *mut WORD;
    let mut p_element_type: *mut WORD = ((*(*p_state_enhaacplus_dec).p_config)
        .element_type)
        .as_mut_ptr() as *mut WORD;
    let mut p_element_instance_order: *mut WORD = ((*(*p_state_enhaacplus_dec).p_config)
        .element_instance_order)
        .as_mut_ptr();
    i = 0 as core::ffi::c_int as WORD32;
    while i < ch {
        if *ptr_is_cpe.offset(i as isize) as core::ffi::c_int == 0 as core::ffi::c_int {
            let fresh1 = idx_no;
            idx_no = idx_no + 1;
            let fresh2 = p_slot_element;
            p_slot_element = p_slot_element.offset(1);
            *fresh2 = fresh1 as WORD;
            let fresh3 = p_element_type;
            p_element_type = p_element_type.offset(1);
            *fresh3 = 0 as core::ffi::c_int as WORD;
            let fresh4 = p_element_instance_order;
            p_element_instance_order = p_element_instance_order.offset(1);
            *fresh4 = *ptr_tag_select.offset(i as isize) as WORD;
        }
        i += 1;
    }
    *ptr_idx_no = idx_no;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_fill_prog_config_slots(
    mut p_state_enhaacplus_dec: *mut ia_aac_dec_state_struct,
) -> VOID {
    let mut idx_no: WORD32 = 0 as WORD32;
    ixheaacd_fill_slot_order(
        p_state_enhaacplus_dec,
        (*(*p_state_enhaacplus_dec).p_config).str_prog_config.num_front_channel_elements,
        ((*(*p_state_enhaacplus_dec).p_config).str_prog_config.front_element_is_cpe)
            .as_mut_ptr(),
        ((*(*p_state_enhaacplus_dec).p_config).str_prog_config.front_element_tag_select)
            .as_mut_ptr(),
        &mut idx_no,
    );
    ixheaacd_fill_slot_order(
        p_state_enhaacplus_dec,
        (*(*p_state_enhaacplus_dec).p_config).str_prog_config.num_side_channel_elements,
        ((*(*p_state_enhaacplus_dec).p_config).str_prog_config.side_element_is_cpe)
            .as_mut_ptr(),
        ((*(*p_state_enhaacplus_dec).p_config).str_prog_config.side_element_tag_select)
            .as_mut_ptr(),
        &mut idx_no,
    );
    ixheaacd_fill_slot_order(
        p_state_enhaacplus_dec,
        (*(*p_state_enhaacplus_dec).p_config).str_prog_config.num_back_channel_elements,
        ((*(*p_state_enhaacplus_dec).p_config).str_prog_config.back_element_is_cpe)
            .as_mut_ptr(),
        ((*(*p_state_enhaacplus_dec).p_config).str_prog_config.back_element_tag_select)
            .as_mut_ptr(),
        &mut idx_no,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_execute(
    mut p_obj_exhaacplus_dec: *mut ia_exhaacplus_dec_api_struct,
) -> WORD32 {
    let mut adts: ia_adts_header_struct = {
        let mut init = ia_adts_header_struct {
            sync_word: 0 as WORD16,
            id: 0,
            layer: 0,
            protection_absent: 0,
            profile: 0,
            samp_freq_index: 0,
            channel_configuration: 0,
            aac_frame_length: 0,
            no_raw_data_blocks: 0,
            crc_check: 0,
        };
        init
    };
    let mut p_state_enhaacplus_dec: *mut ia_aac_dec_state_struct = 0
        as *mut ia_aac_dec_state_struct;
    let mut in_buffer: *mut UWORD8 = 0 as *mut UWORD8;
    let mut time_data: *mut WORD16 = 0 as *mut WORD16;
    let mut num_of_out_samples: WORD16 = 0 as WORD16;
    let mut frame_size: WORD16 = 0 as WORD16;
    let mut sample_rate_dec: WORD32 = 0 as WORD32;
    let mut sample_rate: WORD32 = 0 as WORD32;
    let mut num_ch: WORD16 = 0 as WORD16;
    let mut it_bit_buff: *mut ia_bit_buf_struct = 0 as *mut ia_bit_buf_struct;
    let mut mps_buffer: *mut UWORD8 = 0 as *mut UWORD8;
    let mut error_code: WORD32 = IA_NO_ERROR;
    let mut ch_idx1: WORD = 0;
    let mut type_0: WORD = 0;
    let mut total_channels: WORD = 0 as WORD;
    let mut total_elements: WORD = 0 as WORD;
    let mut actual_out_buffer: *mut WORD16 = 0 as *mut WORD16;
    let mut ps_enable: WORD = 0;
    let mut esbr_mono_downmix: WORD = 0 as WORD;
    let mut element_used: [WORD8; 10] = [0; 10];
    let mut channel_coupling_flag: WORD32 = 0 as WORD32;
    let mut bytes_for_sync: size_t = 0;
    let mut audio_mux_length_bytes_last: WORD32 = 0 as WORD32;
    let mut ret_val: WORD32 = 0;
    let mut mps_out_samples: WORD32 = 0;
    (*p_obj_exhaacplus_dec).aac_config.ui_sbr_mode = 0 as UWORD32;
    (*p_obj_exhaacplus_dec).aac_config.frame_status = 1 as core::ffi::c_int as WORD32;
    if !((*p_obj_exhaacplus_dec).p_state_aac).is_null() {
        ret_val = _setjmp(
            ((*(*p_obj_exhaacplus_dec).p_state_aac).xaac_jmp_buf).as_mut_ptr(),
        ) as WORD32;
        if ret_val != 0 as core::ffi::c_int {
            (*(*p_obj_exhaacplus_dec).p_state_aac).i_bytes_consumed = (*(*p_obj_exhaacplus_dec)
                .p_state_aac)
                .ui_in_bytes as WORD32;
            (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = 0 as UWORD32;
            (*p_obj_exhaacplus_dec).aac_config.frame_status = 0 as core::ffi::c_int
                as WORD32;
            return IA_NO_ERROR;
        }
    }
    time_data = *((*p_obj_exhaacplus_dec).pp_mem_aac)
        .offset(IA_ENHAACPLUS_DEC_OUTPUT_IDX as isize) as *mut WORD16;
    in_buffer = *((*p_obj_exhaacplus_dec).pp_mem_aac)
        .offset(IA_ENHAACPLUS_DEC_INPUT_IDX as isize) as *mut UWORD8;
    p_state_enhaacplus_dec = (*p_obj_exhaacplus_dec).p_state_aac;
    (*p_state_enhaacplus_dec).aac_scratch_mem_v = *((*p_obj_exhaacplus_dec).pp_mem_aac)
        .offset(IA_ENHAACPLUS_DEC_SCRATCH_IDX as isize) as *mut core::ffi::c_void;
    (*p_state_enhaacplus_dec).mps_header = -(1 as core::ffi::c_int) as WORD32;
    mps_buffer = ((*p_state_enhaacplus_dec).mps_buffer).as_mut_ptr();
    it_bit_buff = (*p_state_enhaacplus_dec).pstr_bit_buf;
    ch_idx1 = 0 as core::ffi::c_int as WORD;
    (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int as WORD32;
    if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
        == AOT_USAC as core::ffi::c_int as core::ffi::c_uint
    {
        let mut pcm_size: WORD32 = (*p_obj_exhaacplus_dec).aac_config.ui_pcm_wdsz
            as WORD32;
        let mut inbuffer: *mut WORD8 = *((*p_obj_exhaacplus_dec).pp_mem_aac)
            .offset(IA_ENHAACPLUS_DEC_INPUT_IDX as isize) as *mut WORD8;
        let mut outbuffer: *mut WORD8 = *((*p_obj_exhaacplus_dec).pp_mem_aac)
            .offset(IA_ENHAACPLUS_DEC_OUTPUT_IDX as isize) as *mut WORD8;
        let mut out_bytes: WORD32 = 0 as WORD32;
        let mut frames_done: WORD32 = (*(*p_obj_exhaacplus_dec).p_state_aac)
            .frame_counter;
        let mut pstr_dec_data: *mut ia_dec_data_struct = (*(*p_obj_exhaacplus_dec)
            .p_state_aac)
            .pstr_dec_data as *mut ia_dec_data_struct;
        if (*pstr_dec_data).str_usac_data.down_samp_sbr != 0 as core::ffi::c_int {
            return IA_FATAL_ERROR as WORD32;
        }
        if (*(*p_obj_exhaacplus_dec).p_state_aac).ui_input_over == 0 as UWORD32 {
            let mut ptr_audio_specific_config: *mut ia_audio_specific_config_struct = (*(*p_obj_exhaacplus_dec)
                .p_state_aac)
                .ia_audio_specific_config as *mut ia_audio_specific_config_struct;
            (*ptr_audio_specific_config)
                .str_usac_config
                .str_usac_dec_config
                .preroll_counter = 0 as core::ffi::c_int as WORD32;
            let mut iii: WORD32 = 0 as WORD32;
            iii = 0 as core::ffi::c_int as WORD32;
            while iii < MAX_AUDIO_PREROLLS + 1 as core::ffi::c_int {
                (*((*(*p_obj_exhaacplus_dec).p_state_aac).pstr_dec_data
                    as *mut ia_dec_data_struct))
                    .str_frame_data
                    .str_audio_specific_config
                    .str_usac_config
                    .str_usac_dec_config
                    .usac_ext_gain_payload_len[iii as usize] = 0 as core::ffi::c_int
                    as WORD32;
                (*ptr_audio_specific_config)
                    .str_usac_config
                    .str_usac_dec_config
                    .usac_ext_gain_payload_len[iii as usize] = 0 as core::ffi::c_int
                    as WORD32;
                (*ptr_audio_specific_config)
                    .str_usac_config
                    .str_usac_dec_config
                    .preroll_bytes[iii as usize] = 0 as UWORD32;
                iii += 1;
            }
            (*((*(*p_obj_exhaacplus_dec).p_state_aac).pstr_dec_data
                as *mut ia_dec_data_struct))
                .str_frame_data
                .str_audio_specific_config
                .str_usac_config
                .str_usac_dec_config
                .preroll_counter = 0 as core::ffi::c_int as WORD32;
            error_code = ixheaacd_dec_main(
                p_obj_exhaacplus_dec as *mut core::ffi::c_void,
                inbuffer,
                outbuffer,
                &mut out_bytes,
                frames_done,
                pcm_size,
                &mut (*(*p_obj_exhaacplus_dec).p_state_aac).num_of_output_ch,
            );
            if error_code != 0 {
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return error_code
                }
            }
            (*(*p_obj_exhaacplus_dec).p_state_aac).frame_counter += 1;
        } else {
            out_bytes = 0 as core::ffi::c_int as WORD32;
        }
        if (*pstr_dec_data).str_usac_data.ec_flag == 0 as core::ffi::c_int {
            if (*p_state_enhaacplus_dec).bs_format != LOAS_BSFORMAT {
                (*(*p_obj_exhaacplus_dec).p_state_aac).i_bytes_consumed = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ui_in_bytes as WORD32;
            }
        } else if (*p_state_enhaacplus_dec).bs_format != LOAS_BSFORMAT {
            if (*pstr_dec_data).str_usac_data.frame_ok == 0 as core::ffi::c_int {
                (*(*p_obj_exhaacplus_dec).p_state_aac).i_bytes_consumed = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .ui_in_bytes as WORD32;
                (*pstr_dec_data).dec_bit_buf.cnt_bits = 0 as core::ffi::c_int as WORD32;
            } else {
                let mut pstr_dec_data_0: *mut ia_dec_data_struct = (*(*p_obj_exhaacplus_dec)
                    .p_state_aac)
                    .pstr_dec_data as *mut ia_dec_data_struct;
                if (*pstr_dec_data_0).dec_bit_buf.cnt_bits as core::ffi::c_int
                    & 7 as core::ffi::c_int != 0
                {
                    (*pstr_dec_data_0).dec_bit_buf.cnt_bits
                        -= (*pstr_dec_data_0).dec_bit_buf.cnt_bits as core::ffi::c_int
                            & 7 as core::ffi::c_int;
                }
                if (*pstr_dec_data_0).dec_bit_buf.cnt_bits == 0 as core::ffi::c_int {
                    (*(*p_obj_exhaacplus_dec).p_state_aac).i_bytes_consumed = (*(*p_obj_exhaacplus_dec)
                        .p_state_aac)
                        .ui_in_bytes as WORD32;
                } else {
                    (*(*p_obj_exhaacplus_dec).p_state_aac).i_bytes_consumed = ((*(*p_obj_exhaacplus_dec)
                        .p_state_aac)
                        .ui_in_bytes)
                        .wrapping_sub(
                            ((*pstr_dec_data_0).dec_bit_buf.cnt_bits
                                >> 3 as core::ffi::c_int) as UWORD32,
                        ) as WORD32;
                }
            }
        }
        (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = out_bytes as UWORD32;
        (*p_obj_exhaacplus_dec).aac_config.ui_n_channels = (*(*p_obj_exhaacplus_dec)
            .p_state_aac)
            .num_of_output_ch as UWORD32;
        (*pstr_dec_data).str_usac_data.sbr_parse_err_flag = 0 as core::ffi::c_int
            as WORD32;
        return 0 as WORD32;
    }
    while (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx1 as usize]
        <= 3 as core::ffi::c_int
        && (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx1 as usize]
            >= 0 as core::ffi::c_int
    {
        if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx1 as usize]
            == 0 as core::ffi::c_int
            || (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx1 as usize]
                == 3 as core::ffi::c_int
        {
            total_channels += 1 as core::ffi::c_int;
            total_elements += 1 as core::ffi::c_int;
        }
        if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx1 as usize]
            == 1 as core::ffi::c_int
        {
            total_elements += 1 as core::ffi::c_int;
            total_channels += 2 as core::ffi::c_int;
        }
        if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx1 as usize]
            == 2 as core::ffi::c_int
        {
            total_elements += 1 as core::ffi::c_int;
        }
        ch_idx1 += 1;
        if !(ch_idx1 > MAX_BS_ELEMENT) {
            continue;
        }
        if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
            break;
        }
        return IA_FATAL_ERROR as WORD32;
    }
    if ch_idx1 != 1 as core::ffi::c_int {
        ps_enable = 0 as core::ffi::c_int as WORD;
        if (*p_obj_exhaacplus_dec).aac_config.ui_max_channels > 2 as UWORD32 {
            let mut scratch_pointer: WORD32 = 0;
            scratch_pointer = (if (if (if (((2 as core::ffi::c_int
                * 2 as core::ffi::c_int * 1024 as core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                ) > 194048 as usize
            {
                (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
            } else {
                194048 as usize
            })
                > (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            {
                (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            } else {
                (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            })
                > (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            {
                (if (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
                    > (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        ) > 194048 as usize
                    {
                        (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                            * 1024 as core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    )
                                    .wrapping_add(
                                        (2 as usize)
                                            .wrapping_mul(
                                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                    as usize)
                                                    .wrapping_add(
                                                        (::core::mem::size_of::<WORD32>() as usize)
                                                            .wrapping_sub(1 as usize),
                                                    )
                                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                            ),
                                    )
                                    .wrapping_add(
                                        (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                    } else {
                        194048 as usize
                    })
                {
                    (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        ) > 194048 as usize
                    {
                        (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                            * 1024 as core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    )
                                    .wrapping_add(
                                        (2 as usize)
                                            .wrapping_mul(
                                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                    as usize)
                                                    .wrapping_add(
                                                        (::core::mem::size_of::<WORD32>() as usize)
                                                            .wrapping_sub(1 as usize),
                                                    )
                                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                            ),
                                    )
                                    .wrapping_add(
                                        (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                    } else {
                        194048 as usize
                    })
                } else {
                    (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        ) > 194048 as usize
                    {
                        (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                            * 1024 as core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    )
                                    .wrapping_add(
                                        (2 as usize)
                                            .wrapping_mul(
                                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                    as usize)
                                                    .wrapping_add(
                                                        (::core::mem::size_of::<WORD32>() as usize)
                                                            .wrapping_sub(1 as usize),
                                                    )
                                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                            ),
                                    )
                                    .wrapping_add(
                                        (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                    } else {
                        194048 as usize
                    })
                })
            } else {
                (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            })
                .wrapping_sub(SCR_INTER_SCR_SIZE) as WORD32;
            (*p_state_enhaacplus_dec).coup_ch_output = ((*(*p_obj_exhaacplus_dec)
                .p_state_aac)
                .aac_scratch_mem_v as *mut WORD8)
                .offset(scratch_pointer as isize) as *mut WORD32;
        }
    } else {
        if total_channels < (*p_obj_exhaacplus_dec).aac_config.ui_n_channels as WORD {
            total_channels = (*p_obj_exhaacplus_dec).aac_config.ui_n_channels as WORD;
        }
        ps_enable = 1 as core::ffi::c_int as WORD;
    }
    (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = 0 as UWORD32;
    (*(*p_obj_exhaacplus_dec).p_state_aac).ui_mps_out_bytes = 0 as core::ffi::c_int
        as WORD32;
    if (*p_state_enhaacplus_dec).ui_in_bytes == 0 as UWORD32 {
        let mut i: UWORD32 = 0;
        let mut j: WORD32 = 0;
        if (*p_state_enhaacplus_dec).peak_lim_init as core::ffi::c_int
            == 1 as core::ffi::c_int
        {
            (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = (((*p_state_enhaacplus_dec)
                .peak_limiter
                .attack_time_samples)
                .wrapping_mul(total_channels as UWORD32) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize) as UWORD32;
            j = 0 as core::ffi::c_int as WORD32;
            while j < total_channels {
                i = 0 as UWORD32;
                while i
                    < ((*p_state_enhaacplus_dec).peak_limiter.attack_time_samples)
                        .wrapping_sub(
                            (*p_state_enhaacplus_dec).peak_limiter.delayed_input_index,
                        )
                {
                    *time_data
                        .offset((total_channels as UWORD32).wrapping_mul(i) as isize)
                        .offset(j as isize) = ixheaac_round16(
                        *((*p_state_enhaacplus_dec).peak_limiter.delayed_input)
                            .offset(
                                ((*p_state_enhaacplus_dec).peak_limiter.delayed_input_index)
                                    .wrapping_mul(total_channels as UWORD32) as isize,
                            )
                            .offset((total_channels as UWORD32).wrapping_mul(i) as isize)
                            .offset(j as isize) as WORD32,
                    );
                    i = i.wrapping_add(1);
                }
                j += 1;
            }
            j = 0 as core::ffi::c_int as WORD32;
            while j < total_channels {
                i = 0 as UWORD32;
                while i < (*p_state_enhaacplus_dec).peak_limiter.delayed_input_index {
                    *time_data
                        .offset(
                            ((*p_state_enhaacplus_dec).peak_limiter.attack_time_samples)
                                .wrapping_sub(
                                    (*p_state_enhaacplus_dec).peak_limiter.delayed_input_index,
                                )
                                .wrapping_mul(total_channels as UWORD32) as isize,
                        )
                        .offset((total_channels as UWORD32).wrapping_mul(i) as isize)
                        .offset(j as isize) = ixheaac_round16(
                        *((*p_state_enhaacplus_dec).peak_limiter.delayed_input)
                            .offset((total_channels as UWORD32).wrapping_mul(i) as isize)
                            .offset(j as isize) as WORD32,
                    );
                    i = i.wrapping_add(1);
                }
                j += 1;
            }
            if (*p_obj_exhaacplus_dec).aac_config.dup_stereo_flag != 0 {
                i = 0 as UWORD32;
                while i < (*p_state_enhaacplus_dec).peak_limiter.attack_time_samples {
                    *time_data
                        .offset(
                            (2 as UWORD32).wrapping_mul(i).wrapping_add(1 as UWORD32)
                                as isize,
                        ) = *time_data
                        .offset(
                            (2 as UWORD32).wrapping_mul(i).wrapping_add(0 as UWORD32)
                                as isize,
                        );
                    i = i.wrapping_add(1);
                }
            }
        } else {
            (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = 0 as UWORD32;
        }
        (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int as WORD32;
        return IA_NO_ERROR;
    }
    if ch_idx1 == 0 as core::ffi::c_int {
        (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int as WORD32;
        if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
            (*p_obj_exhaacplus_dec).aac_config.frame_status = 0 as core::ffi::c_int
                as WORD32;
        } else {
            return IA_XHEAAC_DEC_EXE_NONFATAL_DECODE_FRAME_ERROR
        }
    }
    if total_channels > (*p_obj_exhaacplus_dec).aac_config.ui_max_channels as WORD {
        (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int as WORD32;
        if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
            (*p_obj_exhaacplus_dec).aac_config.frame_status = 0 as core::ffi::c_int
                as WORD32;
        } else {
            return IA_XHEAAC_DEC_CONFIG_NONFATAL_INVALID_MAX_CHANNEL
        }
    }
    if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
        == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
        || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
            == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
    {
        if (*p_obj_exhaacplus_dec).aac_config.ui_mp4_flag != 0 {
            (*p_state_enhaacplus_dec).frame_size = (*p_state_enhaacplus_dec).ui_in_bytes
                as WORD32;
        }
    }
    if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
        == AOT_ER_AAC_LC as core::ffi::c_int as core::ffi::c_uint
    {
        if (*p_obj_exhaacplus_dec).aac_config.ui_mp4_flag != 0 {
            (*p_state_enhaacplus_dec).frame_size = 1024 as core::ffi::c_int as WORD32;
        }
    }
    ixheaacd_create_init_bit_buf(
        it_bit_buff as *mut ia_bit_buf_struct,
        in_buffer,
        (*p_state_enhaacplus_dec).ui_in_bytes as WORD32,
    );
    (*it_bit_buff).xaac_jmp_buf = &mut (*p_state_enhaacplus_dec).xaac_jmp_buf;
    (*it_bit_buff).adts_header_present = (*p_state_enhaacplus_dec).s_adts_hdr_present
        as WORD32;
    (*it_bit_buff).no_raw_data_blocks = (*p_state_enhaacplus_dec).b_n_raw_data_blk
        as WORD8;
    (*it_bit_buff).protection_absent = (*p_state_enhaacplus_dec).protection_absent;
    if (*p_state_enhaacplus_dec).s_adts_hdr_present != 0 {
        if (*p_state_enhaacplus_dec).b_n_raw_data_blk as core::ffi::c_int
            == 0 as core::ffi::c_int
        {
            let mut error: WORD32 = 0;
            error = ixheaacd_readifadts(p_state_enhaacplus_dec, it_bit_buff, &mut adts);
            if error != 0 {
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                    if adts.samp_freq_index > 11 as core::ffi::c_int {
                        adts.samp_freq_index = 11 as core::ffi::c_int as WORD32;
                    }
                } else {
                    return error
                }
            }
            if (*p_state_enhaacplus_dec).sampling_rate as WORD32
                != (*(*p_obj_exhaacplus_dec).aac_tables.pstr_huffmann_tables)
                    .str_sample_rate_info[adts.samp_freq_index as usize]
                    .sampling_frequency
            {
                (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int
                    as WORD32;
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return IA_XHEAAC_DEC_EXE_NONFATAL_CHANGED_ADTS_SF
                }
            }
        }
    }
    bytes_for_sync = (*it_bit_buff).ptr_read_next as size_t;
    if (*p_state_enhaacplus_dec).bs_format == LOAS_BSFORMAT {
        let mut result: WORD32 = 0;
        let mut audio_mux_len_bytes_last: WORD32 = 0;
        let mut cnt_bits: WORD32 = (*it_bit_buff).cnt_bits;
        let mut sync: WORD32 = ixheaacd_read_bits_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            11 as WORD,
        );
        let mut curr_samp_rate: UWORD32 = 0 as UWORD32;
        if (*p_state_enhaacplus_dec).latm_initialized != 0 {
            curr_samp_rate = (*p_state_enhaacplus_dec)
                .latm_struct_element
                .layer_info[0 as core::ffi::c_int
                    as usize][0 as core::ffi::c_int as usize]
                .asc
                .sampling_freq;
        }
        while sync != 0x2b7 as core::ffi::c_int {
            sync = (sync & 0x3ff as WORD32) << 1 as core::ffi::c_int
                | ixheaacd_read_bits_buf(
                    it_bit_buff as *mut ia_bit_buf_struct,
                    1 as WORD,
                );
            if (*it_bit_buff).cnt_bits < 13 as core::ffi::c_int {
                ixheaacd_read_bidirection(
                    it_bit_buff as *mut ia_bit_buf_struct,
                    -(11 as WORD32),
                );
                (*p_state_enhaacplus_dec).i_bytes_consumed = ((cnt_bits
                    as core::ffi::c_int - (*it_bit_buff).cnt_bits as core::ffi::c_int)
                    / 8 as core::ffi::c_int) as WORD32;
                if (*p_state_enhaacplus_dec).i_bytes_consumed == 0 as core::ffi::c_int {
                    (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                        as WORD32;
                }
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return 0x1000 as WORD32
                }
            }
        }
        (*it_bit_buff).audio_mux_align = ((*it_bit_buff).cnt_bits as core::ffi::c_int
            - 13 as core::ffi::c_int) as WORD32;
        audio_mux_len_bytes_last = ixheaacd_read_bits_buf(
            it_bit_buff as *mut ia_bit_buf_struct,
            13 as WORD,
        );
        audio_mux_length_bytes_last = audio_mux_len_bytes_last;
        bytes_for_sync = ((*it_bit_buff).ptr_read_next as size_t)
            .wrapping_sub(bytes_for_sync);
        if (*it_bit_buff).cnt_bits < audio_mux_len_bytes_last << 3 as core::ffi::c_int {
            ixheaacd_read_bidirection(
                it_bit_buff as *mut ia_bit_buf_struct,
                -(13 as WORD32 + 11 as WORD32),
            );
            (*p_state_enhaacplus_dec).i_bytes_consumed = ((cnt_bits as core::ffi::c_int
                - (*it_bit_buff).cnt_bits as core::ffi::c_int) / 8 as core::ffi::c_int)
                as WORD32;
            if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                (*p_obj_exhaacplus_dec).aac_config.frame_status = 0 as core::ffi::c_int
                    as WORD32;
            } else {
                return IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES
            }
        } else {
            ixheaacd_read_bidirection(
                it_bit_buff as *mut ia_bit_buf_struct,
                -(13 as WORD32),
            );
        }
        if sync == 0x2b7 as core::ffi::c_int {
            result = ixheaacd_latm_audio_mux_element(
                it_bit_buff,
                &mut (*p_state_enhaacplus_dec).latm_struct_element,
                p_state_enhaacplus_dec,
                &mut *((*(*p_obj_exhaacplus_dec).aac_tables.pstr_huffmann_tables)
                    .str_sample_rate_info)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize)
                    as *mut ia_sampling_rate_info_struct,
            );
            if result < 0 as core::ffi::c_int {
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return result
                }
            }
            if (*p_state_enhaacplus_dec).latm_initialized == 0 {
                (*p_state_enhaacplus_dec).sampling_rate = (*p_state_enhaacplus_dec)
                    .latm_struct_element
                    .layer_info[0 as core::ffi::c_int
                        as usize][0 as core::ffi::c_int as usize]
                    .asc
                    .sampling_freq;
                (*p_state_enhaacplus_dec).latm_initialized = 1 as core::ffi::c_int
                    as WORD32;
            } else if (*p_state_enhaacplus_dec).sampling_rate != curr_samp_rate {
                (*p_state_enhaacplus_dec).i_bytes_consumed = 0 as core::ffi::c_int
                    as WORD32;
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return IA_XHEAAC_DEC_EXE_NONFATAL_CHANGED_ADTS_SF
                }
            }
        }
    }
    if total_elements == 2 as core::ffi::c_int && total_channels == 2 as core::ffi::c_int
        && ((*(*p_state_enhaacplus_dec).p_config).ui_pce_found_in_hdr == 1 as UWORD32
            || (*(*p_state_enhaacplus_dec).p_config).ui_pce_found_in_hdr == 3 as UWORD32)
    {
        ixheaacd_fill_prog_config_slots(p_state_enhaacplus_dec);
    }
    memset(
        element_used.as_mut_ptr() as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<WORD8>() as size_t)
            .wrapping_mul(MAX_BS_ELEMENT as size_t),
    );
    if (*it_bit_buff).cnt_bits <= 0 as core::ffi::c_int {
        (*it_bit_buff).cnt_bits = -(1 as core::ffi::c_int) as WORD32;
        ixheaacd_updatebytesconsumed(p_state_enhaacplus_dec, it_bit_buff);
        if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
            (*p_obj_exhaacplus_dec).aac_config.frame_status = 0 as core::ffi::c_int
                as WORD32;
        } else {
            return IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES as WORD16
                as WORD32
        }
    }
    (*it_bit_buff).initial_cnt_bits = (*it_bit_buff).cnt_bits;
    if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
        == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
    {
        if (*p_state_enhaacplus_dec).s_adts_hdr_present != 0 {
            (*p_state_enhaacplus_dec).frame_size = adts.aac_frame_length as WORD32;
        }
    }
    if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
        == AOT_ER_AAC_LC as core::ffi::c_int as core::ffi::c_uint
    {
        if (*p_state_enhaacplus_dec).s_adts_hdr_present != 0 {
            (*p_state_enhaacplus_dec).frame_size = 1024 as core::ffi::c_int as WORD32;
        }
    }
    if !((*p_state_enhaacplus_dec).pstr_drc_dec).is_null() {
        (*(*p_state_enhaacplus_dec).pstr_drc_dec).num_drc_elements = 0 as UWORD8;
        (*(*p_state_enhaacplus_dec).pstr_drc_dec).state = 1 as core::ffi::c_int
            as WORD32;
    }
    if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
        if total_elements > MAX_BS_ELEMENT {
            total_elements = MAX_BS_ELEMENT as WORD;
        }
    }
    let mut intermediate_scr: *mut WORD16 = ((*p_state_enhaacplus_dec).aac_scratch_mem_v
        as *mut WORD8 as *mut WORD16)
        .offset(
            (if (if (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                * 1024 as core::ffi::c_int) as usize)
                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                .wrapping_add((8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                .wrapping_add(
                    (2 as usize)
                        .wrapping_mul(
                            (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                as usize)
                                .wrapping_add(
                                    (::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                                )
                                & !(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_sub(1 as usize),
                        ),
                )
                .wrapping_add(
                    (1024 as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                )
                .wrapping_add(
                    (128 as usize)
                        .wrapping_mul(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ),
                ) > 194048 as usize
            {
                (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
            } else {
                194048 as usize
            })
                > (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            {
                (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            } else {
                (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            })
                > (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            {
                (if (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
                    > (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        ) > 194048 as usize
                    {
                        (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                            * 1024 as core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    )
                                    .wrapping_add(
                                        (2 as usize)
                                            .wrapping_mul(
                                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                    as usize)
                                                    .wrapping_add(
                                                        (::core::mem::size_of::<WORD32>() as usize)
                                                            .wrapping_sub(1 as usize),
                                                    )
                                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                            ),
                                    )
                                    .wrapping_add(
                                        (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                    } else {
                        194048 as usize
                    })
                {
                    (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        ) > 194048 as usize
                    {
                        (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                            * 1024 as core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    )
                                    .wrapping_add(
                                        (2 as usize)
                                            .wrapping_mul(
                                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                    as usize)
                                                    .wrapping_add(
                                                        (::core::mem::size_of::<WORD32>() as usize)
                                                            .wrapping_sub(1 as usize),
                                                    )
                                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                            ),
                                    )
                                    .wrapping_add(
                                        (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                    } else {
                        194048 as usize
                    })
                } else {
                    (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        ) > 194048 as usize
                    {
                        (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                            * 1024 as core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    )
                                    .wrapping_add(
                                        (2 as usize)
                                            .wrapping_mul(
                                                (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                    as usize)
                                                    .wrapping_add(
                                                        (::core::mem::size_of::<WORD32>() as usize)
                                                            .wrapping_sub(1 as usize),
                                                    )
                                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                            ),
                                    )
                                    .wrapping_add(
                                        (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                            .wrapping_add(
                                (128 as usize)
                                    .wrapping_mul(
                                        (1024 as usize)
                                            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                            .wrapping_add(
                                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                            )
                                            & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    ),
                            )
                    } else {
                        194048 as usize
                    })
                })
            } else {
                (if (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                    * 1024 as core::ffi::c_int) as usize)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                    .wrapping_add(
                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            )
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                            as usize)
                                            .wrapping_add(
                                                (::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                            )
                                            & !(::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                    ),
                            )
                            .wrapping_add(
                                (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                    as usize)
                                    .wrapping_add(
                                        (::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                    )
                                    & !(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_sub(1 as usize),
                            ),
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (1024 as usize)
                            .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                            .wrapping_add(
                                (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                    )
                    .wrapping_add(
                        (2 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    )
                    .wrapping_add(
                        (128 as usize)
                            .wrapping_mul(
                                (1024 as usize)
                                    .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                    .wrapping_add(
                                        (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                    )
                                    & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                            ),
                    ) > 194048 as usize
                {
                    (((2 as core::ffi::c_int * 2 as core::ffi::c_int
                        * 1024 as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                        .wrapping_add(
                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        ) & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize)
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (::core::mem::size_of::<ia_aac_dec_channel_info_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                )
                                .wrapping_add(
                                    (2 as usize)
                                        .wrapping_mul(
                                            (::core::mem::size_of::<ia_aac_sfb_code_book_struct>()
                                                as usize)
                                                .wrapping_add(
                                                    (::core::mem::size_of::<WORD32>() as usize)
                                                        .wrapping_sub(1 as usize),
                                                )
                                                & !(::core::mem::size_of::<WORD32>() as usize)
                                                    .wrapping_sub(1 as usize),
                                        ),
                                )
                                .wrapping_add(
                                    (::core::mem::size_of::<ia_pns_stereo_data_struct>()
                                        as usize)
                                        .wrapping_add(
                                            (::core::mem::size_of::<WORD32>() as usize)
                                                .wrapping_sub(1 as usize),
                                        )
                                        & !(::core::mem::size_of::<WORD32>() as usize)
                                            .wrapping_sub(1 as usize),
                                ),
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (1024 as usize)
                                .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                .wrapping_add(
                                    (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                )
                                & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                        )
                        .wrapping_add(
                            (2 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD32>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                        .wrapping_add(
                            (128 as usize)
                                .wrapping_mul(
                                    (1024 as usize)
                                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                                        .wrapping_add(
                                            (8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                        )
                                        & !(8 as core::ffi::c_int - 1 as core::ffi::c_int) as usize,
                                ),
                        )
                } else {
                    194048 as usize
                })
            })
                .wrapping_sub(SCR_INTER_SCR_SIZE)
                .wrapping_sub(SCR_COUP_CH_OUT_SIZE) as isize,
        );
    ch_idx1 = 0 as core::ffi::c_int as WORD;
    while ch_idx1 < total_elements {
        let mut skip_full_decode: WORD32 = 0 as WORD32;
        let mut ch_idx: WORD32 = ch_idx1 as WORD32;
        let mut channel: WORD32 = 0 as WORD32;
        let mut ch_fac: WORD = 0;
        let mut slot_ele: WORD = 0;
        if ((*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint)
            < ER_OBJECT_START as core::ffi::c_uint
            || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                != AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
                && (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    != AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
                && (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    != AOT_ER_AAC_LC as core::ffi::c_int as core::ffi::c_uint
        {
            let mut local: jmp_buf = [__jmp_buf_tag {
                __jmpbuf: [0; 8],
                __mask_was_saved: 0,
                __saved_mask: __sigset_t { __val: [0; 16] },
            }; 1];
            if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal == 1 as core::ffi::c_int
            {
                ret_val = _setjmp(local.as_mut_ptr()) as WORD32;
            }
            if ret_val == 0 as core::ffi::c_int {
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal
                    == 1 as core::ffi::c_int
                {
                    (*(*(*p_obj_exhaacplus_dec).p_state_aac).ptr_bit_stream)
                        .xaac_jmp_buf = &mut local;
                }
                error_code = ixheaacd_get_element_index_tag(
                    p_obj_exhaacplus_dec,
                    ch_idx1,
                    &mut ch_idx,
                    &mut channel,
                    ((*p_obj_exhaacplus_dec).aac_config.element_instance_order)
                        .as_mut_ptr(),
                    total_elements,
                    element_used.as_mut_ptr(),
                    total_channels,
                    (*p_state_enhaacplus_dec).pstr_drc_dec,
                    &mut (*p_state_enhaacplus_dec).drc_dummy,
                    mps_buffer,
                    &mut (*p_state_enhaacplus_dec).mps_header,
                    &mut (*p_state_enhaacplus_dec).ui_mps_out_bytes,
                );
            }
            if error_code != 0 || ret_val != 0 {
                ixheaacd_updatebytesconsumed(p_state_enhaacplus_dec, it_bit_buff);
                if (*it_bit_buff).cnt_bits < 0 as core::ffi::c_int {
                    (*p_state_enhaacplus_dec).ui_out_bytes = 0 as UWORD32;
                    (*p_state_enhaacplus_dec).ui_mps_out_bytes = 0 as core::ffi::c_int
                        as WORD32;
                    (*p_state_enhaacplus_dec).b_n_raw_data_blk = 0 as WORD16;
                }
                (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                    as WORD32;
                (*p_state_enhaacplus_dec).b_n_raw_data_blk = 0 as WORD16;
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return error_code
                }
            }
            if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                == ID_CPE as core::ffi::c_int
            {
                if channel != 2 as core::ffi::c_int {
                    if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                        (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                            as core::ffi::c_int as WORD32;
                        channel = 2 as core::ffi::c_int as WORD32;
                    } else {
                        return IA_FATAL_ERROR as WORD32
                    }
                }
            } else if channel != 1 as core::ffi::c_int {
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                    channel = 1 as core::ffi::c_int as WORD32;
                } else {
                    return IA_FATAL_ERROR as WORD32
                }
            }
        } else if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
            == ID_SCE as core::ffi::c_int
        {
            channel = 1 as core::ffi::c_int as WORD32;
        } else {
            channel = 2 as core::ffi::c_int as WORD32;
        }
        if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0
            && (error_code != 0 || ret_val != 0)
        {
            if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                == 0 as core::ffi::c_int
                || (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                    == 3 as core::ffi::c_int
            {
                if channel > 1 as core::ffi::c_int {
                    channel = 1 as core::ffi::c_int as WORD32;
                }
            }
            if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                == 1 as core::ffi::c_int
            {
                if channel > 2 as core::ffi::c_int {
                    channel = 2 as core::ffi::c_int as WORD32;
                }
            }
            if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
                == 2 as core::ffi::c_int
            {
                if (*p_obj_exhaacplus_dec).aac_config.ui_max_channels > 2 as UWORD32 {
                    if !((*p_obj_exhaacplus_dec)
                        .aac_config
                        .element_instance_order[ch_idx as usize]
                        != (*p_obj_exhaacplus_dec).aac_config.ui_coupling_channel)
                    {
                        if channel > 1 as core::ffi::c_int {
                            channel = 1 as core::ffi::c_int as WORD32;
                        }
                    }
                }
            }
            if ps_enable == 1 as core::ffi::c_int {
                if channel > 2 as core::ffi::c_int {
                    channel = 2 as core::ffi::c_int as WORD32;
                }
            }
        }
        ch_fac = total_channels;
        slot_ele = (*p_obj_exhaacplus_dec).aac_config.slot_element[ch_idx as usize]
            as WORD;
        actual_out_buffer = time_data;
        if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
            == 2 as core::ffi::c_int
        {
            (*(*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize])
                .p_ind_channel_info = &mut (*p_state_enhaacplus_dec).ind_cc_info
                as *mut ia_enhaacplus_dec_ind_cc as *mut core::ffi::c_void;
            if (*p_obj_exhaacplus_dec).aac_config.element_instance_order[ch_idx as usize]
                != (*p_obj_exhaacplus_dec).aac_config.ui_coupling_channel
            {
                skip_full_decode = 1 as core::ffi::c_int as WORD32;
                ixheaacd_set_aac_persistent_buffers(
                    (*p_state_enhaacplus_dec).pers_mem_ptr as *mut core::ffi::c_void,
                    channel,
                );
                let mut aac_persistent_mem: *mut ia_aac_persistent_struct = (*p_state_enhaacplus_dec)
                    .pers_mem_ptr as *mut ia_aac_persistent_struct;
                (*aac_persistent_mem).str_aac_decoder.pstr_aac_tables = &mut (*p_obj_exhaacplus_dec)
                    .aac_tables;
                (*aac_persistent_mem).str_aac_decoder.pstr_common_tables = (*p_obj_exhaacplus_dec)
                    .common_tables;
                (*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize] = 0
                    as *mut ia_aac_decoder_struct;
                (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize] = 0
                    as ia_handle_sbr_dec_inst_struct;
                (*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize] = ixheaacd_aac_decoder_init(
                    p_state_enhaacplus_dec,
                    (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                        .offset(ch_idx as isize))
                        .as_mut_ptr(),
                    channel as WORD,
                    (*p_state_enhaacplus_dec).pers_mem_ptr as *mut core::ffi::c_void,
                    (*p_state_enhaacplus_dec).frame_length,
                );
                if ((*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize])
                    .is_null()
                {
                    (*p_state_enhaacplus_dec).i_bytes_consumed = 1 as core::ffi::c_int
                        as WORD32;
                    if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                        (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                            as core::ffi::c_int as WORD32;
                    } else {
                        return IA_XHEAAC_DEC_INIT_FATAL_DEC_INIT_FAIL as WORD32
                    }
                }
                (*(*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize])
                    .p_ind_channel_info = ((*p_state_enhaacplus_dec).aac_scratch_mem_v
                    as *mut WORD8)
                    .offset(
                        SCR_BASE_SCR_8K_SIZE
                            .wrapping_add(SCR_EXTRA_SCR_4K_0_SIZE)
                            .wrapping_add(SCR_EXTRA_SCR_4K_2_SIZE) as isize,
                    ) as *mut core::ffi::c_void;
            }
            if (*p_obj_exhaacplus_dec)
                .aac_config
                .element_type[1 as core::ffi::c_int as usize] < 3 as core::ffi::c_int
                && (*p_obj_exhaacplus_dec)
                    .aac_config
                    .element_type[1 as core::ffi::c_int as usize] > 0 as core::ffi::c_int
                && (*p_obj_exhaacplus_dec).aac_config.ui_max_channels > 2 as UWORD32
            {
                actual_out_buffer = (*p_state_enhaacplus_dec).coup_ch_output
                    as *mut core::ffi::c_void as *mut WORD16;
            }
            ch_fac = 1 as core::ffi::c_int as WORD;
            slot_ele = 0 as core::ffi::c_int as WORD;
        }
        type_0 = -(1 as core::ffi::c_int) as WORD;
        (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
            .offset(ch_idx as isize))[0 as core::ffi::c_int as usize]
            .no_elements = 0 as WORD16;
        let mut element_index_order1: [WORD; 10] = [0; 10];
        let mut aac_scratch_struct: ia_aac_dec_scratch_struct = ia_aac_dec_scratch_struct {
            base_scr_8k: 0 as *mut core::ffi::c_void,
            extra_scr_4k: [0 as *mut core::ffi::c_void; 4],
            in_data: 0 as *mut WORD32,
            out_data: 0 as *mut WORD32,
        };
        memset(
            &mut aac_scratch_struct as *mut ia_aac_dec_scratch_struct
                as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<ia_aac_dec_scratch_struct>() as size_t,
        );
        ixheaacd_allocate_aac_scr(
            &mut aac_scratch_struct,
            (*p_state_enhaacplus_dec).aac_scratch_mem_v,
            time_data as *mut core::ffi::c_void,
            channel as WORD,
            (*p_obj_exhaacplus_dec).aac_config.ui_max_channels as WORD,
            (*p_state_enhaacplus_dec).audio_object_type as WORD32,
        );
        if (*p_state_enhaacplus_dec).ch_config == 2 as UWORD32
            && channel == 1 as core::ffi::c_int
        {
            if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                (*p_obj_exhaacplus_dec).aac_config.frame_status = 0 as core::ffi::c_int
                    as WORD32;
            } else {
                return IA_XHEAAC_DEC_EXE_NONFATAL_DECODE_FRAME_ERROR
            }
        }
        error_code = ixheaacd_aacdec_decodeframe(
            p_obj_exhaacplus_dec,
            &mut aac_scratch_struct,
            actual_out_buffer as *mut core::ffi::c_void,
            (*p_obj_exhaacplus_dec).aac_config.frame_status as FLAG,
            &mut type_0,
            &mut ch_idx,
            0 as WORD,
            channel as WORD,
            element_index_order1.as_mut_ptr(),
            skip_full_decode as WORD,
            ch_fac,
            slot_ele,
            (*p_obj_exhaacplus_dec).aac_config.ui_max_channels as WORD,
            total_channels as WORD32,
            (*(*p_obj_exhaacplus_dec).p_state_aac).frame_length,
            (*(*p_obj_exhaacplus_dec).p_state_aac).frame_size,
            (*p_state_enhaacplus_dec).pstr_drc_dec,
            (*p_state_enhaacplus_dec).audio_object_type as WORD32,
            (*p_state_enhaacplus_dec).ch_config as WORD32,
            (*p_state_enhaacplus_dec).eld_specific_config,
            (*p_state_enhaacplus_dec).s_adts_hdr_present,
            &mut (*p_state_enhaacplus_dec).drc_dummy,
            (*p_state_enhaacplus_dec).ldmps_present,
            &mut (*p_state_enhaacplus_dec).slot_pos,
            mps_buffer,
            &mut (*p_state_enhaacplus_dec).mps_header,
            &mut (*p_state_enhaacplus_dec).ui_mps_out_bytes,
            0 as WORD32,
            (*p_obj_exhaacplus_dec).aac_config.first_frame as WORD32,
        );
        (*p_state_enhaacplus_dec).slot_pos = ((*p_state_enhaacplus_dec).slot_pos
            as core::ffi::c_int - (channel as core::ffi::c_int - 1 as core::ffi::c_int))
            as UWORD8;
        (*p_state_enhaacplus_dec).sbr_present = 0 as UWORD8;
        if (*(*p_obj_exhaacplus_dec).p_state_aac)
            .qshift_adj[0 as core::ffi::c_int as usize] as core::ffi::c_int != LD_OBJ
            && (*p_state_enhaacplus_dec).frame_counter == 0 as core::ffi::c_int
        {
            ixheaacd_peak_limiter_init(
                &mut (*p_state_enhaacplus_dec).peak_limiter,
                total_channels as UWORD32,
                (*(*(*p_obj_exhaacplus_dec).p_state_aac).p_config).ui_samp_freq,
                &mut *((*p_state_enhaacplus_dec).peak_limiter.buffer)
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                &mut (*(*p_obj_exhaacplus_dec).p_state_aac).delay_in_samples,
            );
            (*(*p_obj_exhaacplus_dec).p_state_aac).peak_lim_init = 1 as UWORD8;
        }
        if ((*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint)
            < ER_OBJECT_START as core::ffi::c_uint
            || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                != AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
                && (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    != AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
                && (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    != AOT_ER_AAC_LC as core::ffi::c_int as core::ffi::c_uint
        {
            if error_code == 0 as core::ffi::c_int
                && ch_idx1 as core::ffi::c_int + 1 as core::ffi::c_int == total_elements
                && type_0 != ID_END as core::ffi::c_int
            {
                (*p_state_enhaacplus_dec).i_bytes_consumed = ((*it_bit_buff)
                    .ptr_read_next)
                    .offset_from((*it_bit_buff).ptr_bit_buf_base) as core::ffi::c_long
                    as WORD32;
                (*p_state_enhaacplus_dec).b_n_raw_data_blk = 0 as WORD16;
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return IA_XHEAAC_DEC_EXE_NONFATAL_ELE_INSTANCE_TAG_NOT_FOUND
                }
            }
        }
        num_ch = (*(*p_state_enhaacplus_dec).pstr_aac_dec_info[ch_idx as usize])
            .channels;
        if skip_full_decode == 0 as core::ffi::c_int {
            if (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
                || (*p_state_enhaacplus_dec).audio_object_type as core::ffi::c_uint
                    == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint
            {
                frame_size = (*p_state_enhaacplus_dec).frame_length as WORD16;
            } else {
                frame_size = (*p_state_enhaacplus_dec).frame_length as WORD16;
            }
            sample_rate_dec = (*(*p_state_enhaacplus_dec)
                .pstr_aac_dec_info[ch_idx as usize])
                .sampling_rate;
        }
        if skip_full_decode == 1 as core::ffi::c_int {
            (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                .offset(ch_idx as isize))[0 as core::ffi::c_int as usize]
                .no_elements = 0 as WORD16;
        }
        if (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
            .offset(ch_idx as isize))[0 as core::ffi::c_int as usize]
            .no_elements as core::ffi::c_int != 0 as core::ffi::c_int
        {
            (*p_obj_exhaacplus_dec).aac_config.ui_sbr_mode = 1 as UWORD32;
        }
        if error_code != 0 {
            if (*p_state_enhaacplus_dec).ui_input_over != 0 {
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return IA_XHEAAC_DEC_INIT_FATAL_EO_INPUT_REACHED as WORD32
                }
            }
            ixheaacd_updatebytesconsumed(p_state_enhaacplus_dec, it_bit_buff);
            (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = ((*(*p_obj_exhaacplus_dec)
                .p_state_aac)
                .ui_out_bytes as core::ffi::c_ulong)
                .wrapping_add(
                    (((*p_state_enhaacplus_dec).num_of_out_samples as core::ffi::c_int
                        * num_ch as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        as core::ffi::c_ulong,
                ) as UWORD32 as UWORD32;
            if error_code != 0 {
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return error_code
                }
            }
        }
        error_code = IA_NO_ERROR as WORD32;
        if (*p_obj_exhaacplus_dec).aac_config.ui_auto_sbr_upsample == 0 as UWORD32 {
            if (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                .offset(ch_idx as isize))[0 as core::ffi::c_int as usize]
                .no_elements as core::ffi::c_int == 0 as core::ffi::c_int
                && !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                    .is_null()
            {
                (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize] = 0
                    as ia_handle_sbr_dec_inst_struct;
                error_code = IA_XHEAAC_DEC_EXE_NONFATAL_SBR_TURNED_OFF as WORD32;
            }
        }
        if ((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null()
            && (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                .offset(ch_idx as isize))[0 as core::ffi::c_int as usize]
                .no_elements as core::ffi::c_int != 0
        {
            let mut harmonic_sbr_flag: WORD32 = 0 as WORD32;
            error_code = IA_XHEAAC_DEC_EXE_NONFATAL_SBR_TURNED_ON as WORD32;
            (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize] = ixheaacd_init_sbr(
                sample_rate_dec,
                frame_size as WORD32,
                &mut (*p_obj_exhaacplus_dec).aac_config.down_sample_flag as *mut UWORD32
                    as *mut FLAG,
                (*p_state_enhaacplus_dec).sbr_persistent_mem_v,
                (*p_state_enhaacplus_dec).ptr_overlap_buf,
                if ps_enable != 0 { 2 as WORD } else { channel as WORD },
                ps_enable,
                1 as WORD,
                frame_size as WORD * 2 as WORD,
                &mut harmonic_sbr_flag,
                NULL as *mut core::ffi::c_void,
                (*p_state_enhaacplus_dec).str_sbr_config,
                (*p_state_enhaacplus_dec).audio_object_type as WORD,
                (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.ldmps_present_flag
                    as WORD32,
                (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.no_ldsbr_present,
            );
            if !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null() {
                (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                    .xaac_jmp_buf = &mut (*p_state_enhaacplus_dec).xaac_jmp_buf;
            }
        }
        if !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null()
            && (*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                .offset(0 as core::ffi::c_int as isize))[0 as core::ffi::c_int as usize]
                .no_elements as core::ffi::c_int != 0
        {
            let mut sbr_scratch_struct: ia_sbr_scr_struct = ia_sbr_scr_struct {
                ptr_work_buf_core: 0 as *mut core::ffi::c_void,
            };
            ixheaacd_allocate_sbr_scr(
                &mut sbr_scratch_struct,
                (*p_state_enhaacplus_dec).aac_scratch_mem_v,
                time_data as *mut core::ffi::c_void,
                total_channels as WORD32,
                ((*(*p_obj_exhaacplus_dec).p_state_aac).qshift_adj).as_mut_ptr(),
                (*p_state_enhaacplus_dec).slot_pos,
                channel as UWORD8,
            );
            (*p_state_enhaacplus_dec).sbr_present = 1 as UWORD8;
            (*p_state_enhaacplus_dec).peak_lim_init = 0 as UWORD8;
            if (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr != 0 {
                let mut audio_object_type: WORD32 = (*p_state_enhaacplus_dec)
                    .audio_object_type as WORD32;
                if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
                    && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int
                {
                    let mut i_0: WORD32 = 0 as WORD32;
                    let mut pstr_dec_data_1: *mut ia_dec_data_struct = (*p_state_enhaacplus_dec)
                        .pstr_dec_data as *mut ia_dec_data_struct;
                    if ch_fac == 1 as core::ffi::c_int {
                        while i_0 < 1024 as core::ffi::c_int {
                            (*pstr_dec_data_1)
                                .str_usac_data
                                .time_sample_vector[0 as core::ffi::c_int
                                as usize][i_0 as usize] = *time_data.offset(i_0 as isize)
                                as FLOAT32;
                            i_0 += 1;
                        }
                        (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                            .time_sample_buf[0 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data_1)
                            .str_usac_data
                            .time_sample_vector)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                        (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                            .time_sample_buf[1 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data_1)
                            .str_usac_data
                            .time_sample_vector)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                    } else if ch_fac == 2 as core::ffi::c_int {
                        while i_0 < 1024 as core::ffi::c_int {
                            (*pstr_dec_data_1)
                                .str_usac_data
                                .time_sample_vector[0 as core::ffi::c_int
                                as usize][i_0 as usize] = *time_data
                                .offset(
                                    (2 as core::ffi::c_int * i_0 as core::ffi::c_int
                                        + 0 as core::ffi::c_int) as isize,
                                ) as FLOAT32;
                            (*pstr_dec_data_1)
                                .str_usac_data
                                .time_sample_vector[1 as core::ffi::c_int
                                as usize][i_0 as usize] = *time_data
                                .offset(
                                    (2 as core::ffi::c_int * i_0 as core::ffi::c_int
                                        + 1 as core::ffi::c_int) as isize,
                                ) as FLOAT32;
                            i_0 += 1;
                        }
                        (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                            .time_sample_buf[0 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data_1)
                            .str_usac_data
                            .time_sample_vector)
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                        (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                            .time_sample_buf[1 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data_1)
                            .str_usac_data
                            .time_sample_vector)
                            .as_mut_ptr()
                            .offset(1 as core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                    } else if ch_fac > 2 as core::ffi::c_int {
                        if channel == 1 as core::ffi::c_int {
                            while i_0 < 1024 as core::ffi::c_int {
                                (*pstr_dec_data_1)
                                    .str_usac_data
                                    .time_sample_vector[0 as core::ffi::c_int
                                    as usize][i_0 as usize] = *time_data
                                    .offset(slot_ele as isize)
                                    .offset((i_0 as WORD * ch_fac) as isize) as FLOAT32;
                                i_0 += 1;
                            }
                            (*(*p_state_enhaacplus_dec)
                                .str_sbr_dec_info[ch_idx as usize])
                                .time_sample_buf[0 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data_1)
                                .str_usac_data
                                .time_sample_vector)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                        } else if channel == 2 as core::ffi::c_int {
                            while i_0 < 1024 as core::ffi::c_int {
                                (*pstr_dec_data_1)
                                    .str_usac_data
                                    .time_sample_vector[0 as core::ffi::c_int
                                    as usize][i_0 as usize] = *time_data
                                    .offset(slot_ele as isize)
                                    .offset(
                                        (ch_fac as core::ffi::c_int * i_0 as core::ffi::c_int
                                            + 0 as core::ffi::c_int) as isize,
                                    ) as FLOAT32;
                                (*pstr_dec_data_1)
                                    .str_usac_data
                                    .time_sample_vector[1 as core::ffi::c_int
                                    as usize][i_0 as usize] = *time_data
                                    .offset(slot_ele as isize)
                                    .offset(
                                        (ch_fac as core::ffi::c_int * i_0 as core::ffi::c_int
                                            + 1 as core::ffi::c_int) as isize,
                                    ) as FLOAT32;
                                i_0 += 1;
                            }
                            (*(*p_state_enhaacplus_dec)
                                .str_sbr_dec_info[ch_idx as usize])
                                .time_sample_buf[0 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data_1)
                                .str_usac_data
                                .time_sample_vector)
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                            (*(*p_state_enhaacplus_dec)
                                .str_sbr_dec_info[ch_idx as usize])
                                .time_sample_buf[1 as core::ffi::c_int as usize] = &mut *(*((*pstr_dec_data_1)
                                .str_usac_data
                                .time_sample_vector)
                                .as_mut_ptr()
                                .offset(1 as core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(0 as core::ffi::c_int as isize) as *mut FLOAT32;
                        }
                    }
                }
            }
            (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).esbr_hq = (*p_obj_exhaacplus_dec)
                .aac_config
                .ui_hq_esbr as FLAG;
            (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).enh_sbr = (*p_obj_exhaacplus_dec)
                .aac_config
                .ui_enh_sbr as FLAG;
            (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).enh_sbr_ps = (*p_obj_exhaacplus_dec)
                .aac_config
                .ui_enh_sbr_ps as FLAG;
            if !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null() {
                (*(*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize])
                    .xaac_jmp_buf = &mut (*p_state_enhaacplus_dec).xaac_jmp_buf;
            }
            if ixheaacd_applysbr(
                (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize],
                &mut *(*((*p_state_enhaacplus_dec).pstr_stream_sbr)
                    .offset(ch_idx as isize))
                    .as_mut_ptr()
                    .offset(0 as core::ffi::c_int as isize),
                actual_out_buffer,
                &mut num_ch,
                (*p_obj_exhaacplus_dec).aac_config.frame_status as FLAG,
                (*p_obj_exhaacplus_dec).aac_config.down_sample_flag as FLAG,
                esbr_mono_downmix as FLAG,
                &mut sbr_scratch_struct,
                ps_enable as WORD32,
                ch_fac as WORD32,
                slot_ele as WORD32,
                0 as *mut ia_bit_buf_struct,
                &mut (*p_state_enhaacplus_dec).str_drc_dec_info,
                (*p_state_enhaacplus_dec).eld_specific_config.ld_sbr_flag_present
                    as WORD,
                (*p_state_enhaacplus_dec).audio_object_type as WORD,
                0 as WORD32,
                (*p_state_enhaacplus_dec).ldmps_present,
                frame_size as WORD32,
                (*p_state_enhaacplus_dec).heaac_mps_handle.heaac_mps_present,
                (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal,
                (*p_obj_exhaacplus_dec).aac_config.first_frame,
            ) != SBRDEC_OK
            {
                (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize] = 0
                    as ia_handle_sbr_dec_inst_struct;
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return -(1 as WORD32)
                }
            } else {
                if (*p_obj_exhaacplus_dec).aac_config.down_sample_flag == 0 {
                    frame_size = (frame_size as core::ffi::c_int * 2 as core::ffi::c_int)
                        as WORD16;
                    sample_rate_dec *= 2 as core::ffi::c_int;
                }
                if (*p_state_enhaacplus_dec)
                    .mps_dec_handle
                    .ldmps_config
                    .ldmps_present_flag == 1 as UINT32
                {
                    ixheaacd_mps_payload(
                        (*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize],
                        p_obj_exhaacplus_dec,
                    );
                }
            }
            if (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr != 0 {
                let mut audio_object_type_0: WORD32 = (*p_state_enhaacplus_dec)
                    .audio_object_type as WORD32;
                if audio_object_type_0 != AOT_ER_AAC_ELD as core::ffi::c_int
                    && audio_object_type_0 != AOT_ER_AAC_LD as core::ffi::c_int
                {
                    let mut out_bytes_0: WORD32 = 0 as WORD32;
                    let mut pstr_dec_data_2: *mut ia_dec_data_struct = (*p_state_enhaacplus_dec)
                        .pstr_dec_data as *mut ia_dec_data_struct;
                    if ch_fac <= 2 as core::ffi::c_int {
                        ixheaacd_samples_sat(
                            time_data as *mut WORD8,
                            2048 as WORD32,
                            16 as WORD32,
                            ((*pstr_dec_data_2).str_usac_data.time_sample_vector)
                                .as_mut_ptr() as *mut [FLOAT32; 4096],
                            &mut out_bytes_0,
                            ch_fac as WORD32,
                        );
                    } else {
                        ixheaacd_samples_sat_mc(
                            time_data.offset(slot_ele as isize) as *mut WORD8,
                            2048 as WORD32,
                            ((*pstr_dec_data_2).str_usac_data.time_sample_vector)
                                .as_mut_ptr() as *mut [FLOAT32; 4096],
                            &mut out_bytes_0,
                            channel,
                            ch_fac as WORD32,
                        );
                    }
                }
            }
            (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.no_ldsbr_present = 0
                as core::ffi::c_int as WORD32;
            if (*p_state_enhaacplus_dec).ui_mps_out_bytes > 0 as core::ffi::c_int {
                (*p_state_enhaacplus_dec).heaac_mps_handle.heaac_mps_present = 1
                    as core::ffi::c_int as WORD32;
            }
        } else {
            (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.no_ldsbr_present = 1
                as core::ffi::c_int as WORD32;
        }
        if (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.ldmps_present_flag
            == 1 as UINT32
            && !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null()
            && (*p_state_enhaacplus_dec).mps_dec_handle.mps_init_done
                == 1 as core::ffi::c_int
        {
            if (*p_state_enhaacplus_dec).ec_enable != 0 {
                if (*p_obj_exhaacplus_dec).aac_config.first_frame == 0 {
                    error_code = ixheaacd_ld_mps_apply(
                        p_obj_exhaacplus_dec,
                        actual_out_buffer,
                    );
                    if error_code != 0 {
                        (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                            as core::ffi::c_int as WORD32;
                    }
                }
            } else {
                error_code = ixheaacd_ld_mps_apply(
                    p_obj_exhaacplus_dec,
                    actual_out_buffer,
                );
                if error_code != 0 {
                    return error_code;
                }
            }
        }
        if sample_rate < sample_rate_dec {
            sample_rate = sample_rate_dec;
        }
        if (*p_state_enhaacplus_dec).sbr_present as core::ffi::c_int != 0
            || (*(*p_obj_exhaacplus_dec).p_state_aac)
                .qshift_adj[0 as core::ffi::c_int as usize] as core::ffi::c_int == LD_OBJ
        {
            num_of_out_samples = frame_size;
        } else {
            num_of_out_samples = (frame_size as core::ffi::c_int
                - (if ((*(*p_obj_exhaacplus_dec).p_state_aac).delay_in_samples as WORD16
                    as core::ffi::c_int) < frame_size as core::ffi::c_int
                {
                    (*(*p_obj_exhaacplus_dec).p_state_aac).delay_in_samples as WORD16
                        as core::ffi::c_int
                } else {
                    frame_size as core::ffi::c_int
                })) as WORD16;
        }
        if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0
            && (*p_obj_exhaacplus_dec).aac_config.first_frame != 0
            && ((*(*p_obj_exhaacplus_dec).p_state_aac).audio_object_type
                as core::ffi::c_uint
                == AOT_ER_AAC_ELD as core::ffi::c_int as core::ffi::c_uint
                || (*(*p_obj_exhaacplus_dec).p_state_aac).audio_object_type
                    as core::ffi::c_uint
                    == AOT_ER_AAC_LD as core::ffi::c_int as core::ffi::c_uint)
        {
            num_of_out_samples = frame_size;
        }
        (*p_obj_exhaacplus_dec).aac_config.ui_samp_freq = sample_rate as UWORD32;
        (*p_state_enhaacplus_dec).num_channel_last = num_ch;
        (*p_state_enhaacplus_dec).num_of_out_samples = num_of_out_samples;
        if (*p_state_enhaacplus_dec).mps_dec_handle.ldmps_config.ldmps_present_flag
            == 1 as UINT32
            && (*p_state_enhaacplus_dec).mps_dec_handle.mps_init_done
                == 1 as core::ffi::c_int
            && !((*p_state_enhaacplus_dec).str_sbr_dec_info[ch_idx as usize]).is_null()
        {
            if !((*(*p_obj_exhaacplus_dec).p_state_aac).mps_dec_handle.output_buffer)
                .is_null()
            {
                ixheaacd_samples_sat(
                    actual_out_buffer as *mut WORD8,
                    num_of_out_samples as WORD32,
                    (*p_obj_exhaacplus_dec).aac_config.ui_pcm_wdsz as WORD32,
                    (*(*p_obj_exhaacplus_dec).p_state_aac).mps_dec_handle.output_buffer,
                    &mut mps_out_samples,
                    2 as WORD32,
                );
                (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = mps_out_samples
                    as UWORD32;
            }
            num_ch = (*(*p_obj_exhaacplus_dec).p_state_aac).mps_dec_handle.out_ch_count
                as WORD16;
            if (*p_state_enhaacplus_dec).ec_enable != 0 {
                if (*p_obj_exhaacplus_dec).aac_config.first_frame != 0 {
                    (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = (((*p_state_enhaacplus_dec)
                        .num_of_out_samples as core::ffi::c_int
                        * num_ch as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        as UWORD32;
                }
            }
        } else if (*p_obj_exhaacplus_dec).aac_config.element_type[ch_idx as usize]
            != 2 as core::ffi::c_int
        {
            if (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo == 1 as UWORD32
                && channel == 1 as core::ffi::c_int
                && total_elements == 1 as core::ffi::c_int
                && num_ch as core::ffi::c_int == 1 as core::ffi::c_int
            {
                num_ch = 2 as WORD16;
                (*p_obj_exhaacplus_dec).aac_config.dup_stereo_flag = 1 as UWORD8;
            } else {
                (*p_obj_exhaacplus_dec).aac_config.dup_stereo_flag = 0 as UWORD8;
            }
            (*p_obj_exhaacplus_dec).aac_config.ui_n_channels = 2 as UWORD32;
            (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = ((*(*p_obj_exhaacplus_dec)
                .p_state_aac)
                .ui_out_bytes as core::ffi::c_ulong)
                .wrapping_add(
                    (((*p_state_enhaacplus_dec).num_of_out_samples as core::ffi::c_int
                        * num_ch as core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<WORD16>() as usize)
                        as core::ffi::c_ulong,
                ) as UWORD32 as UWORD32;
        } else {
            channel_coupling_flag = 1 as core::ffi::c_int as WORD32;
        }
        if (*p_state_enhaacplus_dec).sbr_present as core::ffi::c_int != 0
            && total_channels > 2 as core::ffi::c_int
        {
            let mut j_0: WORD32 = 0 as WORD32;
            while j_0 < channel {
                let mut i_1: WORD32 = 0 as WORD32;
                while i_1 < frame_size as core::ffi::c_int {
                    *intermediate_scr
                        .offset(
                            (total_channels as core::ffi::c_int * i_1 as core::ffi::c_int
                                + j_0 as core::ffi::c_int
                                + (*p_state_enhaacplus_dec).slot_pos as core::ffi::c_int)
                                as isize,
                        ) = *actual_out_buffer
                        .offset(
                            (total_channels as core::ffi::c_int * i_1 as core::ffi::c_int
                                + j_0 as core::ffi::c_int
                                + (*p_state_enhaacplus_dec).slot_pos as core::ffi::c_int)
                                as isize,
                        );
                    i_1 += 1;
                }
                j_0 += 1;
            }
        }
        ch_idx1 += 1;
    }
    if (*p_state_enhaacplus_dec).sbr_present as core::ffi::c_int != 0
        && total_channels > 2 as core::ffi::c_int
    {
        memcpy(
            time_data as *mut core::ffi::c_void,
            intermediate_scr as *const core::ffi::c_void,
            (::core::mem::size_of::<WORD16>() as size_t)
                .wrapping_mul(frame_size as size_t)
                .wrapping_mul(total_channels as size_t),
        );
    }
    let mut ptr_adts_crc_info: *mut ia_adts_crc_info_struct = (*(*p_state_enhaacplus_dec)
        .ptr_bit_stream)
        .pstr_adts_crc_info;
    if (*ptr_adts_crc_info).crc_active as core::ffi::c_int == 1 as core::ffi::c_int {
        error_code = ixheaacd_adts_crc_check_crc(ptr_adts_crc_info);
        if error_code != 0 {
            if error_code != 0 {
                if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
                    (*p_obj_exhaacplus_dec).aac_config.frame_status = 0
                        as core::ffi::c_int as WORD32;
                } else {
                    return error_code
                }
            }
        }
    }
    (*p_obj_exhaacplus_dec).aac_config.ui_n_channels = total_channels as UWORD32;
    (*p_state_enhaacplus_dec).frame_counter += 1;
    let mut i_2: WORD32 = 0;
    let mut j_1: WORD32 = 0;
    if channel_coupling_flag != 0 {
        error_code = ixheaacd_dec_ind_coupling(
            p_obj_exhaacplus_dec,
            (*p_state_enhaacplus_dec).coup_ch_output,
            num_of_out_samples,
            total_channels,
            time_data as *mut core::ffi::c_void,
        ) as WORD32;
        if error_code != 0 {
            return error_code;
        }
    }
    i_2 = 0 as core::ffi::c_int as WORD32;
    while i_2 < total_channels {
        if (*(*p_obj_exhaacplus_dec).p_state_aac)
            .qshift_adj[(i_2 as core::ffi::c_int + 1 as core::ffi::c_int) as usize]
            as core::ffi::c_int == 0 as core::ffi::c_int
        {
            (*(*p_obj_exhaacplus_dec).p_state_aac)
                .qshift_adj[(i_2 as core::ffi::c_int + 1 as core::ffi::c_int)
                as usize] = (*(*p_obj_exhaacplus_dec).p_state_aac)
                .qshift_adj[0 as core::ffi::c_int as usize];
        }
        i_2 += 1;
    }
    if (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo == 1 as UWORD32
        && total_elements == 1 as core::ffi::c_int
        && num_ch as core::ffi::c_int == 2 as core::ffi::c_int
        && (*p_obj_exhaacplus_dec).aac_config.dup_stereo_flag as core::ffi::c_int
            == 1 as core::ffi::c_int
    {
        let mut i_3: WORD = 0;
        if (*p_state_enhaacplus_dec).sbr_present == 0
            && (*(*p_obj_exhaacplus_dec).p_state_aac)
                .qshift_adj[0 as core::ffi::c_int as usize] as core::ffi::c_int != LD_OBJ
        {
            i_3 = 0 as core::ffi::c_int as WORD;
            while i_3 < frame_size as core::ffi::c_int {
                *(actual_out_buffer as *mut WORD32)
                    .offset((2 as WORD * i_3) as isize)
                    .offset(1 as core::ffi::c_int as isize) = *(actual_out_buffer
                    as *mut WORD32)
                    .offset((2 as WORD * i_3) as isize);
                i_3 += 1;
            }
        } else {
            i_3 = 0 as core::ffi::c_int as WORD;
            while i_3 < frame_size as core::ffi::c_int {
                *actual_out_buffer
                    .offset((2 as WORD * i_3) as isize)
                    .offset(1 as core::ffi::c_int as isize) = *actual_out_buffer
                    .offset((2 as WORD * i_3) as isize);
                i_3 += 1;
            }
        }
    }
    if (*p_state_enhaacplus_dec).sbr_present == 0
        && (*(*p_obj_exhaacplus_dec).p_state_aac).peak_lim_init as core::ffi::c_int
            == 1 as core::ffi::c_int
        && (*(*p_obj_exhaacplus_dec).p_state_aac)
            .qshift_adj[0 as core::ffi::c_int as usize] as core::ffi::c_int != LD_OBJ
    {
        if (*p_obj_exhaacplus_dec).aac_config.peak_limiter_off == 0 {
            ixheaacd_peak_limiter_process(
                &mut (*p_state_enhaacplus_dec).peak_limiter,
                time_data as *mut core::ffi::c_void,
                frame_size as UWORD32,
                ((*(*p_obj_exhaacplus_dec).p_state_aac).qshift_adj).as_mut_ptr(),
            );
        } else {
            ixheaacd_scale_adjust(
                time_data as *mut core::ffi::c_void,
                frame_size as UWORD32,
                ((*(*p_obj_exhaacplus_dec).p_state_aac).qshift_adj).as_mut_ptr(),
                total_channels,
            );
        }
        i_2 = 0 as core::ffi::c_int as WORD32;
        while i_2 < frame_size as core::ffi::c_int * 2 as core::ffi::c_int {
            j_1 = 0 as core::ffi::c_int as WORD32;
            while j_1 < total_channels {
                *time_data
                    .offset((total_channels as WORD32 * i_2) as isize)
                    .offset(j_1 as isize) = ixheaac_round16(
                    *(time_data as *mut WORD32)
                        .offset((total_channels as WORD32 * i_2) as isize)
                        .offset(j_1 as isize),
                );
                j_1 += 1;
            }
            i_2 += 1;
        }
        memmove(
            time_data as *mut core::ffi::c_void,
            time_data
                .offset(
                    (total_channels as UWORD32)
                        .wrapping_mul(
                            (*(*p_obj_exhaacplus_dec).p_state_aac).delay_in_samples,
                        ) as isize,
                ) as *const core::ffi::c_void,
            (::core::mem::size_of::<WORD16>() as size_t)
                .wrapping_mul(num_of_out_samples as size_t)
                .wrapping_mul(total_channels as size_t),
        );
        (*(*p_obj_exhaacplus_dec).p_state_aac).delay_in_samples = ((*(*p_obj_exhaacplus_dec)
            .p_state_aac)
            .delay_in_samples)
            .wrapping_sub(
                (if (*(*p_obj_exhaacplus_dec).p_state_aac).delay_in_samples
                    < frame_size as UWORD16 as UWORD32
                {
                    (*(*p_obj_exhaacplus_dec).p_state_aac).delay_in_samples
                } else {
                    frame_size as UWORD16 as UWORD32
                }),
            );
    }
    if (*p_state_enhaacplus_dec).heaac_mps_handle.heaac_mps_present
        == 1 as core::ffi::c_int
    {
        let mut pstr_mps_state: *mut ia_heaac_mps_state_struct = &mut (*p_state_enhaacplus_dec)
            .heaac_mps_handle;
        if (*p_state_enhaacplus_dec).sbr_present as core::ffi::c_int
            == 0 as core::ffi::c_int
        {
            (*p_state_enhaacplus_dec).heaac_mps_handle.mps_decode = 1 as core::ffi::c_int
                as WORD32;
        } else {
            (*p_state_enhaacplus_dec).heaac_mps_handle.mps_with_sbr = 1
                as core::ffi::c_int as WORD32;
        }
        if (*p_obj_exhaacplus_dec).aac_config.ui_enh_sbr == 0 {
            (*p_state_enhaacplus_dec).heaac_mps_handle.mps_decode = 1 as core::ffi::c_int
                as WORD32;
        }
        if (*p_state_enhaacplus_dec).heaac_mps_handle.mps_init_done
            == 1 as core::ffi::c_int
        {
            (*(*p_obj_exhaacplus_dec).p_state_aac).heaac_mps_handle.frame_ok = (*p_obj_exhaacplus_dec)
                .aac_config
                .frame_status;
            (*(*p_obj_exhaacplus_dec).p_state_aac).heaac_mps_handle.ec_flag = (*p_obj_exhaacplus_dec)
                .aac_config
                .ui_err_conceal;
            error_code = ixheaacd_heaac_mps_apply(
                p_obj_exhaacplus_dec,
                actual_out_buffer,
                mps_buffer,
                (*p_state_enhaacplus_dec).ui_mps_out_bytes,
            ) as WORD32;
            if error_code != IA_NO_ERROR {
                return error_code;
            }
            (*p_state_enhaacplus_dec).heaac_mps_handle.mps_decode = 1 as core::ffi::c_int
                as WORD32;
            (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = (((*pstr_mps_state)
                .num_output_channels_at * (*pstr_mps_state).frame_length) as UWORD32)
                .wrapping_mul(
                    (*p_obj_exhaacplus_dec).aac_config.ui_pcm_wdsz
                        >> 3 as core::ffi::c_int,
                );
            (*(*p_obj_exhaacplus_dec).p_state_aac).heaac_mps_handle.first_frame = 0
                as core::ffi::c_int as WORD32;
        }
    }
    if total_channels > 2 as core::ffi::c_int
        && 1 as core::ffi::c_int == (*p_obj_exhaacplus_dec).aac_config.downmix
    {
        ixheaacd_dec_downmix_to_stereo(
            p_obj_exhaacplus_dec,
            num_of_out_samples,
            total_elements,
            time_data,
            total_channels,
        );
        total_channels = 2 as core::ffi::c_int as WORD;
        (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = (((*p_state_enhaacplus_dec)
            .num_of_out_samples as core::ffi::c_int * 2 as core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize) as UWORD32;
    }
    if (*p_obj_exhaacplus_dec).aac_config.flag_downmix != 0
        && total_channels == 2 as core::ffi::c_int
    {
        let mut out_ch: WORD32 = 1 as WORD32;
        let mut i_4: WORD = 0;
        if (*p_obj_exhaacplus_dec).aac_config.flag_to_stereo == 1 as UWORD32 {
            out_ch = 2 as core::ffi::c_int as WORD32;
        }
        (*p_obj_exhaacplus_dec).aac_config.ui_n_channels = out_ch as UWORD32;
        (*(*p_obj_exhaacplus_dec).p_state_aac).ui_out_bytes = (((*p_state_enhaacplus_dec)
            .num_of_out_samples as WORD32 * out_ch) as usize)
            .wrapping_mul(::core::mem::size_of::<WORD16>() as usize) as UWORD32;
        i_4 = 0 as core::ffi::c_int as WORD;
        while i_4 < num_of_out_samples as core::ffi::c_int {
            let mut temp: WORD16 = 0;
            temp = ((*time_data
                .offset(
                    (2 as core::ffi::c_int * i_4 as core::ffi::c_int
                        + 0 as core::ffi::c_int) as isize,
                ) as core::ffi::c_int >> 1 as core::ffi::c_int)
                + (*time_data
                    .offset(
                        (2 as core::ffi::c_int * i_4 as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) as core::ffi::c_int >> 1 as core::ffi::c_int)) as WORD16;
            if out_ch == 2 as core::ffi::c_int {
                *time_data
                    .offset(
                        (2 as core::ffi::c_int * i_4 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    ) = temp;
                *time_data
                    .offset(
                        (2 as core::ffi::c_int * i_4 as core::ffi::c_int
                            + 1 as core::ffi::c_int) as isize,
                    ) = *time_data
                    .offset(
                        (2 as core::ffi::c_int * i_4 as core::ffi::c_int
                            + 0 as core::ffi::c_int) as isize,
                    );
            } else {
                *time_data.offset(i_4 as isize) = temp;
            }
            i_4 += 1;
        }
    }
    if (*p_state_enhaacplus_dec).s_adts_hdr_present != 0 {
        if adts.no_raw_data_blocks != 0 as core::ffi::c_int {
            if adts.protection_absent == 0 as core::ffi::c_int
                && (*it_bit_buff).cnt_bits >= 16 as core::ffi::c_int
            {
                adts.crc_check = ixheaacd_read_bits_buf(
                    it_bit_buff as *mut ia_bit_buf_struct,
                    16 as WORD,
                );
            }
        }
        (*p_state_enhaacplus_dec).b_n_raw_data_blk -= 1;
    }
    ixheaacd_updatebytesconsumed(p_state_enhaacplus_dec, it_bit_buff);
    if (*p_state_enhaacplus_dec).bs_format == LOAS_BSFORMAT {
        (*p_state_enhaacplus_dec).i_bytes_consumed = (audio_mux_length_bytes_last
            as size_t)
            .wrapping_add(bytes_for_sync) as WORD32;
    }
    if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0
        && (*p_obj_exhaacplus_dec).aac_config.first_frame != 0
    {
        (*p_obj_exhaacplus_dec).aac_config.first_frame = 0 as core::ffi::c_int as FLAG;
    }
    if (*p_obj_exhaacplus_dec).aac_config.ui_err_conceal != 0 {
        if (*p_obj_exhaacplus_dec).aac_config.frame_status != 1 as core::ffi::c_int {
            (*p_state_enhaacplus_dec).i_bytes_consumed = (*p_state_enhaacplus_dec)
                .ui_in_bytes as WORD32;
        }
        return IA_NO_ERROR;
    } else {
        return error_code
    };
}
