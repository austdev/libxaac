#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]


pub mod src {
pub mod common {
pub mod ixheaac_esbr_fft;
pub mod ixheaac_esbr_rom;
pub mod ixheaac_fft_ifft_32x32_rom;
} // mod common
pub mod decoder {
pub mod drc_src {
pub mod impd_drc_api;
pub mod impd_drc_bitbuffer;
pub mod impd_drc_dec;
pub mod impd_drc_dynamic_payload;
pub mod impd_drc_eq;
pub mod impd_drc_extr_delta_coded_info;
pub mod impd_drc_filter_bank;
pub mod impd_drc_gain_dec;
pub mod impd_drc_gain_decoder;
pub mod impd_drc_init;
pub mod impd_drc_interface_decoder;
pub mod impd_drc_loudness_control;
pub mod impd_drc_main_td_process;
pub mod impd_drc_multiband;
pub mod impd_drc_parametric_dec;
pub mod impd_drc_peak_limiter;
pub mod impd_drc_process;
pub mod impd_drc_rom;
pub mod impd_drc_selection_process;
pub mod impd_drc_selection_process_drcset_selection;
pub mod impd_drc_selection_process_init;
pub mod impd_drc_shape_filter;
pub mod impd_drc_static_payload;
} // mod drc_src
pub mod generic {
pub mod ixheaacd_qmf_dec_generic;
} // mod generic
pub mod ixheaacd_Windowing;
pub mod ixheaacd_aac_ec;
pub mod ixheaacd_aac_imdct;
pub mod ixheaacd_aac_rom;
pub mod ixheaacd_aac_tns;
pub mod ixheaacd_aacdecoder;
pub mod ixheaacd_aacpluscheck;
pub mod ixheaacd_acelp_bitparse;
pub mod ixheaacd_acelp_decode;
pub mod ixheaacd_acelp_mdct;
pub mod ixheaacd_acelp_tools;
pub mod ixheaacd_adts_crc_check;
pub mod ixheaacd_api;
pub mod ixheaacd_arith_dec;
pub mod ixheaacd_avq_dec;
pub mod ixheaacd_avq_rom;
pub mod ixheaacd_basic_funcs;
pub mod ixheaacd_basic_ops;
pub mod ixheaacd_bitbuffer;
pub mod ixheaacd_block;
pub mod ixheaacd_channel;
pub mod ixheaacd_common_initfuncs;
pub mod ixheaacd_common_lpfuncs;
pub mod ixheaacd_common_rom;
pub mod ixheaacd_create;
pub mod ixheaacd_decode_main;
pub mod ixheaacd_drc_freq_dec;
pub mod ixheaacd_dsp_fft32x32s;
pub mod ixheaacd_ec_rom;
pub mod ixheaacd_env_calc;
pub mod ixheaacd_env_dec;
pub mod ixheaacd_env_extr;
pub mod ixheaacd_esbr_envcal;
pub mod ixheaacd_esbr_polyphase;
pub mod ixheaacd_ext_ch_ele;
pub mod ixheaacd_fft;
pub mod ixheaacd_fft_ifft_32x32;
pub mod ixheaacd_freq_sca;
pub mod ixheaacd_fwd_alias_cnx;
pub mod ixheaacd_hbe_dft_trans;
pub mod ixheaacd_hbe_trans;
pub mod ixheaacd_headerdecode;
pub mod ixheaacd_huff_code_reorder;
pub mod ixheaacd_huff_tools;
pub mod ixheaacd_hufftables;
pub mod ixheaacd_hybrid;
pub mod ixheaacd_imdct;
pub mod ixheaacd_init_config;
pub mod ixheaacd_initfuncs;
pub mod ixheaacd_latmdemux;
pub mod ixheaacd_ld_mps_config;
pub mod ixheaacd_ld_mps_dec;
pub mod ixheaacd_longblock;
pub mod ixheaacd_lpc;
pub mod ixheaacd_lpc_dec;
pub mod ixheaacd_lpfuncs;
pub mod ixheaacd_lpp_tran;
pub mod ixheaacd_lt_predict;
pub mod ixheaacd_mps_apply_common;
pub mod ixheaacd_mps_apply_m1;
pub mod ixheaacd_mps_apply_m2;
pub mod ixheaacd_mps_bitdec;
pub mod ixheaacd_mps_blind;
pub mod ixheaacd_mps_calc_m1m2_common;
pub mod ixheaacd_mps_calc_m1m2_emm;
pub mod ixheaacd_mps_calc_m1m2_tree_515x;
pub mod ixheaacd_mps_calc_m1m2_tree_51sx;
pub mod ixheaacd_mps_calc_m1m2_tree_52xx;
pub mod ixheaacd_mps_calc_m1m2_tree_727x;
pub mod ixheaacd_mps_calc_m1m2_tree_757x;
pub mod ixheaacd_mps_dec;
pub mod ixheaacd_mps_decorr;
pub mod ixheaacd_mps_get_index;
pub mod ixheaacd_mps_hybrid_filt;
pub mod ixheaacd_mps_initfuncs;
pub mod ixheaacd_mps_m1m2_common;
pub mod ixheaacd_mps_mdct_2_qmf;
pub mod ixheaacd_mps_parse;
pub mod ixheaacd_mps_poly_filt;
pub mod ixheaacd_mps_polyphase;
pub mod ixheaacd_mps_pre_mix;
pub mod ixheaacd_mps_process;
pub mod ixheaacd_mps_res_block;
pub mod ixheaacd_mps_res_channel;
pub mod ixheaacd_mps_res_channel_info;
pub mod ixheaacd_mps_res_longblock;
pub mod ixheaacd_mps_res_pns_js_thumb;
pub mod ixheaacd_mps_res_pulsedata;
pub mod ixheaacd_mps_res_tns;
pub mod ixheaacd_mps_reshape_bb_env;
pub mod ixheaacd_mps_rom;
pub mod ixheaacd_mps_smoothing;
pub mod ixheaacd_mps_temp_process;
pub mod ixheaacd_mps_temp_reshape;
pub mod ixheaacd_mps_tonality;
pub mod ixheaacd_multichannel;
pub mod ixheaacd_peak_limiter;
pub mod ixheaacd_pns_js_thumb;
pub mod ixheaacd_pred_vec_block;
pub mod ixheaacd_process;
pub mod ixheaacd_ps_bitdec;
pub mod ixheaacd_ps_dec;
pub mod ixheaacd_ps_dec_flt;
pub mod ixheaacd_pvc_rom;
pub mod ixheaacd_qmf_dec;
pub mod ixheaacd_rev_vlc;
pub mod ixheaacd_rom;
pub mod ixheaacd_sbr_crc;
pub mod ixheaacd_sbr_dec;
pub mod ixheaacd_sbr_rom;
pub mod ixheaacd_sbrdec_initfuncs;
pub mod ixheaacd_sbrdec_lpfuncs;
pub mod ixheaacd_sbrdecoder;
pub mod ixheaacd_spectrum_dec;
pub mod ixheaacd_stereo;
pub mod ixheaacd_tcx_fwd_alcnx;
pub mod ixheaacd_tcx_fwd_mdct;
pub mod ixheaacd_thumb_ps_dec;
pub mod ixheaacd_tns;
pub mod ixheaacd_usac_ec;
pub mod x86 {
pub mod ixheaacd_function_selector_x86;
} // mod x86
} // mod decoder
pub mod test {
pub mod decoder {
pub mod ixheaacd_error;
pub mod ixheaacd_fileifc;
pub mod ixheaacd_metadata_read;
} // mod decoder
} // mod test
} // mod src
