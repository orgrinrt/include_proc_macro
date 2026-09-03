//! Every declaration in the matrix is invoked here and asserted to have produced
//! its own marker, rather than merely to have compiled.

#![allow(dead_code)]

arm_matrix::f_bare!();
arm_matrix::f_use!();
arm_matrix::f_mod2!();
arm_matrix::f_emod2!();
arm_matrix::f_umod2!();
arm_matrix::f_nest!();
arm_matrix::f_unest!();
arm_matrix::f_lit!();
arm_matrix::f_crel!();
arm_matrix::f_croot!();
arm_matrix::f_sroot!();
#[arm_matrix::a_bare]
struct P11;
#[arm_matrix::a_use]
struct P12;
#[arm_matrix::a_mod2]
struct P13;
#[arm_matrix::a_emod2]
struct P14;
#[arm_matrix::a_umod2]
struct P15;
#[arm_matrix::a_nest]
struct P16;
#[arm_matrix::a_unest]
struct P17;
#[arm_matrix::a_lit]
struct P18;
#[arm_matrix::a_crel]
struct P19;
#[arm_matrix::a_croot]
struct P20;
#[arm_matrix::a_sroot]
struct P21;
#[derive(arm_matrix::DDBare)]
struct S22;
#[derive(arm_matrix::DDUse)]
struct S23;
#[derive(arm_matrix::DDMod2)]
struct S24;
#[derive(arm_matrix::DDEmod2)]
struct S25;
#[derive(arm_matrix::DDUmod2)]
struct S26;
#[derive(arm_matrix::DDNest)]
struct S27;
#[derive(arm_matrix::DDUnest)]
struct S28;
#[derive(arm_matrix::DDLit)]
struct S29;
#[derive(arm_matrix::DDCrel)]
struct S30;
#[derive(arm_matrix::DDCroot)]
struct S31;
#[derive(arm_matrix::DDSroot)]
struct S32;
#[derive(arm_matrix::DDaBare)]
struct S33;
#[derive(arm_matrix::DDaUse)]
struct S34;
#[derive(arm_matrix::DDaMod2)]
struct S35;
#[derive(arm_matrix::DDaEmod2)]
struct S36;
#[derive(arm_matrix::DDaUmod2)]
struct S37;
#[derive(arm_matrix::DDaNest)]
struct S38;
#[derive(arm_matrix::DDaUnest)]
struct S39;
#[derive(arm_matrix::DDaLit)]
struct S40;
#[derive(arm_matrix::DDaCrel)]
struct S41;
#[derive(arm_matrix::DDaCroot)]
struct S42;
#[derive(arm_matrix::DDaSroot)]
struct S43;
arm_matrix::lf_bare!();
arm_matrix::lf_use!();
arm_matrix::lf_mod2!();
arm_matrix::lf_emod2!();
arm_matrix::lf_umod2!();
arm_matrix::lf_nest!();
arm_matrix::lf_unest!();
arm_matrix::lf_lit!();
arm_matrix::lf_crel!();
arm_matrix::lf_croot!();
arm_matrix::lf_sroot!();
#[arm_matrix::la_bare]
struct P55;
#[arm_matrix::la_use]
struct P56;
#[arm_matrix::la_mod2]
struct P57;
#[arm_matrix::la_emod2]
struct P58;
#[arm_matrix::la_umod2]
struct P59;
#[arm_matrix::la_nest]
struct P60;
#[arm_matrix::la_unest]
struct P61;
#[arm_matrix::la_lit]
struct P62;
#[arm_matrix::la_crel]
struct P63;
#[arm_matrix::la_croot]
struct P64;
#[arm_matrix::la_sroot]
struct P65;
#[derive(arm_matrix::DLdBare)]
struct S66;
#[derive(arm_matrix::DLdUse)]
struct S67;
#[derive(arm_matrix::DLdMod2)]
struct S68;
#[derive(arm_matrix::DLdEmod2)]
struct S69;
#[derive(arm_matrix::DLdUmod2)]
struct S70;
#[derive(arm_matrix::DLdNest)]
struct S71;
#[derive(arm_matrix::DLdUnest)]
struct S72;
#[derive(arm_matrix::DLdLit)]
struct S73;
#[derive(arm_matrix::DLdCrel)]
struct S74;
#[derive(arm_matrix::DLdCroot)]
struct S75;
#[derive(arm_matrix::DLdSroot)]
struct S76;
#[derive(arm_matrix::DLdaBare)]
struct S77;
#[derive(arm_matrix::DLdaUse)]
struct S78;
#[derive(arm_matrix::DLdaMod2)]
struct S79;
#[derive(arm_matrix::DLdaEmod2)]
struct S80;
#[derive(arm_matrix::DLdaUmod2)]
struct S81;
#[derive(arm_matrix::DLdaNest)]
struct S82;
#[derive(arm_matrix::DLdaUnest)]
struct S83;
#[derive(arm_matrix::DLdaLit)]
struct S84;
#[derive(arm_matrix::DLdaCrel)]
struct S85;
#[derive(arm_matrix::DLdaCroot)]
struct S86;
#[derive(arm_matrix::DLdaSroot)]
struct S87;
arm_matrix::if_mod2!();
arm_matrix::if_emod2!();
arm_matrix::if_umod2!();
arm_matrix::if_nest!();
arm_matrix::if_unest!();
arm_matrix::if_lit!();
arm_matrix::if_crel!();
arm_matrix::if_croot!();
arm_matrix::if_sroot!();
#[arm_matrix::ia_mod2]
struct P97;
#[arm_matrix::ia_emod2]
struct P98;
#[arm_matrix::ia_umod2]
struct P99;
#[arm_matrix::ia_nest]
struct P100;
#[arm_matrix::ia_unest]
struct P101;
#[arm_matrix::ia_lit]
struct P102;
#[arm_matrix::ia_crel]
struct P103;
#[arm_matrix::ia_croot]
struct P104;
#[arm_matrix::ia_sroot]
struct P105;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_arm_of_the_grammar_produces_its_own_marker() {
        assert_eq!(f_bare_mark(), "f_bare");
        assert_eq!(f_use_mark(), "f_use");
        assert_eq!(f_mod2_mark(), "f_mod2");
        assert_eq!(f_emod2_mark(), "f_emod2");
        assert_eq!(f_umod2_mark(), "f_umod2");
        assert_eq!(f_nest_mark(), "f_nest");
        assert_eq!(f_unest_mark(), "f_unest");
        assert_eq!(f_lit_mark(), "f_lit");
        assert_eq!(f_crel_mark(), "f_crel");
        assert_eq!(f_croot_mark(), "f_croot");
        assert_eq!(f_sroot_mark(), "f_sroot");
        assert_eq!(a_bare_mark(), "a_bare");
        assert_eq!(a_use_mark(), "a_use");
        assert_eq!(a_mod2_mark(), "a_mod2");
        assert_eq!(a_emod2_mark(), "a_emod2");
        assert_eq!(a_umod2_mark(), "a_umod2");
        assert_eq!(a_nest_mark(), "a_nest");
        assert_eq!(a_unest_mark(), "a_unest");
        assert_eq!(a_lit_mark(), "a_lit");
        assert_eq!(a_crel_mark(), "a_crel");
        assert_eq!(a_croot_mark(), "a_croot");
        assert_eq!(a_sroot_mark(), "a_sroot");
        assert_eq!(d_bare_mark(), "d_bare");
        assert_eq!(d_use_mark(), "d_use");
        assert_eq!(d_mod2_mark(), "d_mod2");
        assert_eq!(d_emod2_mark(), "d_emod2");
        assert_eq!(d_umod2_mark(), "d_umod2");
        assert_eq!(d_nest_mark(), "d_nest");
        assert_eq!(d_unest_mark(), "d_unest");
        assert_eq!(d_lit_mark(), "d_lit");
        assert_eq!(d_crel_mark(), "d_crel");
        assert_eq!(d_croot_mark(), "d_croot");
        assert_eq!(d_sroot_mark(), "d_sroot");
        assert_eq!(da_bare_mark(), "da_bare");
        assert_eq!(da_use_mark(), "da_use");
        assert_eq!(da_mod2_mark(), "da_mod2");
        assert_eq!(da_emod2_mark(), "da_emod2");
        assert_eq!(da_umod2_mark(), "da_umod2");
        assert_eq!(da_nest_mark(), "da_nest");
        assert_eq!(da_unest_mark(), "da_unest");
        assert_eq!(da_lit_mark(), "da_lit");
        assert_eq!(da_crel_mark(), "da_crel");
        assert_eq!(da_croot_mark(), "da_croot");
        assert_eq!(da_sroot_mark(), "da_sroot");
        assert_eq!(lf_bare_mark(), "lf_bare");
        assert_eq!(lf_use_mark(), "lf_use");
        assert_eq!(lf_mod2_mark(), "lf_mod2");
        assert_eq!(lf_emod2_mark(), "lf_emod2");
        assert_eq!(lf_umod2_mark(), "lf_umod2");
        assert_eq!(lf_nest_mark(), "lf_nest");
        assert_eq!(lf_unest_mark(), "lf_unest");
        assert_eq!(lf_lit_mark(), "lf_lit");
        assert_eq!(lf_crel_mark(), "lf_crel");
        assert_eq!(lf_croot_mark(), "lf_croot");
        assert_eq!(lf_sroot_mark(), "lf_sroot");
        assert_eq!(la_bare_mark(), "la_bare");
        assert_eq!(la_use_mark(), "la_use");
        assert_eq!(la_mod2_mark(), "la_mod2");
        assert_eq!(la_emod2_mark(), "la_emod2");
        assert_eq!(la_umod2_mark(), "la_umod2");
        assert_eq!(la_nest_mark(), "la_nest");
        assert_eq!(la_unest_mark(), "la_unest");
        assert_eq!(la_lit_mark(), "la_lit");
        assert_eq!(la_crel_mark(), "la_crel");
        assert_eq!(la_croot_mark(), "la_croot");
        assert_eq!(la_sroot_mark(), "la_sroot");
        assert_eq!(ld_bare_mark(), "ld_bare");
        assert_eq!(ld_use_mark(), "ld_use");
        assert_eq!(ld_mod2_mark(), "ld_mod2");
        assert_eq!(ld_emod2_mark(), "ld_emod2");
        assert_eq!(ld_umod2_mark(), "ld_umod2");
        assert_eq!(ld_nest_mark(), "ld_nest");
        assert_eq!(ld_unest_mark(), "ld_unest");
        assert_eq!(ld_lit_mark(), "ld_lit");
        assert_eq!(ld_crel_mark(), "ld_crel");
        assert_eq!(ld_croot_mark(), "ld_croot");
        assert_eq!(ld_sroot_mark(), "ld_sroot");
        assert_eq!(lda_bare_mark(), "lda_bare");
        assert_eq!(lda_use_mark(), "lda_use");
        assert_eq!(lda_mod2_mark(), "lda_mod2");
        assert_eq!(lda_emod2_mark(), "lda_emod2");
        assert_eq!(lda_umod2_mark(), "lda_umod2");
        assert_eq!(lda_nest_mark(), "lda_nest");
        assert_eq!(lda_unest_mark(), "lda_unest");
        assert_eq!(lda_lit_mark(), "lda_lit");
        assert_eq!(lda_crel_mark(), "lda_crel");
        assert_eq!(lda_croot_mark(), "lda_croot");
        assert_eq!(lda_sroot_mark(), "lda_sroot");
        assert_eq!(if_mod2_mark(), "if_mod2");
        assert_eq!(if_emod2_mark(), "if_emod2");
        assert_eq!(if_umod2_mark(), "if_umod2");
        assert_eq!(if_nest_mark(), "if_nest");
        assert_eq!(if_unest_mark(), "if_unest");
        assert_eq!(if_lit_mark(), "if_lit");
        assert_eq!(if_crel_mark(), "if_crel");
        assert_eq!(if_croot_mark(), "if_croot");
        assert_eq!(if_sroot_mark(), "if_sroot");
        assert_eq!(ia_mod2_mark(), "ia_mod2");
        assert_eq!(ia_emod2_mark(), "ia_emod2");
        assert_eq!(ia_umod2_mark(), "ia_umod2");
        assert_eq!(ia_nest_mark(), "ia_nest");
        assert_eq!(ia_unest_mark(), "ia_unest");
        assert_eq!(ia_lit_mark(), "ia_lit");
        assert_eq!(ia_crel_mark(), "ia_crel");
        assert_eq!(ia_croot_mark(), "ia_croot");
        assert_eq!(ia_sroot_mark(), "ia_sroot");
    }
}
