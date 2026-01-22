extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn pow(__x: core::ffi::c_double, __y: core::ffi::c_double) -> core::ffi::c_double;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: core::ffi::c_int) -> !;
    fn ixheaacd_read_bits_buf(
        it_bit_buff: *mut ia_bit_buf_struct,
        no_of_bits: WORD,
    ) -> WORD32;
}
pub type size_t = usize;
pub type WORD8 = core::ffi::c_schar;
pub type UWORD8 = core::ffi::c_uchar;
pub type WORD16 = core::ffi::c_short;
pub type UWORD16 = core::ffi::c_ushort;
pub type WORD32 = core::ffi::c_int;
pub type UWORD32 = core::ffi::c_uint;
pub type WORD64 = core::ffi::c_longlong;
pub type FLOAT32 = core::ffi::c_float;
pub type VOID = ();
pub type FLAG = core::ffi::c_int;
pub type WORD = core::ffi::c_int;
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const DVB_DRC_ANC_DATA: C2RustUnnamed = 2;
pub const MPEG_DRC_EXT_DATA: C2RustUnnamed = 1;
pub const UNKNOWN_PAYLOAD: C2RustUnnamed = 0;
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
pub const AOT_ER_AAC_LD: AUDIO_OBJECT_TYPE = 23;
pub const AOT_ER_AAC_ELD: AUDIO_OBJECT_TYPE = 39;
pub type AUDIO_OBJECT_TYPE = core::ffi::c_uint;
pub const AOT_USAC: AUDIO_OBJECT_TYPE = 42;
pub const AOT_ESC: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_31: AUDIO_OBJECT_TYPE = 31;
pub const AOT_RSVD_30: AUDIO_OBJECT_TYPE = 30;
pub const AOT_PS: AUDIO_OBJECT_TYPE = 29;
pub const AOT_RSVD_28: AUDIO_OBJECT_TYPE = 28;
pub const AOT_ER_PARA: AUDIO_OBJECT_TYPE = 27;
pub const AOT_ER_HILN: AUDIO_OBJECT_TYPE = 26;
pub const AOT_ER_HVXC: AUDIO_OBJECT_TYPE = 25;
pub const AOT_ER_CELP: AUDIO_OBJECT_TYPE = 24;
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
pub const IA_NO_ERROR: core::ffi::c_int = 0 as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES: core::ffi::c_int = 0x1804
    as core::ffi::c_int;
pub const IA_XHEAAC_DEC_EXE_FATAL_INVALID_DRC_DATA: core::ffi::c_uint = 0xffff9803
    as core::ffi::c_uint;
