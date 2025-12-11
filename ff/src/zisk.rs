
extern "C" {
    // Field operations
    pub fn add_fp_bls12_381_c(a: *mut u64, b: *const u64);
    pub fn dbl_fp_bls12_381_c(a: *mut u64);
    pub fn sub_fp_bls12_381_c(a: *mut u64, b: *const u64);
    pub fn neg_fp_bls12_381_c(a: *mut u64);
    pub fn inv_fp_bls12_381_c(a: *mut u64);

    // Extension field operations
    pub fn add_fp2_bls12_381_c(a: *mut u64, b: *const u64);
    pub fn dbl_fp2_bls12_381_c(a: *mut u64);
    pub fn neg_fp2_bls12_381_c(a: *mut u64);
    pub fn sub_fp2_bls12_381_c(a: *mut u64, b: *const u64);
    pub fn mul_fp2_bls12_381_c(a: *mut u64, b: *const u64);
    pub fn inv_fp2_bls12_381_c(a: *mut u64);
}