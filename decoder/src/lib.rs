
pub mod ixheaacd_basic_ops;
mod gen_ixheaacd_bind;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_down_no_change() {
        let src = vec![1000, 2000, 3000];
        let mut dest = vec![0; 3];

        ixheaacd_basic_ops::scale_down(&mut dest, &src, 5, 5); // Same shift

        assert_eq!(dest, src);
    }
}