pub const MAX_DRC_BANDS: core::ffi::c_int = 16 as core::ffi::c_int;
pub const MAX_AUDIO_CHANNELS: core::ffi::c_int = 8 as core::ffi::c_int;
pub const SBR_QMF_SUB_SAMPLES: core::ffi::c_int = 64 as core::ffi::c_int;
pub const SBR_QMF_SUB_BANDS: core::ffi::c_int = 64 as core::ffi::c_int;
pub const DVB_ANC_DATA_SYNC_BYTE: core::ffi::c_int = 0xbc as core::ffi::c_int;
pub const DRC_SBR_ONE_Q25: core::ffi::c_int = (1 as core::ffi::c_int)
    << 25 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_mult32x16in32_shift29(
    mut a: WORD32,
    mut b: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    result = (temp_result >> 29 as core::ffi::c_int) as WORD32;
    return result;
}
#[inline]
unsafe extern "C" fn ixheaacd_mult32x16in32_shift25(
    mut a: WORD32,
    mut b: WORD32,
) -> WORD32 {
    let mut result: WORD32 = 0;
    let mut temp_result: WORD64 = 0;
    temp_result = a as WORD64 * b as WORD64;
    temp_result = temp_result >> 25 as core::ffi::c_int;
    if temp_result >= MAX_32 as WORD64 {
        result = MAX_32;
    } else if temp_result < MIN_32 as WORD64 {
        result = MIN_32;
    } else {
        result = temp_result as WORD32;
    }
    return result;
}
static mut ixheaacd_drc_pow_tbl_2_q29: [WORD32; 1000] = [
    536870912 as core::ffi::c_int,
    537242967 as core::ffi::c_int,
    537615991 as core::ffi::c_int,
    537988562 as core::ffi::c_int,
    538361391 as core::ffi::c_int,
    538734479 as core::ffi::c_int,
    539108539 as core::ffi::c_int,
    539482144 as core::ffi::c_int,
    539856009 as core::ffi::c_int,
    540230847 as core::ffi::c_int,
    540605230 as core::ffi::c_int,
    540979873 as core::ffi::c_int,
    541354776 as core::ffi::c_int,
    541730654 as core::ffi::c_int,
    542106077 as core::ffi::c_int,
    542481760 as core::ffi::c_int,
    542858421 as core::ffi::c_int,
    543234626 as core::ffi::c_int,
    543611091 as core::ffi::c_int,
    543987817 as core::ffi::c_int,
    544365523 as core::ffi::c_int,
    544742772 as core::ffi::c_int,
    545120282 as core::ffi::c_int,
    545498775 as core::ffi::c_int,
    545876810 as core::ffi::c_int,
    546255106 as core::ffi::c_int,
    546633664 as core::ffi::c_int,
    547013208 as core::ffi::c_int,
    547392292 as core::ffi::c_int,
    547771638 as core::ffi::c_int,
    548151972 as core::ffi::c_int,
    548531845 as core::ffi::c_int,
    548911981 as core::ffi::c_int,
    549293107 as core::ffi::c_int,
    549673770 as core::ffi::c_int,
    550054698 as core::ffi::c_int,
    550435889 as core::ffi::c_int,
    550818073 as core::ffi::c_int,
    551199794 as core::ffi::c_int,
    551581779 as core::ffi::c_int,
    551964758 as core::ffi::c_int,
    552347273 as core::ffi::c_int,
    552730053 as core::ffi::c_int,
    553113099 as core::ffi::c_int,
    553497142 as core::ffi::c_int,
    553880719 as core::ffi::c_int,
    554264562 as core::ffi::c_int,
    554649404 as core::ffi::c_int,
    555033779 as core::ffi::c_int,
    555418421 as core::ffi::c_int,
    555803330 as core::ffi::c_int,
    556189241 as core::ffi::c_int,
    556574683 as core::ffi::c_int,
    556960393 as core::ffi::c_int,
    557347107 as core::ffi::c_int,
    557733352 as core::ffi::c_int,
    558119865 as core::ffi::c_int,
    558506646 as core::ffi::c_int,
    558894433 as core::ffi::c_int,
    559281751 as core::ffi::c_int,
    559669337 as core::ffi::c_int,
    560057931 as core::ffi::c_int,
    560446055 as core::ffi::c_int,
    560834448 as core::ffi::c_int,
    561223110 as core::ffi::c_int,
    561612784 as core::ffi::c_int,
    562001985 as core::ffi::c_int,
    562391456 as core::ffi::c_int,
    562781941 as core::ffi::c_int,
    563171952 as core::ffi::c_int,
    563562234 as core::ffi::c_int,
    563952786 as core::ffi::c_int,
    564344355 as core::ffi::c_int,
    564735450 as core::ffi::c_int,
    565126815 as core::ffi::c_int,
    565519199 as core::ffi::c_int,
    565911108 as core::ffi::c_int,
    566303288 as core::ffi::c_int,
    566695739 as core::ffi::c_int,
    567089213 as core::ffi::c_int,
    567482209 as core::ffi::c_int,
    567875478 as core::ffi::c_int,
    568269771 as core::ffi::c_int,
    568663586 as core::ffi::c_int,
    569057673 as core::ffi::c_int,
    569452034 as core::ffi::c_int,
    569847421 as core::ffi::c_int,
    570242329 as core::ffi::c_int,
    570637511 as core::ffi::c_int,
    571033721 as core::ffi::c_int,
    571429451 as core::ffi::c_int,
    571825455 as core::ffi::c_int,
    572221734 as core::ffi::c_int,
    572619044 as core::ffi::c_int,
    573015873 as core::ffi::c_int,
    573412977 as core::ffi::c_int,
    573811114 as core::ffi::c_int,
    574208769 as core::ffi::c_int,
    574606699 as core::ffi::c_int,
    575005666 as core::ffi::c_int,
    575404148 as core::ffi::c_int,
    575802907 as core::ffi::c_int,
    576201942 as core::ffi::c_int,
    576602016 as core::ffi::c_int,
    577001605 as core::ffi::c_int,
    577401471 as core::ffi::c_int,
    577802378 as core::ffi::c_int,
    578202799 as core::ffi::c_int,
    578603497 as core::ffi::c_int,
    579004473 as core::ffi::c_int,
    579406493 as core::ffi::c_int,
    579808025 as core::ffi::c_int,
    580209836 as core::ffi::c_int,
    580612693 as core::ffi::c_int,
    581015061 as core::ffi::c_int,
    581417708 as core::ffi::c_int,
    581820634 as core::ffi::c_int,
    582224610 as core::ffi::c_int,
    582628095 as core::ffi::c_int,
    583031860 as core::ffi::c_int,
    583436676 as core::ffi::c_int,
    583841002 as core::ffi::c_int,
    584245607 as core::ffi::c_int,
    584650493 as core::ffi::c_int,
    585056433 as core::ffi::c_int,
    585461881 as core::ffi::c_int,
    585867610 as core::ffi::c_int,
    586274395 as core::ffi::c_int,
    586680687 as core::ffi::c_int,
    587087260 as core::ffi::c_int,
    587494116 as core::ffi::c_int,
    587902030 as core::ffi::c_int,
    588309450 as core::ffi::c_int,
    588717152 as core::ffi::c_int,
    589125916 as core::ffi::c_int,
    589534184 as core::ffi::c_int,
    589942735 as core::ffi::c_int,
    590351569 as core::ffi::c_int,
    590761467 as core::ffi::c_int,
    591170869 as core::ffi::c_int,
    591580554 as core::ffi::c_int,
    591991306 as core::ffi::c_int,
    592401560 as core::ffi::c_int,
    592812098 as core::ffi::c_int,
    593222920 as core::ffi::c_int,
    593634813 as core::ffi::c_int,
    594046205 as core::ffi::c_int,
    594457883 as core::ffi::c_int,
    594870633 as core::ffi::c_int,
    595282882 as core::ffi::c_int,
    595695417 as core::ffi::c_int,
    596108238 as core::ffi::c_int,
    596522133 as core::ffi::c_int,
    596935527 as core::ffi::c_int,
    597349207 as core::ffi::c_int,
    597763964 as core::ffi::c_int,
    598178218 as core::ffi::c_int,
    598592760 as core::ffi::c_int,
    599008380 as core::ffi::c_int,
    599423497 as core::ffi::c_int,
    599838901 as core::ffi::c_int,
    600254594 as core::ffi::c_int,
    600671368 as core::ffi::c_int,
    601087637 as core::ffi::c_int,
    601504195 as core::ffi::c_int,
    601921837 as core::ffi::c_int,
    602338973 as core::ffi::c_int,
    602756397 as core::ffi::c_int,
    603174111 as core::ffi::c_int,
    603592913 as core::ffi::c_int,
    604011207 as core::ffi::c_int,
    604429790 as core::ffi::c_int,
    604849464 as core::ffi::c_int,
    605268628 as core::ffi::c_int,
    605688083 as core::ffi::c_int,
    606107829 as core::ffi::c_int,
    606528668 as core::ffi::c_int,
    606948996 as core::ffi::c_int,
    607369615 as core::ffi::c_int,
    607791330 as core::ffi::c_int,
    608212533 as core::ffi::c_int,
    608634029 as core::ffi::c_int,
    609055816 as core::ffi::c_int,
    609478701 as core::ffi::c_int,
    609901074 as core::ffi::c_int,
    610323739 as core::ffi::c_int,
    610747505 as core::ffi::c_int,
    611170757 as core::ffi::c_int,
    611594302 as core::ffi::c_int,
    612018141 as core::ffi::c_int,
    612443083 as core::ffi::c_int,
    612867510 as core::ffi::c_int,
    613292231 as core::ffi::c_int,
    613718058 as core::ffi::c_int,
    614143369 as core::ffi::c_int,
    614568974 as core::ffi::c_int,
    614994874 as core::ffi::c_int,
    615421883 as core::ffi::c_int,
    615848375 as core::ffi::c_int,
    616275162 as core::ffi::c_int,
    616703060 as core::ffi::c_int,
    617130439 as core::ffi::c_int,
    617558114 as core::ffi::c_int,
    617986086 as core::ffi::c_int,
    618415172 as core::ffi::c_int,
    618843738 as core::ffi::c_int,
    619272600 as core::ffi::c_int,
    619702579 as core::ffi::c_int,
    620132037 as core::ffi::c_int,
    620561793 as core::ffi::c_int,
    620991846 as core::ffi::c_int,
    621423019 as core::ffi::c_int,
    621853669 as core::ffi::c_int,
    622284618 as core::ffi::c_int,
    622716688 as core::ffi::c_int,
    623148235 as core::ffi::c_int,
    623580081 as core::ffi::c_int,
    624012226 as core::ffi::c_int,
    624445496 as core::ffi::c_int,
    624878241 as core::ffi::c_int,
    625311285 as core::ffi::c_int,
    625745457 as core::ffi::c_int,
    626179103 as core::ffi::c_int,
    626613049 as core::ffi::c_int,
    627048125 as core::ffi::c_int,
    627482673 as core::ffi::c_int,
    627917523 as core::ffi::c_int,
    628352674 as core::ffi::c_int,
    628788957 as core::ffi::c_int,
    629224712 as core::ffi::c_int,
    629660769 as core::ffi::c_int,
    630097961 as core::ffi::c_int,
    630534623 as core::ffi::c_int,
    630971588 as core::ffi::c_int,
    631408855 as core::ffi::c_int,
    631847261 as core::ffi::c_int,
    632285135 as core::ffi::c_int,
    632723313 as core::ffi::c_int,
    633162631 as core::ffi::c_int,
    633601417 as core::ffi::c_int,
    634040507 as core::ffi::c_int,
    634479901 as core::ffi::c_int,
    634920439 as core::ffi::c_int,
    635360443 as core::ffi::c_int,
    635800752 as core::ffi::c_int,
    636242207 as core::ffi::c_int,
    636683127 as core::ffi::c_int,
    637124352 as core::ffi::c_int,
    637565884 as core::ffi::c_int,
    638008564 as core::ffi::c_int,
    638450708 as core::ffi::c_int,
    638893159 as core::ffi::c_int,
    639336761 as core::ffi::c_int,
    639779826 as core::ffi::c_int,
    640223197 as core::ffi::c_int,
    640666876 as core::ffi::c_int,
    641111710 as core::ffi::c_int,
    641556004 as core::ffi::c_int,
    642000607 as core::ffi::c_int,
    642446367 as core::ffi::c_int,
    642891586 as core::ffi::c_int,
    643337114 as core::ffi::c_int,
    643782951 as core::ffi::c_int,
    644229948 as core::ffi::c_int,
    644676404 as core::ffi::c_int,
    645123169 as core::ffi::c_int,
    645571097 as core::ffi::c_int,
    646018482 as core::ffi::c_int,
    646466177 as core::ffi::c_int,
    646914182 as core::ffi::c_int,
    647363354 as core::ffi::c_int,
    647811981 as core::ffi::c_int,
    648260918 as core::ffi::c_int,
    648711025 as core::ffi::c_int,
    649160586 as core::ffi::c_int,
    649610458 as core::ffi::c_int,
    650060643 as core::ffi::c_int,
    650511999 as core::ffi::c_int,
    650962808 as core::ffi::c_int,
    651413929 as core::ffi::c_int,
    651866225 as core::ffi::c_int,
    652317973 as core::ffi::c_int,
    652770033 as core::ffi::c_int,
    653223271 as core::ffi::c_int,
    653675959 as core::ffi::c_int,
    654128960 as core::ffi::c_int,
    654582276 as core::ffi::c_int,
    655036772 as core::ffi::c_int,
    655490716 as core::ffi::c_int,
    655944976 as core::ffi::c_int,
    656400417 as core::ffi::c_int,
    656855307 as core::ffi::c_int,
    657310512 as core::ffi::c_int,
    657766033 as core::ffi::c_int,
    658222739 as core::ffi::c_int,
    658678891 as core::ffi::c_int,
    659135360 as core::ffi::c_int,
    659593017 as core::ffi::c_int,
    660050119 as core::ffi::c_int,
    660507538 as core::ffi::c_int,
    660965274 as core::ffi::c_int,
    661424202 as core::ffi::c_int,
    661882573 as core::ffi::c_int,
    662341262 as core::ffi::c_int,
    662801145 as core::ffi::c_int,
    663260471 as core::ffi::c_int,
    663720114 as core::ffi::c_int,
    664180077 as core::ffi::c_int,
    664641237 as core::ffi::c_int,
    665101837 as core::ffi::c_int,
    665562757 as core::ffi::c_int,
    666024877 as core::ffi::c_int,
    666486436 as core::ffi::c_int,
    666948316 as core::ffi::c_int,
    667410515 as core::ffi::c_int,
    667873918 as core::ffi::c_int,
    668336759 as core::ffi::c_int,
    668799921 as core::ffi::c_int,
    669264288 as core::ffi::c_int,
    669728093 as core::ffi::c_int,
    670192218 as core::ffi::c_int,
    670656666 as core::ffi::c_int,
    671122323 as core::ffi::c_int,
    671587415 as core::ffi::c_int,
    672052829 as core::ffi::c_int,
    672519455 as core::ffi::c_int,
    672985516 as core::ffi::c_int,
    673451899 as core::ffi::c_int,
    673918605 as core::ffi::c_int,
    674386527 as core::ffi::c_int,
    674853881 as core::ffi::c_int,
    675321559 as core::ffi::c_int,
    675790455 as core::ffi::c_int,
    676258782 as core::ffi::c_int,
    676727434 as core::ffi::c_int,
    677196410 as core::ffi::c_int,
    677666608 as core::ffi::c_int,
    678136235 as core::ffi::c_int,
    678606188 as core::ffi::c_int,
    679077364 as core::ffi::c_int,
    679547969 as core::ffi::c_int,
    680018900 as core::ffi::c_int,
    680490157 as core::ffi::c_int,
    680962642 as core::ffi::c_int,
    681434553 as core::ffi::c_int,
    681906792 as core::ffi::c_int,
    682380260 as core::ffi::c_int,
    682853154 as core::ffi::c_int,
    683326375 as core::ffi::c_int,
    683800829 as core::ffi::c_int,
    684274707 as core::ffi::c_int,
    684748914 as core::ffi::c_int,
    685223450 as core::ffi::c_int,
    685699220 as core::ffi::c_int,
    686174414 as core::ffi::c_int,
    686649938 as core::ffi::c_int,
    687126699 as core::ffi::c_int,
    687602882 as core::ffi::c_int,
    688079395 as core::ffi::c_int,
    688556239 as core::ffi::c_int,
    689034324 as core::ffi::c_int,
    689511829 as core::ffi::c_int,
    689989665 as core::ffi::c_int,
    690468745 as core::ffi::c_int,
    690947244 as core::ffi::c_int,
    691426075 as core::ffi::c_int,
    691905238 as core::ffi::c_int,
    692385648 as core::ffi::c_int,
    692865476 as core::ffi::c_int,
    693345636 as core::ffi::c_int,
    693827046 as core::ffi::c_int,
    694307873 as core::ffi::c_int,
    694789033 as core::ffi::c_int,
    695270526 as core::ffi::c_int,
    695753273 as core::ffi::c_int,
    696235434 as core::ffi::c_int,
    696717930 as core::ffi::c_int,
    697201682 as core::ffi::c_int,
    697684847 as core::ffi::c_int,
    698168347 as core::ffi::c_int,
    698652182 as core::ffi::c_int,
    699137277 as core::ffi::c_int,
    699621784 as core::ffi::c_int,
    700106626 as core::ffi::c_int,
    700592731 as core::ffi::c_int,
    701078246 as core::ffi::c_int,
    701564098 as core::ffi::c_int,
    702050286 as core::ffi::c_int,
    702537740 as core::ffi::c_int,
    703024604 as core::ffi::c_int,
    703511804 as core::ffi::c_int,
    704000273 as core::ffi::c_int,
    704488150 as core::ffi::c_int,
    704976365 as core::ffi::c_int,
    705464918 as core::ffi::c_int,
    705954743 as core::ffi::c_int,
    706443974 as core::ffi::c_int,
    706933544 as core::ffi::c_int,
    707424389 as core::ffi::c_int,
    707914639 as core::ffi::c_int,
    708405228 as core::ffi::c_int,
    708896158 as core::ffi::c_int,
    709388365 as core::ffi::c_int,
    709879976 as core::ffi::c_int,
    710371927 as core::ffi::c_int,
    710865159 as core::ffi::c_int,
    711357793 as core::ffi::c_int,
    711850769 as core::ffi::c_int,
    712345028 as core::ffi::c_int,
    712838688 as core::ffi::c_int,
    713332689 as core::ffi::c_int,
    713827033 as core::ffi::c_int,
    714322665 as core::ffi::c_int,
    714817695 as core::ffi::c_int,
    715313068 as core::ffi::c_int,
    715809731 as core::ffi::c_int,
    716305792 as core::ffi::c_int,
    716802196 as core::ffi::c_int,
    717298945 as core::ffi::c_int,
    717796987 as core::ffi::c_int,
    718294425 as core::ffi::c_int,
    718792207 as core::ffi::c_int,
    719291286 as core::ffi::c_int,
    719789759 as core::ffi::c_int,
    720288578 as core::ffi::c_int,
    720787743 as core::ffi::c_int,
    721288207 as core::ffi::c_int,
    721788064 as core::ffi::c_int,
    722288268 as core::ffi::c_int,
    722789774 as core::ffi::c_int,
    723290672 as core::ffi::c_int,
    723791917 as core::ffi::c_int,
    724293510 as core::ffi::c_int,
    724796408 as core::ffi::c_int,
    725298697 as core::ffi::c_int,
    725801333 as core::ffi::c_int,
    726305278 as core::ffi::c_int,
    726808613 as core::ffi::c_int,
    727312296 as core::ffi::c_int,
    727816328 as core::ffi::c_int,
    728321672 as core::ffi::c_int,
    728826404 as core::ffi::c_int,
    729331485 as core::ffi::c_int,
    729837881 as core::ffi::c_int,
    730343664 as core::ffi::c_int,
    730849797 as core::ffi::c_int,
    731356280 as core::ffi::c_int,
    731864082 as core::ffi::c_int,
    732371269 as core::ffi::c_int,
    732878807 as core::ffi::c_int,
    733387666 as core::ffi::c_int,
    733895909 as core::ffi::c_int,
    734404503 as core::ffi::c_int,
    734913450 as core::ffi::c_int,
    735423722 as core::ffi::c_int,
    735933376 as core::ffi::c_int,
    736443382 as core::ffi::c_int,
    736954717 as core::ffi::c_int,
    737465431 as core::ffi::c_int,
    737976499 as core::ffi::c_int,
    738487922 as core::ffi::c_int,
    739000676 as core::ffi::c_int,
    739512808 as core::ffi::c_int,
    740025295 as core::ffi::c_int,
    740539116 as core::ffi::c_int,
    741052315 as core::ffi::c_int,
    741565869 as core::ffi::c_int,
    742079779 as core::ffi::c_int,
    742595027 as core::ffi::c_int,
    743109650 as core::ffi::c_int,
    743624629 as core::ffi::c_int,
    744140950 as core::ffi::c_int,
    744656644 as core::ffi::c_int,
    745172696 as core::ffi::c_int,
    745690092 as core::ffi::c_int,
    746206860 as core::ffi::c_int,
    746723986 as core::ffi::c_int,
    747241470 as core::ffi::c_int,
    747760302 as core::ffi::c_int,
    748278505 as core::ffi::c_int,
    748797067 as core::ffi::c_int,
    749316978 as core::ffi::c_int,
    749836260 as core::ffi::c_int,
    750355901 as core::ffi::c_int,
    750875903 as core::ffi::c_int,
    751397258 as core::ffi::c_int,
    751917981 as core::ffi::c_int,
    752439065 as core::ffi::c_int,
    752961506 as core::ffi::c_int,
    753483313 as core::ffi::c_int,
    754005482 as core::ffi::c_int,
    754528012 as core::ffi::c_int,
    755051903 as core::ffi::c_int,
    755575159 as core::ffi::c_int,
    756098778 as core::ffi::c_int,
    756623759 as core::ffi::c_int,
    757148104 as core::ffi::c_int,
    757672813 as core::ffi::c_int,
    758197885 as core::ffi::c_int,
    758724324 as core::ffi::c_int,
    759250125 as core::ffi::c_int,
    759776290 as core::ffi::c_int,
    760303825 as core::ffi::c_int,
    760830721 as core::ffi::c_int,
    761357981 as core::ffi::c_int,
    761885607 as core::ffi::c_int,
    762414607 as core::ffi::c_int,
    762942965 as core::ffi::c_int,
    763471690 as core::ffi::c_int,
    764001790 as core::ffi::c_int,
    764531249 as core::ffi::c_int,
    765061074 as core::ffi::c_int,
    765591266 as core::ffi::c_int,
    766122838 as core::ffi::c_int,
    766653766 as core::ffi::c_int,
    767185062 as core::ffi::c_int,
    767717742 as core::ffi::c_int,
    768249775 as core::ffi::c_int,
    768782177 as core::ffi::c_int,
    769314948 as core::ffi::c_int,
    769849106 as core::ffi::c_int,
    770382616 as core::ffi::c_int,
    770916497 as core::ffi::c_int,
    771451767 as core::ffi::c_int,
    771986388 as core::ffi::c_int,
    772521379 as core::ffi::c_int,
    773057763 as core::ffi::c_int,
    773593497 as core::ffi::c_int,
    774129603 as core::ffi::c_int,
    774666080 as core::ffi::c_int,
    775203953 as core::ffi::c_int,
    775741174 as core::ffi::c_int,
    776278768 as core::ffi::c_int,
    776817761 as core::ffi::c_int,
    777356101 as core::ffi::c_int,
    777894814 as core::ffi::c_int,
    778433900 as core::ffi::c_int,
    778974389 as core::ffi::c_int,
    779514224 as core::ffi::c_int,
    780054432 as core::ffi::c_int,
    780596047 as core::ffi::c_int,
    781137005 as core::ffi::c_int,
    781678338 as core::ffi::c_int,
    782220046 as core::ffi::c_int,
    782763164 as core::ffi::c_int,
    783305624 as core::ffi::c_int,
    783848460 as core::ffi::c_int,
    784392709 as core::ffi::c_int,
    784936298 as core::ffi::c_int,
    785480264 as core::ffi::c_int,
    786024607 as core::ffi::c_int,
    786570367 as core::ffi::c_int,
    787115466 as core::ffi::c_int,
    787660942 as core::ffi::c_int,
    788207838 as core::ffi::c_int,
    788754071 as core::ffi::c_int,
    789300683 as core::ffi::c_int,
    789847673 as core::ffi::c_int,
    790396087 as core::ffi::c_int,
    790943837 as core::ffi::c_int,
    791491966 as core::ffi::c_int,
    792041522 as core::ffi::c_int,
    792590412 as core::ffi::c_int,
    793139683 as core::ffi::c_int,
    793689333 as core::ffi::c_int,
    794240415 as core::ffi::c_int,
    794790829 as core::ffi::c_int,
    795341624 as core::ffi::c_int,
    795893853 as core::ffi::c_int,
    796445413 as core::ffi::c_int,
    796997355 as core::ffi::c_int,
    797549679 as core::ffi::c_int,
    798103441 as core::ffi::c_int,
    798656532 as core::ffi::c_int,
    799210006 as core::ffi::c_int,
    799764921 as core::ffi::c_int,
    800319163 as core::ffi::c_int,
    800873790 as core::ffi::c_int,
    801428800 as core::ffi::c_int,
    801985256 as core::ffi::c_int,
    802541037 as core::ffi::c_int,
    803097203 as core::ffi::c_int,
    803654817 as core::ffi::c_int,
    804211755 as core::ffi::c_int,
    804769079 as core::ffi::c_int,
    805326789 as core::ffi::c_int,
    805885951 as core::ffi::c_int,
    806444435 as core::ffi::c_int,
    807003307 as core::ffi::c_int,
    807563633 as core::ffi::c_int,
    808123280 as core::ffi::c_int,
    808683314 as core::ffi::c_int,
    809243737 as core::ffi::c_int,
    809805619 as core::ffi::c_int,
    810366819 as core::ffi::c_int,
    810928409 as core::ffi::c_int,
    811491460 as core::ffi::c_int,
    812053829 as core::ffi::c_int,
    812616587 as core::ffi::c_int,
    813179736 as core::ffi::c_int,
    813744351 as core::ffi::c_int,
    814308281 as core::ffi::c_int,
    814872602 as core::ffi::c_int,
    815438392 as core::ffi::c_int,
    816003496 as core::ffi::c_int,
    816568991 as core::ffi::c_int,
    817135959 as core::ffi::c_int,
    817702240 as core::ffi::c_int,
    818268913 as core::ffi::c_int,
    818835978 as core::ffi::c_int,
    819404520 as core::ffi::c_int,
    819972373 as core::ffi::c_int,
    820540619 as core::ffi::c_int,
    821110344 as core::ffi::c_int,
    821679379 as core::ffi::c_int,
    822248808 as core::ffi::c_int,
    822818632 as core::ffi::c_int,
    823389939 as core::ffi::c_int,
    823960554 as core::ffi::c_int,
    824531564 as core::ffi::c_int,
    825104060 as core::ffi::c_int,
    825675863 as core::ffi::c_int,
    826248061 as core::ffi::c_int,
    826820657 as core::ffi::c_int,
    827394742 as core::ffi::c_int,
    827968132 as core::ffi::c_int,
    828541920 as core::ffi::c_int,
    829117201 as core::ffi::c_int,
    829691784 as core::ffi::c_int,
    830266766 as core::ffi::c_int,
    830842146 as core::ffi::c_int,
    831419024 as core::ffi::c_int,
    831995203 as core::ffi::c_int,
    832571781 as core::ffi::c_int,
    833149860 as core::ffi::c_int,
    833727238 as core::ffi::c_int,
    834305017 as core::ffi::c_int,
    834883195 as core::ffi::c_int,
    835462879 as core::ffi::c_int,
    836041861 as core::ffi::c_int,
    836621243 as core::ffi::c_int,
    837202134 as core::ffi::c_int,
    837782320 as core::ffi::c_int,
    838362909 as core::ffi::c_int,
    838943900 as core::ffi::c_int,
    839526403 as core::ffi::c_int,
    840108200 as core::ffi::c_int,
    840690401 as core::ffi::c_int,
    841274117 as core::ffi::c_int,
    841857125 as core::ffi::c_int,
    842440538 as core::ffi::c_int,
    843025469 as core::ffi::c_int,
    843609691 as core::ffi::c_int,
    844194318 as core::ffi::c_int,
    844779350 as core::ffi::c_int,
    845365905 as core::ffi::c_int,
    845951749 as core::ffi::c_int,
    846537999 as core::ffi::c_int,
    847125775 as core::ffi::c_int,
    847712839 as core::ffi::c_int,
    848300310 as core::ffi::c_int,
    848888187 as core::ffi::c_int,
    849477595 as core::ffi::c_int,
    850066289 as core::ffi::c_int,
    850655390 as core::ffi::c_int,
    851246025 as core::ffi::c_int,
    851835944 as core::ffi::c_int,
    852426272 as core::ffi::c_int,
    853017009 as core::ffi::c_int,
    853609284 as core::ffi::c_int,
    854200840 as core::ffi::c_int,
    854792807 as core::ffi::c_int,
    855386315 as core::ffi::c_int,
    855979103 as core::ffi::c_int,
    856572302 as core::ffi::c_int,
    857165912 as core::ffi::c_int,
    857761068 as core::ffi::c_int,
    858355502 as core::ffi::c_int,
    858950348 as core::ffi::c_int,
    859546742 as core::ffi::c_int,
    860142413 as core::ffi::c_int,
    860738498 as core::ffi::c_int,
    861334995 as core::ffi::c_int,
    861933045 as core::ffi::c_int,
    862530370 as core::ffi::c_int,
    863128110 as core::ffi::c_int,
    863727405 as core::ffi::c_int,
    864325973 as core::ffi::c_int,
    864924957 as core::ffi::c_int,
    865524355 as core::ffi::c_int,
    866125314 as core::ffi::c_int,
    866725545 as core::ffi::c_int,
    867326191 as core::ffi::c_int,
    867928401 as core::ffi::c_int,
    868529881 as core::ffi::c_int,
    869131778 as core::ffi::c_int,
    869734092 as core::ffi::c_int,
    870337974 as core::ffi::c_int,
    870941124 as core::ffi::c_int,
    871544692 as core::ffi::c_int,
    872149831 as core::ffi::c_int,
    872754236 as core::ffi::c_int,
    873359061 as core::ffi::c_int,
    873964304 as core::ffi::c_int,
    874571123 as core::ffi::c_int,
    875177207 as core::ffi::c_int,
    875783710 as core::ffi::c_int,
    876391792 as core::ffi::c_int,
    876999138 as core::ffi::c_int,
    877606904 as core::ffi::c_int,
    878215091 as core::ffi::c_int,
    878824861 as core::ffi::c_int,
    879433893 as core::ffi::c_int,
    880043346 as core::ffi::c_int,
    880654386 as core::ffi::c_int,
    881264685 as core::ffi::c_int,
    881875407 as core::ffi::c_int,
    882486553 as core::ffi::c_int,
    883099289 as core::ffi::c_int,
    883711283 as core::ffi::c_int,
    884323700 as core::ffi::c_int,
    884937712 as core::ffi::c_int,
    885550980 as core::ffi::c_int,
    886164672 as core::ffi::c_int,
    886778790 as core::ffi::c_int,
    887394507 as core::ffi::c_int,
    888009477 as core::ffi::c_int,
    888624873 as core::ffi::c_int,
    889241872 as core::ffi::c_int,
    889858122 as core::ffi::c_int,
    890474799 as core::ffi::c_int,
    891093082 as core::ffi::c_int,
    891710615 as core::ffi::c_int,
    892328577 as core::ffi::c_int,
    892946966 as core::ffi::c_int,
    893566965 as core::ffi::c_int,
    894186213 as core::ffi::c_int,
    894805890 as core::ffi::c_int,
    895427180 as core::ffi::c_int,
    896047717 as core::ffi::c_int,
    896668684 as core::ffi::c_int,
    897290081 as core::ffi::c_int,
    897913096 as core::ffi::c_int,
    898535355 as core::ffi::c_int,
    899158046 as core::ffi::c_int,
    899782358 as core::ffi::c_int,
    900405913 as core::ffi::c_int,
    901029900 as core::ffi::c_int,
    901654319 as core::ffi::c_int,
    902280365 as core::ffi::c_int,
    902905651 as core::ffi::c_int,
    903531370 as core::ffi::c_int,
    904158719 as core::ffi::c_int,
    904785306 as core::ffi::c_int,
    905412328 as core::ffi::c_int,
    906039785 as core::ffi::c_int,
    906668875 as core::ffi::c_int,
    907297202 as core::ffi::c_int,
    907925965 as core::ffi::c_int,
    908556365 as core::ffi::c_int,
    909186000 as core::ffi::c_int,
    909816072 as core::ffi::c_int,
    910446581 as core::ffi::c_int,
    911078730 as core::ffi::c_int,
    911710114 as core::ffi::c_int,
    912341935 as core::ffi::c_int,
    912975401 as core::ffi::c_int,
    913608099 as core::ffi::c_int,
    914241235 as core::ffi::c_int,
    914874810 as core::ffi::c_int,
    915510034 as core::ffi::c_int,
    916144489 as core::ffi::c_int,
    916779383 as core::ffi::c_int,
    917415930 as core::ffi::c_int,
    918051705 as core::ffi::c_int,
    918687921 as core::ffi::c_int,
    919325793 as core::ffi::c_int,
    919962891 as core::ffi::c_int,
    920600432 as core::ffi::c_int,
    921238414 as core::ffi::c_int,
    921878057 as core::ffi::c_int,
    922516924 as core::ffi::c_int,
    923156234 as core::ffi::c_int,
    923797209 as core::ffi::c_int,
    924437406 as core::ffi::c_int,
    925078047 as core::ffi::c_int,
    925719132 as core::ffi::c_int,
    926361886 as core::ffi::c_int,
    927003861 as core::ffi::c_int,
    927646281 as core::ffi::c_int,
    928290373 as core::ffi::c_int,
    928933684 as core::ffi::c_int,
    929577441 as core::ffi::c_int,
    930221644 as core::ffi::c_int,
    930867524 as core::ffi::c_int,
    931512622 as core::ffi::c_int,
    932158166 as core::ffi::c_int,
    932805391 as core::ffi::c_int,
    933451831 as core::ffi::c_int,
    934098719 as core::ffi::c_int,
    934746055 as core::ffi::c_int,
    935395077 as core::ffi::c_int,
    936043312 as core::ffi::c_int,
    936691996 as core::ffi::c_int,
    937342369 as core::ffi::c_int,
    937991953 as core::ffi::c_int,
    938641988 as core::ffi::c_int,
    939292472 as core::ffi::c_int,
    939944651 as core::ffi::c_int,
    940596039 as core::ffi::c_int,
    941247878 as core::ffi::c_int,
    941901414 as core::ffi::c_int,
    942554158 as core::ffi::c_int,
    943207354 as core::ffi::c_int,
    943861002 as core::ffi::c_int,
    944516353 as core::ffi::c_int,
    945170909 as core::ffi::c_int,
    945825918 as core::ffi::c_int,
    946482633 as core::ffi::c_int,
    947138552 as core::ffi::c_int,
    947794925 as core::ffi::c_int,
    948451753 as core::ffi::c_int,
    949110291 as core::ffi::c_int,
    949768030 as core::ffi::c_int,
    950426226 as core::ffi::c_int,
    951086135 as core::ffi::c_int,
    951745243 as core::ffi::c_int,
    952404809 as core::ffi::c_int,
    953064832 as core::ffi::c_int,
    953726573 as core::ffi::c_int,
    954387511 as core::ffi::c_int,
    955048908 as core::ffi::c_int,
    955712027 as core::ffi::c_int,
    956374341 as core::ffi::c_int,
    957037115 as core::ffi::c_int,
    957700348 as core::ffi::c_int,
    958365307 as core::ffi::c_int,
    959029460 as core::ffi::c_int,
    959694074 as core::ffi::c_int,
    960360418 as core::ffi::c_int,
    961025954 as core::ffi::c_int,
    961691951 as core::ffi::c_int,
    962358410 as core::ffi::c_int,
    963026603 as core::ffi::c_int,
    963693987 as core::ffi::c_int,
    964361833 as core::ffi::c_int,
    965031418 as core::ffi::c_int,
    965700191 as core::ffi::c_int,
    966369428 as core::ffi::c_int,
    967039128 as core::ffi::c_int,
    967710571 as core::ffi::c_int,
    968381201 as core::ffi::c_int,
    969052296 as core::ffi::c_int,
    969725137 as core::ffi::c_int,
    970397163 as core::ffi::c_int,
    971069654 as core::ffi::c_int,
    971743897 as core::ffi::c_int,
    972417321 as core::ffi::c_int,
    973091213 as core::ffi::c_int,
    973765571 as core::ffi::c_int,
    974441685 as core::ffi::c_int,
    975116980 as core::ffi::c_int,
    975792742 as core::ffi::c_int,
    976470264 as core::ffi::c_int,
    977146964 as core::ffi::c_int,
    977824133 as core::ffi::c_int,
    978501771 as core::ffi::c_int,
    979181174 as core::ffi::c_int,
    979859753 as core::ffi::c_int,
    980538802 as core::ffi::c_int,
    981219619 as core::ffi::c_int,
    981899611 as core::ffi::c_int,
    982580073 as core::ffi::c_int,
    983261008 as core::ffi::c_int,
    983943715 as core::ffi::c_int,
    984625594 as core::ffi::c_int,
    985307946 as core::ffi::c_int,
    985992074 as core::ffi::c_int,
    986675373 as core::ffi::c_int,
    987359145 as core::ffi::c_int,
    988043392 as core::ffi::c_int,
    988729419 as core::ffi::c_int,
    989414615 as core::ffi::c_int,
    990100286 as core::ffi::c_int,
    990787742 as core::ffi::c_int,
    991474364 as core::ffi::c_int,
    992161462 as core::ffi::c_int,
    992849036 as core::ffi::c_int,
    993538401 as core::ffi::c_int,
    994226929 as core::ffi::c_int,
    994915935 as core::ffi::c_int,
    995606734 as core::ffi::c_int,
    996296696 as core::ffi::c_int,
    996987136 as core::ffi::c_int,
    997678055 as core::ffi::c_int,
    998370772 as core::ffi::c_int,
    999062649 as core::ffi::c_int,
    999755006 as core::ffi::c_int,
    1000449165 as core::ffi::c_int,
    1001142483 as core::ffi::c_int,
    1001836281 as core::ffi::c_int,
    1002531886 as core::ffi::c_int,
    1003226647 as core::ffi::c_int,
    1003921889 as core::ffi::c_int,
    1004617614 as core::ffi::c_int,
    1005315149 as core::ffi::c_int,
    1006011839 as core::ffi::c_int,
    1006709012 as core::ffi::c_int,
    1007407999 as core::ffi::c_int,
    1008106140 as core::ffi::c_int,
    1008804764 as core::ffi::c_int,
    1009503872 as core::ffi::c_int,
    1010204800 as core::ffi::c_int,
    1010904879 as core::ffi::c_int,
    1011605442 as core::ffi::c_int,
    1012307830 as core::ffi::c_int,
    1013009365 as core::ffi::c_int,
    1013711388 as core::ffi::c_int,
    1014413896 as core::ffi::c_int,
    1015118233 as core::ffi::c_int,
    1015821717 as core::ffi::c_int,
    1016525688 as core::ffi::c_int,
    1017231492 as core::ffi::c_int,
    1017936440 as core::ffi::c_int,
    1018641876 as core::ffi::c_int,
    1019347801 as core::ffi::c_int,
    1020055565 as core::ffi::c_int,
    1020762470 as core::ffi::c_int,
    1021469865 as core::ffi::c_int,
    1022179101 as core::ffi::c_int,
    1022887478 as core::ffi::c_int,
    1023596346 as core::ffi::c_int,
    1024305704 as core::ffi::c_int,
    1025016910 as core::ffi::c_int,
    1025727253 as core::ffi::c_int,
    1026438089 as core::ffi::c_int,
    1027150775 as core::ffi::c_int,
    1027862597 as core::ffi::c_int,
    1028574913 as core::ffi::c_int,
    1029287722 as core::ffi::c_int,
    1030002386 as core::ffi::c_int,
    1030716185 as core::ffi::c_int,
    1031430478 as core::ffi::c_int,
    1032146630 as core::ffi::c_int,
    1032861915 as core::ffi::c_int,
    1033577694 as core::ffi::c_int,
    1034293970 as core::ffi::c_int,
    1035012111 as core::ffi::c_int,
    1035729381 as core::ffi::c_int,
    1036447148 as core::ffi::c_int,
    1037166784 as core::ffi::c_int,
    1037885547 as core::ffi::c_int,
    1038604809 as core::ffi::c_int,
    1039324569 as core::ffi::c_int,
    1040046202 as core::ffi::c_int,
    1040766961 as core::ffi::c_int,
    1041488219 as core::ffi::c_int,
    1042211355 as core::ffi::c_int,
    1042933614 as core::ffi::c_int,
    1043656374 as core::ffi::c_int,
    1044379635 as core::ffi::c_int,
    1045104778 as core::ffi::c_int,
    1045829042 as core::ffi::c_int,
    1046553809 as core::ffi::c_int,
    1047280462 as core::ffi::c_int,
    1048006234 as core::ffi::c_int,
    1048732509 as core::ffi::c_int,
    1049459287 as core::ffi::c_int,
    1050187958 as core::ffi::c_int,
    1050915745 as core::ffi::c_int,
    1051644036 as core::ffi::c_int,
    1052374224 as core::ffi::c_int,
    1053103526 as core::ffi::c_int,
    1053833333 as core::ffi::c_int,
    1054563647 as core::ffi::c_int,
    1055295861 as core::ffi::c_int,
    1056027188 as core::ffi::c_int,
    1056759022 as core::ffi::c_int,
    1057492761 as core::ffi::c_int,
    1058225610 as core::ffi::c_int,
    1058958967 as core::ffi::c_int,
    1059694233 as core::ffi::c_int,
    1060428608 as core::ffi::c_int,
    1061163492 as core::ffi::c_int,
    1061898885 as core::ffi::c_int,
    1062636193 as core::ffi::c_int,
    1063372607 as core::ffi::c_int,
    1064109531 as core::ffi::c_int,
    1064848373 as core::ffi::c_int,
    1065586320 as core::ffi::c_int,
    1066324778 as core::ffi::c_int,
    1067063748 as core::ffi::c_int,
    1067804642 as core::ffi::c_int,
    1068544637 as core::ffi::c_int,
    1069285146 as core::ffi::c_int,
    1070027582 as core::ffi::c_int,
    1070769118 as core::ffi::c_int,
    1071511168 as core::ffi::c_int,
    1072253732 as core::ffi::c_int,
    1072998229 as core::ffi::c_int,
];
static mut ixheaacd_drc_pow_tbl_1_2_q29: [WORD32; 1000] = [
    536870912 as core::ffi::c_int,
    536499115 as core::ffi::c_int,
    536126866 as core::ffi::c_int,
    535755584 as core::ffi::c_int,
    535384559 as core::ffi::c_int,
    535013791 as core::ffi::c_int,
    534642573 as core::ffi::c_int,
    534272319 as core::ffi::c_int,
    533902321 as core::ffi::c_int,
    533531874 as core::ffi::c_int,
    533162389 as core::ffi::c_int,
    532793160 as core::ffi::c_int,
    532424187 as core::ffi::c_int,
    532054765 as core::ffi::c_int,
    531686303 as core::ffi::c_int,
    531318096 as core::ffi::c_int,
    530949443 as core::ffi::c_int,
    530581746 as core::ffi::c_int,
    530214304 as core::ffi::c_int,
    529847117 as core::ffi::c_int,
    529479484 as core::ffi::c_int,
    529112805 as core::ffi::c_int,
    528746380 as core::ffi::c_int,
    528379511 as core::ffi::c_int,
    528013594 as core::ffi::c_int,
    527647931 as core::ffi::c_int,
    527282520 as core::ffi::c_int,
    526916667 as core::ffi::c_int,
    526551763 as core::ffi::c_int,
    526187112 as core::ffi::c_int,
    525822018 as core::ffi::c_int,
    525457872 as core::ffi::c_int,
    525093979 as core::ffi::c_int,
    524729644 as core::ffi::c_int,
    524366255 as core::ffi::c_int,
    524003117 as core::ffi::c_int,
    523640231 as core::ffi::c_int,
    523276904 as core::ffi::c_int,
    522914521 as core::ffi::c_int,
    522552389 as core::ffi::c_int,
    522189817 as core::ffi::c_int,
    521828187 as core::ffi::c_int,
    521466807 as core::ffi::c_int,
    521105678 as core::ffi::c_int,
    520744110 as core::ffi::c_int,
    520383480 as core::ffi::c_int,
    520023101 as core::ffi::c_int,
    519662284 as core::ffi::c_int,
    519302404 as core::ffi::c_int,
    518942774 as core::ffi::c_int,
    518583392 as core::ffi::c_int,
    518223574 as core::ffi::c_int,
    517864691 as core::ffi::c_int,
    517506056 as core::ffi::c_int,
    517146985 as core::ffi::c_int,
    516788847 as core::ffi::c_int,
    516430957 as core::ffi::c_int,
    516073315 as core::ffi::c_int,
    515715239 as core::ffi::c_int,
    515358092 as core::ffi::c_int,
    515001193 as core::ffi::c_int,
    514643861 as core::ffi::c_int,
    514287456 as core::ffi::c_int,
    513931299 as core::ffi::c_int,
    513575388 as core::ffi::c_int,
    513219044 as core::ffi::c_int,
    512863627 as core::ffi::c_int,
    512508455 as core::ffi::c_int,
    512152852 as core::ffi::c_int,
    511798173 as core::ffi::c_int,
    511443739 as core::ffi::c_int,
    511089551 as core::ffi::c_int,
    510734932 as core::ffi::c_int,
    510381235 as core::ffi::c_int,
    510027782 as core::ffi::c_int,
    509673901 as core::ffi::c_int,
    509320938 as core::ffi::c_int,
    508968220 as core::ffi::c_int,
    508615746 as core::ffi::c_int,
    508262844 as core::ffi::c_int,
    507910858 as core::ffi::c_int,
    507559117 as core::ffi::c_int,
    507206948 as core::ffi::c_int,
    506855694 as core::ffi::c_int,
    506504683 as core::ffi::c_int,
    506153915 as core::ffi::c_int,
    505802721 as core::ffi::c_int,
    505452439 as core::ffi::c_int,
    505102400 as core::ffi::c_int,
    504751936 as core::ffi::c_int,
    504402382 as core::ffi::c_int,
    504053070 as core::ffi::c_int,
    503704000 as core::ffi::c_int,
    503354506 as core::ffi::c_int,
    503005920 as core::ffi::c_int,
    502657575 as core::ffi::c_int,
    502308807 as core::ffi::c_int,
    501960945 as core::ffi::c_int,
    501613323 as core::ffi::c_int,
    501265280 as core::ffi::c_int,
    500918141 as core::ffi::c_int,
    500571242 as core::ffi::c_int,
    500224583 as core::ffi::c_int,
    499877503 as core::ffi::c_int,
    499531325 as core::ffi::c_int,
    499185386 as core::ffi::c_int,
    498839027 as core::ffi::c_int,
    498493568 as core::ffi::c_int,
    498148348 as core::ffi::c_int,
    497803367 as core::ffi::c_int,
    497457967 as core::ffi::c_int,
    497113465 as core::ffi::c_int,
    496769200 as core::ffi::c_int,
    496424518 as core::ffi::c_int,
    496080731 as core::ffi::c_int,
    495737182 as core::ffi::c_int,
    495393871 as core::ffi::c_int,
    495050143 as core::ffi::c_int,
    494707308 as core::ffi::c_int,
    494364710 as core::ffi::c_int,
    494021696 as core::ffi::c_int,
    493679573 as core::ffi::c_int,
    493337687 as core::ffi::c_int,
    492996037 as core::ffi::c_int,
    492653973 as core::ffi::c_int,
    492312797 as core::ffi::c_int,
    491971857 as core::ffi::c_int,
    491630504 as core::ffi::c_int,
    491290037 as core::ffi::c_int,
    490949805 as core::ffi::c_int,
    490609809 as core::ffi::c_int,
    490269401 as core::ffi::c_int,
    489929876 as core::ffi::c_int,
    489590587 as core::ffi::c_int,
    489250886 as core::ffi::c_int,
    488912067 as core::ffi::c_int,
    488573482 as core::ffi::c_int,
    488235132 as core::ffi::c_int,
    487896371 as core::ffi::c_int,
    487558490 as core::ffi::c_int,
    487220843 as core::ffi::c_int,
    486882786 as core::ffi::c_int,
    486545606 as core::ffi::c_int,
    486208661 as core::ffi::c_int,
    485871948 as core::ffi::c_int,
    485534827 as core::ffi::c_int,
    485198581 as core::ffi::c_int,
    484862569 as core::ffi::c_int,
    484526148 as core::ffi::c_int,
    484190601 as core::ffi::c_int,
    483855286 as core::ffi::c_int,
    483520203 as core::ffi::c_int,
    483184714 as core::ffi::c_int,
    482850096 as core::ffi::c_int,
    482515709 as core::ffi::c_int,
    482180917 as core::ffi::c_int,
    481846994 as core::ffi::c_int,
    481513302 as core::ffi::c_int,
    481179205 as core::ffi::c_int,
    480845976 as core::ffi::c_int,
    480512977 as core::ffi::c_int,
    480180209 as core::ffi::c_int,
    479847037 as core::ffi::c_int,
    479514730 as core::ffi::c_int,
    479182654 as core::ffi::c_int,
    478850174 as core::ffi::c_int,
    478518557 as core::ffi::c_int,
    478187171 as core::ffi::c_int,
    477856013 as core::ffi::c_int,
    477524454 as core::ffi::c_int,
    477193756 as core::ffi::c_int,
    476863286 as core::ffi::c_int,
    476532416 as core::ffi::c_int,
    476202404 as core::ffi::c_int,
    475872622 as core::ffi::c_int,
    475543067 as core::ffi::c_int,
    475213113 as core::ffi::c_int,
    474884015 as core::ffi::c_int,
    474555145 as core::ffi::c_int,
    474225876 as core::ffi::c_int,
    473897462 as core::ffi::c_int,
    473569276 as core::ffi::c_int,
    473241317 as core::ffi::c_int,
    472912959 as core::ffi::c_int,
    472585454 as core::ffi::c_int,
    472258176 as core::ffi::c_int,
    471930501 as core::ffi::c_int,
    471603677 as core::ffi::c_int,
    471277079 as core::ffi::c_int,
    470950707 as core::ffi::c_int,
    470623939 as core::ffi::c_int,
    470298019 as core::ffi::c_int,
    469972325 as core::ffi::c_int,
    469646236 as core::ffi::c_int,
    469320994 as core::ffi::c_int,
    468995977 as core::ffi::c_int,
    468671184 as core::ffi::c_int,
    468345998 as core::ffi::c_int,
    468021656 as core::ffi::c_int,
    467697539 as core::ffi::c_int,
    467373028 as core::ffi::c_int,
    467049359 as core::ffi::c_int,
    466725915 as core::ffi::c_int,
    466402695 as core::ffi::c_int,
    466079083 as core::ffi::c_int,
    465756311 as core::ffi::c_int,
    465433762 as core::ffi::c_int,
    465110822 as core::ffi::c_int,
    464788720 as core::ffi::c_int,
    464466842 as core::ffi::c_int,
    464145186 as core::ffi::c_int,
    463823140 as core::ffi::c_int,
    463501930 as core::ffi::c_int,
    463180943 as core::ffi::c_int,
    462859566 as core::ffi::c_int,
    462539024 as core::ffi::c_int,
    462218703 as core::ffi::c_int,
    461898604 as core::ffi::c_int,
    461578117 as core::ffi::c_int,
    461258462 as core::ffi::c_int,
    460939028 as core::ffi::c_int,
    460619207 as core::ffi::c_int,
    460300216 as core::ffi::c_int,
    459981446 as core::ffi::c_int,
    459662289 as core::ffi::c_int,
    459343960 as core::ffi::c_int,
    459025852 as core::ffi::c_int,
    458707965 as core::ffi::c_int,
    458389691 as core::ffi::c_int,
    458072244 as core::ffi::c_int,
    457755017 as core::ffi::c_int,
    457437405 as core::ffi::c_int,
    457120617 as core::ffi::c_int,
    456804049 as core::ffi::c_int,
    456487700 as core::ffi::c_int,
    456170967 as core::ffi::c_int,
    455855057 as core::ffi::c_int,
    455539365 as core::ffi::c_int,
    455223290 as core::ffi::c_int,
    454908036 as core::ffi::c_int,
    454593000 as core::ffi::c_int,
    454278182 as core::ffi::c_int,
    453962983 as core::ffi::c_int,
    453648601 as core::ffi::c_int,
    453334438 as core::ffi::c_int,
    453019893 as core::ffi::c_int,
    452706164 as core::ffi::c_int,
    452392653 as core::ffi::c_int,
    452079359 as core::ffi::c_int,
    451765685 as core::ffi::c_int,
    451452825 as core::ffi::c_int,
    451140182 as core::ffi::c_int,
    450827160 as core::ffi::c_int,
    450514950 as core::ffi::c_int,
    450202956 as core::ffi::c_int,
    449891179 as core::ffi::c_int,
    449579023 as core::ffi::c_int,
    449267678 as core::ffi::c_int,
    448956548 as core::ffi::c_int,
    448645040 as core::ffi::c_int,
    448334342 as core::ffi::c_int,
    448023858 as core::ffi::c_int,
    447713590 as core::ffi::c_int,
    447402945 as core::ffi::c_int,
    447093107 as core::ffi::c_int,
    446783483 as core::ffi::c_int,
    446473483 as core::ffi::c_int,
    446164288 as core::ffi::c_int,
    445855308 as core::ffi::c_int,
    445546541 as core::ffi::c_int,
    445237400 as core::ffi::c_int,
    444929061 as core::ffi::c_int,
    444620936 as core::ffi::c_int,
    444312437 as core::ffi::c_int,
    444004738 as core::ffi::c_int,
    443697253 as core::ffi::c_int,
    443389981 as core::ffi::c_int,
    443082336 as core::ffi::c_int,
    442775490 as core::ffi::c_int,
    442468856 as core::ffi::c_int,
    442161850 as core::ffi::c_int,
    441855641 as core::ffi::c_int,
    441549645 as core::ffi::c_int,
    441243276 as core::ffi::c_int,
    440937704 as core::ffi::c_int,
    440632343 as core::ffi::c_int,
    440327193 as core::ffi::c_int,
    440021673 as core::ffi::c_int,
    439716946 as core::ffi::c_int,
    439412431 as core::ffi::c_int,
    439107545 as core::ffi::c_int,
    438803452 as core::ffi::c_int,
    438499569 as core::ffi::c_int,
    438195896 as core::ffi::c_int,
    437891855 as core::ffi::c_int,
    437588603 as core::ffi::c_int,
    437285562 as core::ffi::c_int,
    436982152 as core::ffi::c_int,
    436679530 as core::ffi::c_int,
    436377118 as core::ffi::c_int,
    436074915 as core::ffi::c_int,
    435772346 as core::ffi::c_int,
    435470562 as core::ffi::c_int,
    435168987 as core::ffi::c_int,
    434867046 as core::ffi::c_int,
    434565889 as core::ffi::c_int,
    434264941 as core::ffi::c_int,
    433964201 as core::ffi::c_int,
    433663096 as core::ffi::c_int,
    433362772 as core::ffi::c_int,
    433062657 as core::ffi::c_int,
    432762178 as core::ffi::c_int,
    432462478 as core::ffi::c_int,
    432162987 as core::ffi::c_int,
    431863702 as core::ffi::c_int,
    431564055 as core::ffi::c_int,
    431265185 as core::ffi::c_int,
    430966523 as core::ffi::c_int,
    430667498 as core::ffi::c_int,
    430369249 as core::ffi::c_int,
    430071207 as core::ffi::c_int,
    429773371 as core::ffi::c_int,
    429475174 as core::ffi::c_int,
    429177751 as core::ffi::c_int,
    428880534 as core::ffi::c_int,
    428582956 as core::ffi::c_int,
    428286151 as core::ffi::c_int,
    427989552 as core::ffi::c_int,
    427693157 as core::ffi::c_int,
    427396403 as core::ffi::c_int,
    427100420 as core::ffi::c_int,
    426804642 as core::ffi::c_int,
    426508504 as core::ffi::c_int,
    426213136 as core::ffi::c_int,
    425917972 as core::ffi::c_int,
    425623013 as core::ffi::c_int,
    425327695 as core::ffi::c_int,
    425033144 as core::ffi::c_int,
    424738798 as core::ffi::c_int,
    424444094 as core::ffi::c_int,
    424150155 as core::ffi::c_int,
    423856420 as core::ffi::c_int,
    423562888 as core::ffi::c_int,
    423269000 as core::ffi::c_int,
    422975875 as core::ffi::c_int,
    422682953 as core::ffi::c_int,
    422389675 as core::ffi::c_int,
    422097159 as core::ffi::c_int,
    421804845 as core::ffi::c_int,
    421512177 as core::ffi::c_int,
    421220269 as core::ffi::c_int,
    420928563 as core::ffi::c_int,
    420637058 as core::ffi::c_int,
    420345200 as core::ffi::c_int,
    420054100 as core::ffi::c_int,
    419763202 as core::ffi::c_int,
    419471950 as core::ffi::c_int,
    419181454 as core::ffi::c_int,
    418891160 as core::ffi::c_int,
    418601067 as core::ffi::c_int,
    418310622 as core::ffi::c_int,
    418020930 as core::ffi::c_int,
    417731440 as core::ffi::c_int,
    417441598 as core::ffi::c_int,
    417152508 as core::ffi::c_int,
    416863619 as core::ffi::c_int,
    416574930 as core::ffi::c_int,
    416285891 as core::ffi::c_int,
    415997602 as core::ffi::c_int,
    415709512 as core::ffi::c_int,
    415421073 as core::ffi::c_int,
    415133383 as core::ffi::c_int,
    414845892 as core::ffi::c_int,
    414558600 as core::ffi::c_int,
    414270960 as core::ffi::c_int,
    413984066 as core::ffi::c_int,
    413697371 as core::ffi::c_int,
    413410328 as core::ffi::c_int,
    413124031 as core::ffi::c_int,
    412837931 as core::ffi::c_int,
    412552030 as core::ffi::c_int,
    412265782 as core::ffi::c_int,
    411980277 as core::ffi::c_int,
    411694970 as core::ffi::c_int,
    411409316 as core::ffi::c_int,
    411124404 as core::ffi::c_int,
    410839690 as core::ffi::c_int,
    410555172 as core::ffi::c_int,
    410270309 as core::ffi::c_int,
    409986186 as core::ffi::c_int,
    409702260 as core::ffi::c_int,
    409417989 as core::ffi::c_int,
    409134456 as core::ffi::c_int,
    408851120 as core::ffi::c_int,
    408567980 as core::ffi::c_int,
    408284496 as core::ffi::c_int,
    408001748 as core::ffi::c_int,
    407719196 as core::ffi::c_int,
    407436301 as core::ffi::c_int,
    407154140 as core::ffi::c_int,
    406872175 as core::ffi::c_int,
    406590406 as core::ffi::c_int,
    406308294 as core::ffi::c_int,
    406026914 as core::ffi::c_int,
    405745730 as core::ffi::c_int,
    405464204 as core::ffi::c_int,
    405183410 as core::ffi::c_int,
    404902809 as core::ffi::c_int,
    404621868 as core::ffi::c_int,
    404341657 as core::ffi::c_int,
    404061640 as core::ffi::c_int,
    403781816 as core::ffi::c_int,
    403501653 as core::ffi::c_int,
    403222218 as core::ffi::c_int,
    402942976 as core::ffi::c_int,
    402663395 as core::ffi::c_int,
    402384540 as core::ffi::c_int,
    402105878 as core::ffi::c_int,
    401827409 as core::ffi::c_int,
    401548602 as core::ffi::c_int,
    401270518 as core::ffi::c_int,
    400992628 as core::ffi::c_int,
    400714400 as core::ffi::c_int,
    400436895 as core::ffi::c_int,
    400159582 as core::ffi::c_int,
    399882461 as core::ffi::c_int,
    399605003 as core::ffi::c_int,
    399328266 as core::ffi::c_int,
    399051721 as core::ffi::c_int,
    398774839 as core::ffi::c_int,
    398498677 as core::ffi::c_int,
    398222706 as core::ffi::c_int,
    397946927 as core::ffi::c_int,
    397670812 as core::ffi::c_int,
    397395415 as core::ffi::c_int,
    397120208 as core::ffi::c_int,
    396844667 as core::ffi::c_int,
    396569841 as core::ffi::c_int,
    396295206 as core::ffi::c_int,
    396020761 as core::ffi::c_int,
    395745983 as core::ffi::c_int,
    395471919 as core::ffi::c_int,
    395198044 as core::ffi::c_int,
    394923836 as core::ffi::c_int,
    394650341 as core::ffi::c_int,
    394377035 as core::ffi::c_int,
    394103919 as core::ffi::c_int,
    393830471 as core::ffi::c_int,
    393557733 as core::ffi::c_int,
    393285184 as core::ffi::c_int,
    393012304 as core::ffi::c_int,
    392740132 as core::ffi::c_int,
    392468149 as core::ffi::c_int,
    392196355 as core::ffi::c_int,
    391924230 as core::ffi::c_int,
    391652812 as core::ffi::c_int,
    391381582 as core::ffi::c_int,
    391110023 as core::ffi::c_int,
    390839169 as core::ffi::c_int,
    390568502 as core::ffi::c_int,
    390298023 as core::ffi::c_int,
    390027216 as core::ffi::c_int,
    389757112 as core::ffi::c_int,
    389487195 as core::ffi::c_int,
    389216950 as core::ffi::c_int,
    388947407 as core::ffi::c_int,
    388678050 as core::ffi::c_int,
    388408881 as core::ffi::c_int,
    388139384 as core::ffi::c_int,
    387870587 as core::ffi::c_int,
    387601977 as core::ffi::c_int,
    387333040 as core::ffi::c_int,
    387064801 as core::ffi::c_int,
    386796749 as core::ffi::c_int,
    386528371 as core::ffi::c_int,
    386260690 as core::ffi::c_int,
    385993194 as core::ffi::c_int,
    385725883 as core::ffi::c_int,
    385458248 as core::ffi::c_int,
    385191308 as core::ffi::c_int,
    384924553 as core::ffi::c_int,
    384657474 as core::ffi::c_int,
    384391089 as core::ffi::c_int,
    384124887 as core::ffi::c_int,
    383858871 as core::ffi::c_int,
    383592531 as core::ffi::c_int,
    383326883 as core::ffi::c_int,
    383061419 as core::ffi::c_int,
    382795633 as core::ffi::c_int,
    382530537 as core::ffi::c_int,
    382265624 as core::ffi::c_int,
    382000895 as core::ffi::c_int,
    381735845 as core::ffi::c_int,
    381471483 as core::ffi::c_int,
    381207303 as core::ffi::c_int,
    380942804 as core::ffi::c_int,
    380678991 as core::ffi::c_int,
    380415360 as core::ffi::c_int,
    380151913 as core::ffi::c_int,
    379888145 as core::ffi::c_int,
    379625062 as core::ffi::c_int,
    379362162 as core::ffi::c_int,
    379098943 as core::ffi::c_int,
    378836406 as core::ffi::c_int,
    378574052 as core::ffi::c_int,
    378311880 as core::ffi::c_int,
    378049389 as core::ffi::c_int,
    377787580 as core::ffi::c_int,
    377525952 as core::ffi::c_int,
    377264006 as core::ffi::c_int,
    377002741 as core::ffi::c_int,
    376741656 as core::ffi::c_int,
    376480753 as core::ffi::c_int,
    376219533 as core::ffi::c_int,
    375958991 as core::ffi::c_int,
    375698629 as core::ffi::c_int,
    375437951 as core::ffi::c_int,
    375177951 as core::ffi::c_int,
    374918130 as core::ffi::c_int,
    374658489 as core::ffi::c_int,
    374398533 as core::ffi::c_int,
    374139252 as core::ffi::c_int,
    373880151 as core::ffi::c_int,
    373620735 as core::ffi::c_int,
    373361993 as core::ffi::c_int,
    373103430 as core::ffi::c_int,
    372844553 as core::ffi::c_int,
    372586348 as core::ffi::c_int,
    372328322 as core::ffi::c_int,
    372070475 as core::ffi::c_int,
    371812315 as core::ffi::c_int,
    371554825 as core::ffi::c_int,
    371297513 as core::ffi::c_int,
    371039889 as core::ffi::c_int,
    370782934 as core::ffi::c_int,
    370526157 as core::ffi::c_int,
    370269558 as core::ffi::c_int,
    370012648 as core::ffi::c_int,
    369756404 as core::ffi::c_int,
    369500338 as core::ffi::c_int,
    369243961 as core::ffi::c_int,
    368988250 as core::ffi::c_int,
    368732715 as core::ffi::c_int,
    368477358 as core::ffi::c_int,
    368221691 as core::ffi::c_int,
    367966688 as core::ffi::c_int,
    367711861 as core::ffi::c_int,
    367456725 as core::ffi::c_int,
    367202252 as core::ffi::c_int,
    366947954 as core::ffi::c_int,
    366693833 as core::ffi::c_int,
    366439403 as core::ffi::c_int,
    366185634 as core::ffi::c_int,
    365932041 as core::ffi::c_int,
    365678140 as core::ffi::c_int,
    365424898 as core::ffi::c_int,
    365171832 as core::ffi::c_int,
    364918941 as core::ffi::c_int,
    364665743 as core::ffi::c_int,
    364413202 as core::ffi::c_int,
    364160836 as core::ffi::c_int,
    363908164 as core::ffi::c_int,
    363656148 as core::ffi::c_int,
    363404306 as core::ffi::c_int,
    363152639 as core::ffi::c_int,
    362900667 as core::ffi::c_int,
    362649348 as core::ffi::c_int,
    362398204 as core::ffi::c_int,
    362146755 as core::ffi::c_int,
    361895959 as core::ffi::c_int,
    361645336 as core::ffi::c_int,
    361394887 as core::ffi::c_int,
    361144134 as core::ffi::c_int,
    360894032 as core::ffi::c_int,
    360644103 as core::ffi::c_int,
    360393871 as core::ffi::c_int,
    360144289 as core::ffi::c_int,
    359894880 as core::ffi::c_int,
    359645643 as core::ffi::c_int,
    359396104 as core::ffi::c_int,
    359147212 as core::ffi::c_int,
    358898493 as core::ffi::c_int,
    358649472 as core::ffi::c_int,
    358401098 as core::ffi::c_int,
    358152896 as core::ffi::c_int,
    357904865 as core::ffi::c_int,
    357656534 as core::ffi::c_int,
    357408847 as core::ffi::c_int,
    357161332 as core::ffi::c_int,
    356913517 as core::ffi::c_int,
    356666345 as core::ffi::c_int,
    356419344 as core::ffi::c_int,
    356172514 as core::ffi::c_int,
    355925384 as core::ffi::c_int,
    355678897 as core::ffi::c_int,
    355432580 as core::ffi::c_int,
    355185964 as core::ffi::c_int,
    354939988 as core::ffi::c_int,
    354694182 as core::ffi::c_int,
    354448547 as core::ffi::c_int,
    354202614 as core::ffi::c_int,
    353957319 as core::ffi::c_int,
    353712195 as core::ffi::c_int,
    353466772 as core::ffi::c_int,
    353221987 as core::ffi::c_int,
    352977371 as core::ffi::c_int,
    352732459 as core::ffi::c_int,
    352488182 as core::ffi::c_int,
    352244075 as core::ffi::c_int,
    352000137 as core::ffi::c_int,
    351755902 as core::ffi::c_int,
    351512302 as core::ffi::c_int,
    351268870 as core::ffi::c_int,
    351025143 as core::ffi::c_int,
    350782049 as core::ffi::c_int,
    350539123 as core::ffi::c_int,
    350296365 as core::ffi::c_int,
    350053313 as core::ffi::c_int,
    349810892 as core::ffi::c_int,
    349568639 as core::ffi::c_int,
    349326091 as core::ffi::c_int,
    349084174 as core::ffi::c_int,
    348842424 as core::ffi::c_int,
    348600841 as core::ffi::c_int,
    348358965 as core::ffi::c_int,
    348117717 as core::ffi::c_int,
    347876636 as core::ffi::c_int,
    347635263 as core::ffi::c_int,
    347394516 as core::ffi::c_int,
    347153937 as core::ffi::c_int,
    346913523 as core::ffi::c_int,
    346672818 as core::ffi::c_int,
    346432738 as core::ffi::c_int,
    346192824 as core::ffi::c_int,
    345952619 as core::ffi::c_int,
    345713038 as core::ffi::c_int,
    345473622 as core::ffi::c_int,
    345234373 as core::ffi::c_int,
    344994832 as core::ffi::c_int,
    344755914 as core::ffi::c_int,
    344517162 as core::ffi::c_int,
    344278119 as core::ffi::c_int,
    344039698 as core::ffi::c_int,
    343801441 as core::ffi::c_int,
    343563349 as core::ffi::c_int,
    343324969 as core::ffi::c_int,
    343087207 as core::ffi::c_int,
    342849610 as core::ffi::c_int,
    342611725 as core::ffi::c_int,
    342374457 as core::ffi::c_int,
    342137354 as core::ffi::c_int,
    341899962 as core::ffi::c_int,
    341663188 as core::ffi::c_int,
    341426577 as core::ffi::c_int,
    341190130 as core::ffi::c_int,
    340953396 as core::ffi::c_int,
    340717277 as core::ffi::c_int,
    340481321 as core::ffi::c_int,
    340245079 as core::ffi::c_int,
    340009450 as core::ffi::c_int,
    339773984 as core::ffi::c_int,
    339538682 as core::ffi::c_int,
    339303094 as core::ffi::c_int,
    339068117 as core::ffi::c_int,
    338833304 as core::ffi::c_int,
    338598205 as core::ffi::c_int,
    338363717 as core::ffi::c_int,
    338129391 as core::ffi::c_int,
    337895227 as core::ffi::c_int,
    337660780 as core::ffi::c_int,
    337426941 as core::ffi::c_int,
    337193263 as core::ffi::c_int,
    336959303 as core::ffi::c_int,
    336725949 as core::ffi::c_int,
    336492758 as core::ffi::c_int,
    336259728 as core::ffi::c_int,
    336026415 as core::ffi::c_int,
    335793707 as core::ffi::c_int,
    335561161 as core::ffi::c_int,
    335328333 as core::ffi::c_int,
    335096109 as core::ffi::c_int,
    334864046 as core::ffi::c_int,
    334632144 as core::ffi::c_int,
    334399960 as core::ffi::c_int,
    334168380 as core::ffi::c_int,
    333936959 as core::ffi::c_int,
    333705258 as core::ffi::c_int,
    333474158 as core::ffi::c_int,
    333243218 as core::ffi::c_int,
    333012438 as core::ffi::c_int,
    332781379 as core::ffi::c_int,
    332550919 as core::ffi::c_int,
    332320618 as core::ffi::c_int,
    332090038 as core::ffi::c_int,
    331860057 as core::ffi::c_int,
    331630235 as core::ffi::c_int,
    331400573 as core::ffi::c_int,
    331170631 as core::ffi::c_int,
    330941287 as core::ffi::c_int,
    330712101 as core::ffi::c_int,
    330482637 as core::ffi::c_int,
    330253769 as core::ffi::c_int,
    330025060 as core::ffi::c_int,
    329796509 as core::ffi::c_int,
    329567680 as core::ffi::c_int,
    329339446 as core::ffi::c_int,
    329111369 as core::ffi::c_int,
    328883016 as core::ffi::c_int,
    328655256 as core::ffi::c_int,
    328427654 as core::ffi::c_int,
    328200209 as core::ffi::c_int,
    327972488 as core::ffi::c_int,
    327745358 as core::ffi::c_int,
    327518386 as core::ffi::c_int,
    327291138 as core::ffi::c_int,
    327064480 as core::ffi::c_int,
    326837979 as core::ffi::c_int,
    326611635 as core::ffi::c_int,
    326385017 as core::ffi::c_int,
    326158986 as core::ffi::c_int,
    325933113 as core::ffi::c_int,
    325706965 as core::ffi::c_int,
    325481404 as core::ffi::c_int,
    325255999 as core::ffi::c_int,
    325030751 as core::ffi::c_int,
    324805229 as core::ffi::c_int,
    324580293 as core::ffi::c_int,
    324355513 as core::ffi::c_int,
    324130459 as core::ffi::c_int,
    323905990 as core::ffi::c_int,
    323681677 as core::ffi::c_int,
    323457091 as core::ffi::c_int,
    323233088 as core::ffi::c_int,
    323009241 as core::ffi::c_int,
    322785548 as core::ffi::c_int,
    322561584 as core::ffi::c_int,
    322338202 as core::ffi::c_int,
    322114974 as core::ffi::c_int,
    321891476 as core::ffi::c_int,
    321668557 as core::ffi::c_int,
    321445793 as core::ffi::c_int,
    321223183 as core::ffi::c_int,
    321000303 as core::ffi::c_int,
    320778002 as core::ffi::c_int,
    320555855 as core::ffi::c_int,
    320333438 as core::ffi::c_int,
    320111599 as core::ffi::c_int,
    319889913 as core::ffi::c_int,
    319668381 as core::ffi::c_int,
    319446579 as core::ffi::c_int,
    319225354 as core::ffi::c_int,
    319004282 as core::ffi::c_int,
    318782942 as core::ffi::c_int,
    318562176 as core::ffi::c_int,
    318341563 as core::ffi::c_int,
    318121103 as core::ffi::c_int,
    317900376 as core::ffi::c_int,
    317680221 as core::ffi::c_int,
    317460219 as core::ffi::c_int,
    317239950 as core::ffi::c_int,
    317020253 as core::ffi::c_int,
    316800708 as core::ffi::c_int,
    316581315 as core::ffi::c_int,
    316361656 as core::ffi::c_int,
    316142567 as core::ffi::c_int,
    315923630 as core::ffi::c_int,
    315704427 as core::ffi::c_int,
    315485794 as core::ffi::c_int,
    315267311 as core::ffi::c_int,
    315048981 as core::ffi::c_int,
    314830385 as core::ffi::c_int,
    314612356 as core::ffi::c_int,
    314394479 as core::ffi::c_int,
    314176337 as core::ffi::c_int,
    313958761 as core::ffi::c_int,
    313741337 as core::ffi::c_int,
    313523648 as core::ffi::c_int,
    313306525 as core::ffi::c_int,
    313089551 as core::ffi::c_int,
    312872729 as core::ffi::c_int,
    312655643 as core::ffi::c_int,
    312439120 as core::ffi::c_int,
    312222748 as core::ffi::c_int,
    312006113 as core::ffi::c_int,
    311790040 as core::ffi::c_int,
    311574117 as core::ffi::c_int,
    311358344 as core::ffi::c_int,
    311142309 as core::ffi::c_int,
    310926835 as core::ffi::c_int,
    310711510 as core::ffi::c_int,
    310495923 as core::ffi::c_int,
    310280896 as core::ffi::c_int,
    310066019 as core::ffi::c_int,
    309851290 as core::ffi::c_int,
    309636300 as core::ffi::c_int,
    309421869 as core::ffi::c_int,
    309207586 as core::ffi::c_int,
    308993043 as core::ffi::c_int,
    308779057 as core::ffi::c_int,
    308565219 as core::ffi::c_int,
    308351530 as core::ffi::c_int,
    308137581 as core::ffi::c_int,
    307924187 as core::ffi::c_int,
    307710942 as core::ffi::c_int,
    307497437 as core::ffi::c_int,
    307284487 as core::ffi::c_int,
    307071684 as core::ffi::c_int,
    306859029 as core::ffi::c_int,
    306646116 as core::ffi::c_int,
    306433755 as core::ffi::c_int,
    306221542 as core::ffi::c_int,
    306009071 as core::ffi::c_int,
    305797151 as core::ffi::c_int,
    305585378 as core::ffi::c_int,
    305373753 as core::ffi::c_int,
    305161870 as core::ffi::c_int,
    304950537 as core::ffi::c_int,
    304739351 as core::ffi::c_int,
    304527908 as core::ffi::c_int,
    304317014 as core::ffi::c_int,
    304106267 as core::ffi::c_int,
    303895665 as core::ffi::c_int,
    303684808 as core::ffi::c_int,
    303474498 as core::ffi::c_int,
    303264334 as core::ffi::c_int,
    303053915 as core::ffi::c_int,
    302844042 as core::ffi::c_int,
    302634314 as core::ffi::c_int,
    302424732 as core::ffi::c_int,
    302214895 as core::ffi::c_int,
    302005603 as core::ffi::c_int,
    301796456 as core::ffi::c_int,
    301587056 as core::ffi::c_int,
    301378199 as core::ffi::c_int,
    301169486 as core::ffi::c_int,
    300960918 as core::ffi::c_int,
    300752097 as core::ffi::c_int,
    300543819 as core::ffi::c_int,
    300335684 as core::ffi::c_int,
    300127297 as core::ffi::c_int,
    299919451 as core::ffi::c_int,
    299711748 as core::ffi::c_int,
    299504190 as core::ffi::c_int,
    299296380 as core::ffi::c_int,
    299089109 as core::ffi::c_int,
    298881982 as core::ffi::c_int,
    298674603 as core::ffi::c_int,
    298467763 as core::ffi::c_int,
    298261067 as core::ffi::c_int,
    298054513 as core::ffi::c_int,
    297847708 as core::ffi::c_int,
    297641441 as core::ffi::c_int,
    297435316 as core::ffi::c_int,
    297228942 as core::ffi::c_int,
    297023103 as core::ffi::c_int,
    296817406 as core::ffi::c_int,
    296611460 as core::ffi::c_int,
    296406049 as core::ffi::c_int,
    296200780 as core::ffi::c_int,
    295995653 as core::ffi::c_int,
    295790277 as core::ffi::c_int,
    295585435 as core::ffi::c_int,
    295380734 as core::ffi::c_int,
    295175785 as core::ffi::c_int,
    294971367 as core::ffi::c_int,
    294767092 as core::ffi::c_int,
    294562958 as core::ffi::c_int,
    294358576 as core::ffi::c_int,
    294154725 as core::ffi::c_int,
    293951015 as core::ffi::c_int,
    293747058 as core::ffi::c_int,
    293543630 as core::ffi::c_int,
    293340343 as core::ffi::c_int,
    293137197 as core::ffi::c_int,
    292933805 as core::ffi::c_int,
    292730940 as core::ffi::c_int,
    292528217 as core::ffi::c_int,
    292325247 as core::ffi::c_int,
    292122804 as core::ffi::c_int,
    291920501 as core::ffi::c_int,
    291718338 as core::ffi::c_int,
    291515930 as core::ffi::c_int,
    291314047 as core::ffi::c_int,
    291112305 as core::ffi::c_int,
    290910317 as core::ffi::c_int,
    290708854 as core::ffi::c_int,
    290507530 as core::ffi::c_int,
    290306346 as core::ffi::c_int,
    290104918 as core::ffi::c_int,
    289904013 as core::ffi::c_int,
    289703246 as core::ffi::c_int,
    289502236 as core::ffi::c_int,
    289301748 as core::ffi::c_int,
    289101399 as core::ffi::c_int,
    288901189 as core::ffi::c_int,
    288700736 as core::ffi::c_int,
    288500803 as core::ffi::c_int,
    288301008 as core::ffi::c_int,
    288100971 as core::ffi::c_int,
    287901454 as core::ffi::c_int,
    287702074 as core::ffi::c_int,
    287502453 as core::ffi::c_int,
    287303350 as core::ffi::c_int,
    287104385 as core::ffi::c_int,
    286905557 as core::ffi::c_int,
    286706488 as core::ffi::c_int,
    286507937 as core::ffi::c_int,
    286309522 as core::ffi::c_int,
    286110867 as core::ffi::c_int,
    285912728 as core::ffi::c_int,
    285714725 as core::ffi::c_int,
    285516860 as core::ffi::c_int,
    285318755 as core::ffi::c_int,
    285121164 as core::ffi::c_int,
    284923710 as core::ffi::c_int,
    284726017 as core::ffi::c_int,
    284528837 as core::ffi::c_int,
    284331793 as core::ffi::c_int,
    284134885 as core::ffi::c_int,
    283937739 as core::ffi::c_int,
    283741105 as core::ffi::c_int,
    283544606 as core::ffi::c_int,
    283347870 as core::ffi::c_int,
    283151644 as core::ffi::c_int,
    282955554 as core::ffi::c_int,
    282759600 as core::ffi::c_int,
    282563407 as core::ffi::c_int,
    282367725 as core::ffi::c_int,
    282172178 as core::ffi::c_int,
    281976393 as core::ffi::c_int,
    281781117 as core::ffi::c_int,
    281585976 as core::ffi::c_int,
    281390970 as core::ffi::c_int,
    281195728 as core::ffi::c_int,
    281000992 as core::ffi::c_int,
    280806392 as core::ffi::c_int,
    280611555 as core::ffi::c_int,
    280417224 as core::ffi::c_int,
    280223028 as core::ffi::c_int,
    280028966 as core::ffi::c_int,
    279834668 as core::ffi::c_int,
    279640875 as core::ffi::c_int,
    279447217 as core::ffi::c_int,
    279253323 as core::ffi::c_int,
    279059933 as core::ffi::c_int,
    278866676 as core::ffi::c_int,
    278673554 as core::ffi::c_int,
    278480197 as core::ffi::c_int,
    278287342 as core::ffi::c_int,
    278094620 as core::ffi::c_int,
    277901665 as core::ffi::c_int,
    277709211 as core::ffi::c_int,
    277516890 as core::ffi::c_int,
    277324702 as core::ffi::c_int,
    277132281 as core::ffi::c_int,
    276940359 as core::ffi::c_int,
    276748571 as core::ffi::c_int,
    276556549 as core::ffi::c_int,
    276365027 as core::ffi::c_int,
    276173637 as core::ffi::c_int,
    275982379 as core::ffi::c_int,
    275790889 as core::ffi::c_int,
    275599897 as core::ffi::c_int,
    275409037 as core::ffi::c_int,
    275217945 as core::ffi::c_int,
    275027349 as core::ffi::c_int,
    274836885 as core::ffi::c_int,
    274646553 as core::ffi::c_int,
    274455990 as core::ffi::c_int,
    274265922 as core::ffi::c_int,
    274075986 as core::ffi::c_int,
    273885819 as core::ffi::c_int,
    273696146 as core::ffi::c_int,
    273506604 as core::ffi::c_int,
    273317193 as core::ffi::c_int,
    273127553 as core::ffi::c_int,
    272938405 as core::ffi::c_int,
    272749388 as core::ffi::c_int,
    272560141 as core::ffi::c_int,
    272371386 as core::ffi::c_int,
    272182762 as core::ffi::c_int,
    271993908 as core::ffi::c_int,
    271805545 as core::ffi::c_int,
    271617313 as core::ffi::c_int,
    271429211 as core::ffi::c_int,
    271240880 as core::ffi::c_int,
    271053039 as core::ffi::c_int,
    270865327 as core::ffi::c_int,
    270677388 as core::ffi::c_int,
    270489937 as core::ffi::c_int,
    270302615 as core::ffi::c_int,
    270115423 as core::ffi::c_int,
    269928004 as core::ffi::c_int,
    269741072 as core::ffi::c_int,
    269554269 as core::ffi::c_int,
    269367240 as core::ffi::c_int,
    269180696 as core::ffi::c_int,
    268994281 as core::ffi::c_int,
    268807995 as core::ffi::c_int,
    268621484 as core::ffi::c_int,
];
pub const MUL_DRC_BAND: core::ffi::c_int = 4 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn ixheaacd_div_by_30(mut op: WORD32) -> WORD32 {
    let mut ret: WORD32 = 0;
    let mut temp: WORD64 = 0;
    temp = op as WORD64 * 35791394 as WORD64;
    ret = (temp + 17895697 as WORD64 >> 30 as core::ffi::c_int) as WORD32;
    return ret;
}
#[inline]
unsafe extern "C" fn ixheaacd_div_by_15(mut op: WORD64) -> WORD32 {
    let mut ret: WORD32 = 0;
    let mut temp: WORD64 = 0;
    temp = op * 71582788 as WORD64;
    ret = (temp + 134217728 as WORD64 >> 30 as core::ffi::c_int) as WORD32;
    return ret;
}
unsafe extern "C" fn ixheaacd_copy_drc_data(
    mut ch_data: *mut ixheaac_drc_data_struct,
    mut ptr_bs_data: *mut ixheaac_drc_bs_data_struct,
    mut frame_size: WORD32,
) -> VOID {
    let mut band_num: WORD32 = 0;
    (*ch_data).n_drc_bands = (*ptr_bs_data).drc_num_bands;
    if (*ch_data).n_drc_bands as core::ffi::c_int == 1 as core::ffi::c_int {
        (*ch_data).n_mdct_bands[0 as core::ffi::c_int as usize] = frame_size as WORD16;
        (*ch_data).drc_fac[0 as core::ffi::c_int as usize] = (*ptr_bs_data)
            .dyn_rng_dlbl[0 as core::ffi::c_int as usize] as WORD16;
        (*ch_data).drc_fac_dvb[0 as core::ffi::c_int as usize] = (*ptr_bs_data)
            .dyn_rng_dlbl_dvb[0 as core::ffi::c_int as usize] as WORD16;
    } else {
        band_num = 0 as core::ffi::c_int as WORD32;
        while band_num < (*ptr_bs_data).drc_num_bands as core::ffi::c_int {
            (*ch_data).n_mdct_bands[band_num as usize] = (((*ptr_bs_data)
                .drc_band_top[band_num as usize] as core::ffi::c_int
                + 1 as core::ffi::c_int) * MUL_DRC_BAND) as WORD16;
            (*ch_data).drc_fac[band_num as usize] = (*ptr_bs_data)
                .dyn_rng_dlbl[band_num as usize] as WORD16;
            (*ch_data).drc_fac_dvb[band_num as usize] = (*ptr_bs_data)
                .dyn_rng_dlbl_dvb[band_num as usize] as WORD16;
            band_num += 1;
        }
    }
    (*ch_data).drc_interp_scheme = (*ptr_bs_data).drc_interpolation_scheme;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_drc_map_channels(
    mut pstr_drc_dec: *mut ia_drc_dec_struct,
    mut num_channels: WORD32,
    mut frame_size: WORD32,
) -> WORD32 {
    let mut i: WORD32 = 0;
    let mut element: WORD32 = 0;
    let mut num_drc_elements: WORD32 = 0;
    let mut ptr_bs_data: *mut ixheaac_drc_bs_data_struct = 0
        as *mut ixheaac_drc_bs_data_struct;
    num_drc_elements = (*pstr_drc_dec).num_drc_elements as WORD32;
    if num_drc_elements == 0 as core::ffi::c_int {
        return IA_NO_ERROR;
    }
    num_drc_elements == 1 as core::ffi::c_int;
    if num_drc_elements > 1 as core::ffi::c_int {
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_channels {
            let mut drc_on: WORD32 = 0 as WORD32;
            element = 0 as core::ffi::c_int as WORD32;
            while element < num_drc_elements {
                ptr_bs_data = &mut *((*pstr_drc_dec).str_drc_bs_data)
                    .as_mut_ptr()
                    .offset(element as isize) as *mut ixheaac_drc_bs_data_struct;
                if (*ptr_bs_data).b_channel_on[i as usize] != 0 {
                    drc_on += 1;
                }
                element += 1;
            }
            if drc_on > 1 as core::ffi::c_int {
                return IA_XHEAAC_DEC_EXE_FATAL_INVALID_DRC_DATA as WORD32;
            }
            i += 1;
        }
    }
    element = 0 as core::ffi::c_int as WORD32;
    while element < num_drc_elements {
        ptr_bs_data = &mut *((*pstr_drc_dec).str_drc_bs_data)
            .as_mut_ptr()
            .offset(element as isize) as *mut ixheaac_drc_bs_data_struct;
        if (*ptr_bs_data).prog_ref_level_present != 0 {
            (*pstr_drc_dec).prog_ref_level = (*ptr_bs_data).prog_ref_level as WORD32;
        }
        i = 0 as core::ffi::c_int as WORD32;
        while i < num_channels {
            if !((*ptr_bs_data).b_channel_on[i as usize] == 0) {
                ixheaacd_copy_drc_data(
                    &mut *((*pstr_drc_dec).str_drc_channel_data)
                        .as_mut_ptr()
                        .offset(i as isize),
                    &mut *((*pstr_drc_dec).str_drc_bs_data)
                        .as_mut_ptr()
                        .offset(element as isize),
                    frame_size,
                );
            }
            i += 1;
        }
        element += 1;
    }
    return IA_NO_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_drc_dec_create(
    mut pstr_drc_dec: *mut ia_drc_dec_struct,
    mut drc_ref_level: WORD16,
    mut drc_def_level: WORD16,
) -> VOID {
    let mut j: WORD32 = 0;
    let mut k: WORD32 = 0;
    let mut ch: WORD32 = 0;
    (*pstr_drc_dec).sbr_allowed = 1 as UWORD8;
    (*pstr_drc_dec).sbr_found = 0 as UWORD8;
    (*pstr_drc_dec).drc_element_found = 0 as UWORD8;
    (*pstr_drc_dec).max_audio_channels = MAX_BS_ELEMENT as UWORD8;
    (*pstr_drc_dec).drc_ref_level = drc_ref_level;
    (*pstr_drc_dec).drc_def_level = drc_def_level;
    (*pstr_drc_dec).num_drc_elements = 0 as UWORD8;
    (*pstr_drc_dec).target_ref_level = 108 as core::ffi::c_int as WORD32;
    (*pstr_drc_dec).prog_ref_level = 108 as core::ffi::c_int as WORD32;
    (*pstr_drc_dec).cut_factor = 0 as core::ffi::c_int as WORD32;
    (*pstr_drc_dec).boost_factor = 0 as core::ffi::c_int as WORD32;
    (*pstr_drc_dec).drc_on = 0 as core::ffi::c_int as FLAG;
    (*pstr_drc_dec).drc_dig_norm = 1 as core::ffi::c_int as FLAG;
    (*pstr_drc_dec).pres_mode = -(1 as core::ffi::c_int) as WORD32;
    (*pstr_drc_dec).length_history = 2 as UWORD8;
    if (*pstr_drc_dec).sbr_allowed != 0 {
        (*pstr_drc_dec).length_history = ((*pstr_drc_dec).length_history)
            .wrapping_add(1);
    }
    ch = 0 as core::ffi::c_int as WORD32;
    while ch < MAX_BS_ELEMENT {
        let mut pstr_drc_data: *mut ixheaac_drc_data_struct = &mut *((*pstr_drc_dec)
            .str_drc_channel_data)
            .as_mut_ptr()
            .offset(ch as isize) as *mut ixheaac_drc_data_struct;
        (*pstr_drc_dec).drc_channel_next_index[ch as usize] = 0 as UWORD8;
        (*pstr_drc_dec).state = 0 as core::ffi::c_int as WORD32;
        j = 0 as core::ffi::c_int as WORD32;
        while j < 64 as core::ffi::c_int {
            k = 0 as core::ffi::c_int as WORD32;
            while k < 64 as core::ffi::c_int {
                (*pstr_drc_data).drc_factors_sbr[j as usize][k as usize] = DRC_SBR_ONE_Q25
                    as WORD32;
                (*pstr_drc_data).drc_factors_sbr_lat[j as usize][k as usize] = DRC_SBR_ONE_Q25
                    as WORD32;
                k += 1;
            }
            j += 1;
        }
        j = 0 as core::ffi::c_int as WORD32;
        while j < MAX_DRC_BANDS {
            (*pstr_drc_data).drc_fac[j as usize] = 0 as WORD16;
            j += 1;
        }
        (*pstr_drc_data).n_mdct_bands[0 as core::ffi::c_int as usize] = FRAME_SIZE
            as WORD16;
        (*pstr_drc_data).drc_exp = 1 as WORD8;
        (*pstr_drc_data).short_block = 0 as UWORD8;
        (*pstr_drc_data).drc_interp_scheme = 0 as UWORD8;
        (*pstr_drc_data).n_drc_bands = 1 as UWORD8;
        (*pstr_drc_data).new_prog_ref_level = 0 as UWORD8;
        (*pstr_drc_data).new_drc_fac = 0 as UWORD8;
        ch += 1;
    }
}
unsafe extern "C" fn ixheaacd_drc_excluded_channels(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut nch: WORD32,
    mut b_channel_on: *mut UWORD8,
) -> WORD32 {
    let mut ich: WORD32 = 0;
    let mut nbyte: WORD32 = 0 as WORD32;
    let mut num_excl_chan: WORD32 = 0;
    let mut exclude_mask: UWORD8 = 0;
    num_excl_chan = 7 as core::ffi::c_int as WORD32;
    ich = 0 as core::ffi::c_int as WORD32;
    while ich < 7 as core::ffi::c_int {
        exclude_mask = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD) as UWORD8;
        if ich < nch {
            *b_channel_on.offset(ich as isize) = (exclude_mask == 0) as core::ffi::c_int
                as UWORD8;
        }
        ich += 1;
    }
    nbyte += 1;
    while ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD) != 0 {
        ich = num_excl_chan;
        while ich < num_excl_chan as core::ffi::c_int + 7 as core::ffi::c_int {
            exclude_mask = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD) as UWORD8;
            if ich < nch {
                *b_channel_on.offset(ich as isize) = (exclude_mask == 0)
                    as core::ffi::c_int as UWORD8;
            }
            ich += 1;
        }
        nbyte += 1;
        num_excl_chan += 7 as core::ffi::c_int;
    }
    return nbyte;
}
unsafe extern "C" fn ixheaacd_drc_element_read(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut pstr_bs_data: *mut ixheaac_drc_bs_data_struct,
) -> WORD32 {
    let mut ich: WORD32 = 0;
    let mut idrc: WORD32 = 0;
    let mut nbyte: WORD32 = 1 as WORD32;
    let mut pce_tag_present: WORD32 = 0;
    let mut drc_bands_present: WORD32 = 0;
    let mut excluded_chns_present: WORD32 = 0;
    let mut drc_band_incr: UWORD8 = 0;
    let mut max_dyn_rng_dlbl: WORD8 = -(128 as core::ffi::c_int) as WORD8;
    (*pstr_bs_data).drc_num_bands = 1 as UWORD8;
    pce_tag_present = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD);
    if pce_tag_present != 0 {
        ixheaacd_read_bits_buf(it_bit_buf, 4 as WORD);
        ixheaacd_read_bits_buf(it_bit_buf, 4 as WORD);
        nbyte += 1;
    }
    ich = 0 as core::ffi::c_int as WORD32;
    while ich < MAX_AUDIO_CHANNELS {
        (*pstr_bs_data).b_channel_on[ich as usize] = 1 as UWORD8;
        ich += 1;
    }
    excluded_chns_present = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD);
    if excluded_chns_present != 0 {
        nbyte
            += ixheaacd_drc_excluded_channels(
                it_bit_buf,
                MAX_AUDIO_CHANNELS,
                ((*pstr_bs_data).b_channel_on).as_mut_ptr(),
            );
    }
    drc_bands_present = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD);
    if drc_bands_present != 0 {
        drc_band_incr = ixheaacd_read_bits_buf(it_bit_buf, 4 as WORD) as UWORD8;
        (*pstr_bs_data).drc_interpolation_scheme = ixheaacd_read_bits_buf(
            it_bit_buf,
            4 as WORD,
        ) as UWORD8;
        nbyte += 1;
        (*pstr_bs_data).drc_num_bands = ((*pstr_bs_data).drc_num_bands
            as core::ffi::c_int + drc_band_incr as core::ffi::c_int) as UWORD8;
        idrc = 0 as core::ffi::c_int as WORD32;
        while idrc < (*pstr_bs_data).drc_num_bands as core::ffi::c_int {
            (*pstr_bs_data).drc_band_top[idrc as usize] = ixheaacd_read_bits_buf(
                it_bit_buf,
                8 as WORD,
            ) as UWORD8;
            nbyte += 1;
            idrc += 1;
        }
    } else {
        (*pstr_bs_data).drc_band_top[0 as core::ffi::c_int as usize] = (FRAME_SIZE
            / 4 as core::ffi::c_int - 1 as core::ffi::c_int) as UWORD8;
        (*pstr_bs_data).drc_interpolation_scheme = 0 as UWORD8;
    }
    (*pstr_bs_data).prog_ref_level_present = ixheaacd_read_bits_buf(
        it_bit_buf,
        1 as WORD,
    ) as UWORD8;
    if (*pstr_bs_data).prog_ref_level_present != 0 {
        (*pstr_bs_data).prog_ref_level = ixheaacd_read_bits_buf(it_bit_buf, 7 as WORD)
            as UWORD8;
        ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD);
        nbyte += 1;
    }
    idrc = 0 as core::ffi::c_int as WORD32;
    while idrc < (*pstr_bs_data).drc_num_bands as core::ffi::c_int {
        let mut sign: WORD32 = ixheaacd_read_bits_buf(it_bit_buf, 1 as WORD);
        (*pstr_bs_data).dyn_rng_dlbl[idrc as usize] = ixheaacd_read_bits_buf(
            it_bit_buf,
            7 as WORD,
        ) as WORD8;
        if sign != 0 {
            (*pstr_bs_data).dyn_rng_dlbl[idrc as usize] = -((*pstr_bs_data)
                .dyn_rng_dlbl[idrc as usize] as core::ffi::c_int) as WORD8;
        }
        max_dyn_rng_dlbl = (if max_dyn_rng_dlbl as core::ffi::c_int
            > (*pstr_bs_data).dyn_rng_dlbl[idrc as usize] as core::ffi::c_int
        {
            max_dyn_rng_dlbl as core::ffi::c_int
        } else {
            (*pstr_bs_data).dyn_rng_dlbl[idrc as usize] as core::ffi::c_int
        }) as WORD8;
        nbyte += 1;
        idrc += 1;
    }
    (*pstr_bs_data).max_dyn_rng_dlbl = max_dyn_rng_dlbl;
    return nbyte;
}
unsafe extern "C" fn ixheaacd_drc_read_compression(
    mut it_bit_buf: *mut ia_bit_buf_struct,
    mut pstr_drc_dec: *mut ia_drc_dec_struct,
    mut bs_pos: WORD32,
) -> WORD32 {
    let mut bit_count: WORD32 = 0 as WORD32;
    let mut dmx_lvl_present: WORD32 = 0;
    let mut ext_present: WORD32 = 0;
    let mut compression_present: WORD32 = 0;
    let mut coarse_gain_present: WORD32 = 0;
    let mut fine_grain_present: WORD32 = 0;
    let mut local_bs: ia_bit_buf_struct = {
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
    let mut bytes: WORD32 = 0 as WORD32;
    let mut bits: WORD32 = 0 as WORD32;
    memcpy(
        &mut local_bs as *mut ia_bit_buf_struct as *mut core::ffi::c_void,
        it_bit_buf as *const core::ffi::c_void,
        ::core::mem::size_of::<ia_bit_buf_struct>() as size_t,
    );
    if local_bs.size < bs_pos {
        longjmp(
            (*local_bs.xaac_jmp_buf).as_mut_ptr(),
            IA_XHEAAC_DEC_EXE_NONFATAL_INSUFFICIENT_INPUT_BYTES,
        );
    }
    bytes = local_bs.size - bs_pos >> 3 as core::ffi::c_int;
    bits = ((local_bs.size as core::ffi::c_int - bs_pos as core::ffi::c_int)
        % 8 as core::ffi::c_int) as WORD32;
    local_bs.cnt_bits = bs_pos;
    local_bs.ptr_read_next = local_bs.ptr_bit_buf_base;
    local_bs.ptr_read_next = (local_bs.ptr_read_next).offset(bytes as isize);
    local_bs.bit_pos = 7 as WORD32 - bits;
    if ixheaacd_read_bits_buf(&mut local_bs, 8 as WORD) != DVB_ANC_DATA_SYNC_BYTE {
        return 0 as WORD32;
    }
    if ixheaacd_read_bits_buf(&mut local_bs, 2 as WORD) != 3 as core::ffi::c_int {
        return 0 as WORD32;
    }
    ixheaacd_read_bits_buf(&mut local_bs, 2 as WORD);
    (*pstr_drc_dec).pres_mode = ixheaacd_read_bits_buf(&mut local_bs, 2 as WORD);
    ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD);
    if ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD) != 0 as core::ffi::c_int {
        return 0 as WORD32;
    }
    if ixheaacd_read_bits_buf(&mut local_bs, 3 as WORD) != 0 as core::ffi::c_int {
        return 0 as WORD32;
    }
    dmx_lvl_present = ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD);
    ext_present = ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD);
    compression_present = ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD);
    coarse_gain_present = ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD);
    fine_grain_present = ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD);
    bit_count += 24 as core::ffi::c_int;
    if dmx_lvl_present != 0 {
        ixheaacd_read_bits_buf(&mut local_bs, 8 as WORD);
        bit_count += 8 as core::ffi::c_int;
    }
    if compression_present != 0 {
        let mut compression_on: UWORD8 = 0;
        let mut compression_val: UWORD8 = 0;
        if ixheaacd_read_bits_buf(&mut local_bs, 7 as WORD) != 0 as core::ffi::c_int {
            return 0 as WORD32;
        }
        compression_on = ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD) as UWORD8;
        compression_val = ixheaacd_read_bits_buf(&mut local_bs, 8 as WORD) as UWORD8;
        bit_count += 16 as core::ffi::c_int;
        if compression_on != 0 {
            (*pstr_drc_dec)
                .str_drc_bs_data[0 as core::ffi::c_int as usize]
                .drc_num_bands = 1 as UWORD8;
            (*pstr_drc_dec)
                .str_drc_bs_data[0 as core::ffi::c_int as usize]
                .dyn_rng_dlbl_dvb[0 as core::ffi::c_int as usize] = compression_val
                as WORD8;
            (*pstr_drc_dec)
                .str_drc_bs_data[0 as core::ffi::c_int as usize]
                .drc_band_top[0 as core::ffi::c_int as usize] = ((1024
                as core::ffi::c_int >> 2 as core::ffi::c_int) - 1 as core::ffi::c_int)
                as UWORD8;
            (*pstr_drc_dec).drc_ref_level = -(1 as core::ffi::c_int) as WORD16;
            (*pstr_drc_dec)
                .str_drc_bs_data[0 as core::ffi::c_int as usize]
                .drc_data_type = DVB_DRC_ANC_DATA as core::ffi::c_int as WORD8;
        } else {
            (*pstr_drc_dec)
                .str_drc_bs_data[0 as core::ffi::c_int as usize]
                .drc_num_bands = 1 as UWORD8;
            (*pstr_drc_dec)
                .str_drc_bs_data[0 as core::ffi::c_int as usize]
                .dyn_rng_dlbl_dvb[0 as core::ffi::c_int as usize] = 0x80
                as core::ffi::c_int as WORD8;
            (*pstr_drc_dec)
                .str_drc_bs_data[0 as core::ffi::c_int as usize]
                .drc_band_top[0 as core::ffi::c_int as usize] = ((1024
                as core::ffi::c_int >> 2 as core::ffi::c_int) - 1 as core::ffi::c_int)
                as UWORD8;
            (*pstr_drc_dec)
                .str_drc_bs_data[0 as core::ffi::c_int as usize]
                .drc_data_type = DVB_DRC_ANC_DATA as core::ffi::c_int as WORD8;
            if compression_val as core::ffi::c_int != 0 as core::ffi::c_int {
                return 0 as WORD32;
            }
        }
    }
    if coarse_gain_present != 0 {
        ixheaacd_read_bits_buf(&mut local_bs, 16 as WORD);
        bit_count += 16 as core::ffi::c_int;
    }
    if fine_grain_present != 0 {
        ixheaacd_read_bits_buf(&mut local_bs, 16 as WORD);
        bit_count += 16 as core::ffi::c_int;
    }
    if ext_present != 0 {
        let mut ext_bits: WORD32 = 8 as WORD32;
        ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD);
        if ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD) != 0 {
            ext_bits += 8 as core::ffi::c_int;
        }
        if ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD) != 0 {
            ext_bits += 16 as core::ffi::c_int;
        }
        if ixheaacd_read_bits_buf(&mut local_bs, 1 as WORD) != 0 {
            ext_bits += 8 as core::ffi::c_int;
        }
        ixheaacd_read_bits_buf(&mut local_bs, ext_bits as WORD - 4 as WORD);
        bit_count += ext_bits;
    }
    return bit_count;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_dec_drc_read_element(
    mut pstr_drc_dec: *mut ia_drc_dec_struct,
    mut drc_dummy: *mut ia_drc_dec_struct,
    mut it_bit_buf: *mut ia_bit_buf_struct,
) -> WORD32 {
    let mut bits_read: WORD32 = 0 as WORD32;
    if ((*pstr_drc_dec).num_drc_elements as core::ffi::c_int)
        < (*pstr_drc_dec).max_audio_channels as core::ffi::c_int
    {
        bits_read = ixheaacd_drc_element_read(
            it_bit_buf,
            &mut *((*pstr_drc_dec).str_drc_bs_data)
                .as_mut_ptr()
                .offset((*pstr_drc_dec).num_drc_elements as isize),
        );
        if (*pstr_drc_dec).dvb_anc_data_present != 0 {
            ixheaacd_drc_read_compression(
                it_bit_buf,
                pstr_drc_dec,
                (*pstr_drc_dec).dvb_anc_data_pos,
            );
        }
        (*pstr_drc_dec).num_drc_elements = ((*pstr_drc_dec).num_drc_elements)
            .wrapping_add(1);
    } else {
        let mut drc_ele_dummy: ixheaac_drc_bs_data_struct = ixheaac_drc_bs_data_struct {
            b_channel_on: [0; 8],
            prog_ref_level_present: 0,
            prog_ref_level: 0,
            drc_num_bands: 0,
            drc_band_top: [0; 16],
            dyn_rng_dlbl: [0; 16],
            dyn_rng_dlbl_dvb: [0; 16],
            max_dyn_rng_dlbl: 0,
            drc_interpolation_scheme: 0,
            drc_data_type: 0,
        };
        bits_read = ixheaacd_drc_element_read(it_bit_buf, &mut drc_ele_dummy);
        if (*pstr_drc_dec).dvb_anc_data_present != 0 {
            ixheaacd_drc_read_compression(
                it_bit_buf,
                drc_dummy,
                (*pstr_drc_dec).dvb_anc_data_pos,
            );
        }
    }
    (*pstr_drc_dec).dvb_anc_data_present = 0 as core::ffi::c_int as FLAG;
    return bits_read;
}
static mut ixheaacd_drc_offset: [[WORD32; 16]; 2] = [
    [
        0 as core::ffi::c_int,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        12 as core::ffi::c_int,
        16 as core::ffi::c_int,
        20 as core::ffi::c_int,
        24 as core::ffi::c_int,
        28 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        12 as core::ffi::c_int,
        16 as core::ffi::c_int,
        19 as core::ffi::c_int,
        22 as core::ffi::c_int,
        26 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_get_div_value_24(mut value: WORD32) -> WORD32 {
    let mut ret: WORD32 = 0;
    let mut temp: WORD64 = 0;
    temp = value as WORD64 * 44739243 as WORD64;
    ret = (temp + 22369621 as WORD64 >> 30 as core::ffi::c_int) as WORD32;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_get_div_value_2400(mut value: WORD32) -> WORD32 {
    let mut ret: WORD32 = 0;
    let mut temp: WORD64 = 0;
    temp = value as WORD64 * 447392 as WORD64;
    ret = (temp + 223696 as WORD64 >> 30 as core::ffi::c_int) as WORD32;
    return ret;
}
unsafe extern "C" fn ixheaacd_drc_div_120_floor(mut value: WORD32) -> WORD32 {
    let mut ret_val: WORD32 = 0;
    let mut temp: WORD64 = 0;
    temp = value as WORD64 * 8947849 as WORD64;
    ret_val = (temp >> 30 as core::ffi::c_int) as WORD32;
    return ret_val;
}
unsafe extern "C" fn ixheaacd_drc_floor(
    mut bottom: WORD32,
    mut frame_size: WORD32,
) -> WORD32 {
    let mut ret_val: WORD32 = 0;
    if 960 as core::ffi::c_int == frame_size {
        ret_val = ixheaacd_drc_div_120_floor(bottom);
        ret_val *= 30 as core::ffi::c_int;
        ret_val = ret_val >> 3 as core::ffi::c_int;
    } else {
        ret_val = bottom >> 7 as core::ffi::c_int;
        ret_val = ret_val << 2 as core::ffi::c_int;
    }
    return ret_val;
}
unsafe extern "C" fn ixheaacd_drc_ceil(
    mut top: WORD32,
    mut frame_size: WORD32,
) -> WORD32 {
    let mut ret_val: WORD32 = 0;
    if 960 as core::ffi::c_int == frame_size {
        top += 119 as core::ffi::c_int;
        ret_val = ixheaacd_drc_div_120_floor(top);
        ret_val *= 30 as core::ffi::c_int;
        ret_val = ret_val >> 3 as core::ffi::c_int;
    } else {
        top += 127 as core::ffi::c_int;
        ret_val = top >> 7 as core::ffi::c_int;
        ret_val = ret_val << 2 as core::ffi::c_int;
    }
    return ret_val;
}
unsafe extern "C" fn ixheaacd_drc_get_bottom_qmf(
    mut bottom: WORD32,
    mut frame_size: WORD32,
) -> WORD32 {
    let mut ret_val: WORD32 = 0;
    if 960 as core::ffi::c_int == frame_size {
        ret_val = (bottom as core::ffi::c_int % 120 as core::ffi::c_int) as WORD32;
        ret_val = ret_val << 5 as core::ffi::c_int;
        ret_val = ixheaacd_drc_div_120_floor(ret_val);
    } else {
        ret_val = (bottom as core::ffi::c_int & 0x7f as core::ffi::c_int) as WORD32;
        ret_val = ret_val >> 2 as core::ffi::c_int;
    }
    return ret_val;
}
#[no_mangle]
pub unsafe extern "C" fn ixheaacd_drc_apply(
    mut pstr_drc_dec: *mut ia_drc_dec_struct,
    mut ptr_spectral_coef: *mut WORD32,
    mut win_seq: WORD32,
    mut channel: WORD32,
    mut frame_size: WORD32,
    mut esbr_flag: WORD32,
    mut audio_object_type: WORD32,
) -> VOID {
    let mut drc_band: WORD32 = 0;
    let mut spec_pos: WORD32 = 0;
    let mut start_pos: WORD32 = 0;
    let mut end_pos: WORD32 = 0;
    let mut low_hi: WORD32 = 0;
    let mut drc_norm: WORD32 = 0;
    let mut drc_freq_fac: WORD32 = 0;
    let mut drc_fac: WORD32 = 0;
    let mut div_val: WORD32 = 0;
    let mut mod_val: WORD32 = 0;
    let mut ret_val: WORD32 = 0;
    let mut offset_value: WORD32 = 0;
    let mut table: *const WORD32 = 0 as *const WORD32;
    let mut pstr_drc_data: *mut ixheaac_drc_data_struct = 0
        as *mut ixheaac_drc_data_struct;
    let mut num_qmf_sub_sample: WORD32 = frame_size >> 5 as core::ffi::c_int;
    let mut num_qmf_sub_sample_by_2: WORD32 = frame_size >> 6 as core::ffi::c_int;
    let mut diff_ref_level: WORD32 = 0;
    let mut drc_sbr_factors: [*mut WORD32; 64] = [0 as *mut WORD32; 64];
    let mut qmf_start_pos: WORD32 = 0;
    let mut qmf_stop_pos: WORD32 = 0;
    let mut qmf_start: WORD32 = 0;
    let mut i: WORD32 = 0;
    let mut j: WORD32 = 0;
    let mut prev_frame_drc_sbr_factors: [WORD32; 64] = [0; 64];
    let mut ptr_drc_fac: *mut WORD32 = 0 as *mut WORD32;
    if audio_object_type != AOT_ER_AAC_ELD as core::ffi::c_int
        && audio_object_type != AOT_ER_AAC_LD as core::ffi::c_int && esbr_flag != 0
    {
        i = 0 as core::ffi::c_int as WORD32;
        while i < SBR_QMF_SUB_SAMPLES {
            j = 0 as core::ffi::c_int as WORD32;
            while j < SBR_QMF_SUB_BANDS {
                (*pstr_drc_dec)
                    .str_drc_channel_data[channel as usize]
                    .drc_factors_sbr[i as usize][j as usize] = (*pstr_drc_dec)
                    .str_drc_channel_data[channel as usize]
                    .drc_factors_sbr_lat[i as usize][j as usize];
                j += 1;
            }
            i += 1;
        }
        j = 0 as core::ffi::c_int as WORD32;
        while j < 32 as core::ffi::c_int {
            memcpy(
                ((*pstr_drc_dec)
                    .str_drc_channel_data[channel as usize]
                    .drc_factors_sbr_lat[j as usize])
                    .as_mut_ptr() as *mut core::ffi::c_void,
                ((*pstr_drc_dec)
                    .str_drc_channel_data[channel as usize]
                    .drc_factors_sbr_lat[(j as core::ffi::c_int + 32 as core::ffi::c_int)
                    as usize])
                    .as_mut_ptr() as *const core::ffi::c_void,
                (SBR_QMF_SUB_BANDS as size_t)
                    .wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
            );
            j += 1;
        }
        ptr_drc_fac = &mut *(*((*((*pstr_drc_dec).str_drc_channel_data)
            .as_mut_ptr()
            .offset(channel as isize))
            .drc_factors_sbr_lat)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    } else {
        ptr_drc_fac = &mut *(*((*((*pstr_drc_dec).str_drc_channel_data)
            .as_mut_ptr()
            .offset(channel as isize))
            .drc_factors_sbr)
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(0 as core::ffi::c_int as isize) as *mut WORD32;
    }
    i = 0 as core::ffi::c_int as WORD32;
    while i < 64 as core::ffi::c_int {
        drc_sbr_factors[i as usize] = ptr_drc_fac;
        ptr_drc_fac = ptr_drc_fac.offset(64 as core::ffi::c_int as isize);
        i += 1;
    }
    memcpy(
        prev_frame_drc_sbr_factors.as_mut_ptr() as *mut core::ffi::c_void,
        drc_sbr_factors[(2 as core::ffi::c_int * num_qmf_sub_sample as core::ffi::c_int
            - 1 as core::ffi::c_int) as usize] as *const core::ffi::c_void,
        (64 as size_t).wrapping_mul(::core::mem::size_of::<WORD32>() as size_t),
    );
    pstr_drc_data = &mut *((*pstr_drc_dec).str_drc_channel_data)
        .as_mut_ptr()
        .offset(channel as isize) as *mut ixheaac_drc_data_struct;
    if (*pstr_drc_dec).drc_on == 0 {
        return;
    }
    if (*pstr_drc_dec).drc_dig_norm != 0 {
        diff_ref_level = (*pstr_drc_dec).target_ref_level
            - (*pstr_drc_dec).prog_ref_level;
        if diff_ref_level < 0 as core::ffi::c_int {
            diff_ref_level = -diff_ref_level;
            table = ixheaacd_drc_pow_tbl_2_q29.as_ptr();
            div_val = ixheaacd_get_div_value_24(diff_ref_level);
            drc_norm = ((1 as core::ffi::c_int) << 25 as WORD32 + div_val) as WORD32;
            mod_val = (diff_ref_level as core::ffi::c_int
                - div_val as core::ffi::c_int * 24 as core::ffi::c_int) as WORD32;
            diff_ref_level = (mod_val as core::ffi::c_int * 1000 as core::ffi::c_int)
                as WORD32;
        } else {
            table = ixheaacd_drc_pow_tbl_1_2_q29.as_ptr();
            div_val = ixheaacd_get_div_value_24(diff_ref_level);
            drc_norm = ((1 as core::ffi::c_int) << 25 as WORD32 - div_val) as WORD32;
            mod_val = (diff_ref_level as core::ffi::c_int
                - div_val as core::ffi::c_int * 24 as core::ffi::c_int) as WORD32;
            diff_ref_level = (mod_val as core::ffi::c_int * 1000 as core::ffi::c_int)
                as WORD32;
        }
        ret_val = ixheaacd_get_div_value_24(diff_ref_level);
        drc_norm = ixheaacd_mult32x16in32_shift29(
            drc_norm,
            *table.offset(ret_val as isize),
        );
    } else {
        drc_norm = ((1 as core::ffi::c_int) << 25 as core::ffi::c_int) as WORD32;
    }
    start_pos = 0 as core::ffi::c_int as WORD32;
    drc_band = 0 as core::ffi::c_int as WORD32;
    while drc_band < (*pstr_drc_data).n_drc_bands as core::ffi::c_int {
        if (*pstr_drc_dec).str_drc_bs_data[0 as core::ffi::c_int as usize].drc_data_type
            as core::ffi::c_int == DVB_DRC_ANC_DATA as core::ffi::c_int
            && (*pstr_drc_dec).heavy_mode != 0
        {
            let mut val_x: WORD32 = 0;
            let mut val_y: WORD32 = 0;
            let mut compression_factor: core::ffi::c_float = 0.;
            let mut temp: core::ffi::c_float = 0.;
            val_x = ((*pstr_drc_data).drc_fac_dvb[drc_band as usize] as UWORD8
                as core::ffi::c_int >> 4 as core::ffi::c_int) as WORD32;
            val_y = ((*pstr_drc_data).drc_fac_dvb[drc_band as usize] as UWORD8
                as core::ffi::c_int & 0xf as core::ffi::c_int) as WORD32;
            compression_factor = (48.164f64 - 6.0206f64 * val_x as core::ffi::c_double
                - 0.4014f64 * val_y as core::ffi::c_double) as FLOAT32
                as core::ffi::c_float;
            temp = pow(
                10 as core::ffi::c_int as core::ffi::c_double,
                compression_factor as core::ffi::c_double / 20.0f64,
            ) as FLOAT32 as core::ffi::c_float;
            drc_freq_fac = (temp * 33554431.0f32) as WORD32;
        } else {
            if ((*pstr_drc_data).drc_fac[drc_band as usize] as core::ffi::c_int)
                < 0 as core::ffi::c_int
            {
                low_hi = (*pstr_drc_dec).cut_factor;
            } else {
                low_hi = (*pstr_drc_dec).boost_factor;
            }
            drc_fac = (*pstr_drc_dec)
                .str_drc_channel_data[channel as usize]
                .drc_fac[drc_band as usize] as WORD32 * low_hi;
            if drc_fac < 0 as core::ffi::c_int {
                drc_fac *= -(1 as core::ffi::c_int);
                table = ixheaacd_drc_pow_tbl_1_2_q29.as_ptr();
                div_val = ixheaacd_get_div_value_2400(drc_fac);
                drc_freq_fac = ((1 as core::ffi::c_int) << 25 as WORD32 - div_val)
                    as WORD32;
                mod_val = (drc_fac as core::ffi::c_int
                    - div_val as core::ffi::c_int * 2400 as core::ffi::c_int) as WORD32;
                drc_fac = (mod_val as core::ffi::c_int * 10 as core::ffi::c_int)
                    as WORD32;
            } else {
                table = ixheaacd_drc_pow_tbl_2_q29.as_ptr();
                div_val = ixheaacd_get_div_value_2400(drc_fac);
                drc_freq_fac = ((1 as core::ffi::c_int) << 25 as WORD32 + div_val)
                    as WORD32;
                mod_val = (drc_fac as core::ffi::c_int
                    - div_val as core::ffi::c_int * 2400 as core::ffi::c_int) as WORD32;
                drc_fac = (mod_val as core::ffi::c_int * 10 as core::ffi::c_int)
                    as WORD32;
            }
            ret_val = ixheaacd_get_div_value_24(drc_fac);
            drc_freq_fac = ixheaacd_mult32x16in32_shift29(
                drc_freq_fac,
                *table.offset(ret_val as isize),
            );
            drc_freq_fac = ixheaacd_mult32x16in32_shift25(drc_freq_fac, drc_norm);
        }
        end_pos = (*pstr_drc_data).n_mdct_bands[drc_band as usize] as WORD32;
        if (*pstr_drc_dec).sbr_found == 0 {
            spec_pos = start_pos;
            while spec_pos < end_pos {
                *ptr_spectral_coef.offset(spec_pos as isize) = ixheaacd_mult32x16in32_shift25(
                    *ptr_spectral_coef.offset(spec_pos as isize),
                    drc_freq_fac,
                );
                spec_pos += 1;
            }
        }
        if (*pstr_drc_dec).sbr_found != 0 {
            if win_seq != EIGHT_SHORT_SEQUENCE {
                if 960 as core::ffi::c_int == frame_size {
                    qmf_start = ixheaacd_div_by_30(start_pos);
                    offset_value = 1 as core::ffi::c_int as WORD32;
                } else {
                    qmf_start = start_pos >> 5 as core::ffi::c_int;
                    offset_value = 0 as core::ffi::c_int as WORD32;
                }
                j = -num_qmf_sub_sample_by_2;
                while j < num_qmf_sub_sample {
                    let mut alpha_val: WORD32 = 0 as WORD32;
                    if j + num_qmf_sub_sample_by_2 < num_qmf_sub_sample {
                        if (*pstr_drc_data).drc_interp_scheme as core::ffi::c_int
                            == 0 as core::ffi::c_int
                        {
                            alpha_val = j + num_qmf_sub_sample_by_2;
                            i = qmf_start;
                            while i < 64 as core::ffi::c_int {
                                let mut temp_drc: WORD64 = alpha_val as WORD64
                                    * drc_freq_fac as WORD64
                                    + (num_qmf_sub_sample - alpha_val) as WORD64
                                        * prev_frame_drc_sbr_factors[i as usize] as WORD64;
                                if frame_size == 512 as core::ffi::c_int {
                                    *(drc_sbr_factors[(num_qmf_sub_sample + j) as usize])
                                        .offset(i as isize) = (temp_drc >> 4 as core::ffi::c_int)
                                        as WORD32;
                                } else if frame_size == 480 as core::ffi::c_int {
                                    *(drc_sbr_factors[(num_qmf_sub_sample + j) as usize])
                                        .offset(i as isize) = ixheaacd_div_by_15(temp_drc);
                                } else {
                                    *(drc_sbr_factors[(num_qmf_sub_sample + j) as usize])
                                        .offset(i as isize) = (temp_drc >> 5 as core::ffi::c_int)
                                        as WORD32;
                                }
                                if 960 as core::ffi::c_int == frame_size {
                                    *(drc_sbr_factors[(num_qmf_sub_sample + j) as usize])
                                        .offset(i as isize) = ixheaacd_div_by_30(
                                        *(drc_sbr_factors[(num_qmf_sub_sample + j) as usize])
                                            .offset(i as isize),
                                    );
                                }
                                i += 1;
                            }
                        } else if j + num_qmf_sub_sample_by_2
                            >= ixheaacd_drc_offset[offset_value
                                as usize][((*pstr_drc_data).drc_interp_scheme
                                as core::ffi::c_int - 1 as core::ffi::c_int) as usize]
                        {
                            alpha_val = 1 as core::ffi::c_int as WORD32;
                            i = qmf_start;
                            while i < 64 as core::ffi::c_int {
                                *(drc_sbr_factors[(num_qmf_sub_sample + j) as usize])
                                    .offset(i as isize) = drc_freq_fac;
                                i += 1;
                            }
                        } else {
                            alpha_val = 0 as core::ffi::c_int as WORD32;
                            i = qmf_start;
                            while i < 64 as core::ffi::c_int {
                                *(drc_sbr_factors[(num_qmf_sub_sample + j) as usize])
                                    .offset(i as isize) = prev_frame_drc_sbr_factors[i
                                    as usize];
                                i += 1;
                            }
                        }
                    } else {
                        alpha_val = 1 as core::ffi::c_int as WORD32;
                        i = qmf_start;
                        while i < 64 as core::ffi::c_int {
                            *(drc_sbr_factors[(num_qmf_sub_sample + j) as usize])
                                .offset(i as isize) = drc_freq_fac;
                            i += 1;
                        }
                    }
                    j += 1;
                }
            } else {
                qmf_start_pos = ixheaacd_drc_floor(start_pos, frame_size);
                qmf_stop_pos = ixheaacd_drc_ceil(end_pos, frame_size);
                qmf_start = ixheaacd_drc_get_bottom_qmf(start_pos, frame_size);
                j = qmf_start_pos;
                while j < qmf_stop_pos {
                    if j > qmf_start_pos
                        && j as core::ffi::c_int & 0x3 as core::ffi::c_int
                            == 0 as core::ffi::c_int
                    {
                        qmf_start = 0 as core::ffi::c_int as WORD32;
                    }
                    i = qmf_start;
                    while i < 64 as core::ffi::c_int {
                        *(drc_sbr_factors[(num_qmf_sub_sample + j) as usize])
                            .offset(i as isize) = drc_freq_fac;
                        i += 1;
                    }
                    j += 1;
                }
            }
        }
        start_pos = end_pos;
        drc_band += 1;
    }
    if win_seq != EIGHT_SHORT_SEQUENCE {
        (*pstr_drc_data).prev_interp_scheme = (*pstr_drc_data).drc_interp_scheme;
    } else {
        (*pstr_drc_data).prev_interp_scheme = 8 as UWORD8;
    };
}
pub const FRAME_SIZE: core::ffi::c_int = 1024 as core::ffi::c_int;
pub const MAX_CC_CHANNEL_NUM: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_BS_ELEMENT: core::ffi::c_int = 8 as core::ffi::c_int + MAX_CC_CHANNEL_NUM;
pub const EIGHT_SHORT_SEQUENCE: core::ffi::c_int = 2 as core::ffi::c_int;
pub const MAX_32: WORD32 = 0x7fffffff as core::ffi::c_long as WORD32;
pub const MIN_32: WORD32 = 0x80000000 as core::ffi::c_long as WORD32;
