// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod bls12_377 {
    use crate::hash_to_curve::{hash_to_curve, try_hash_to_curve};
    use snarkvm_curves::{
        bls12_377::{FqParameters, G1Affine, G2Affine},
        AffineCurve,
    };
    use snarkvm_fields::{FieldParameters, PrimeField};
    use snarkvm_utilities::BigInteger384;

    const FIELD_BITS: u32 = FqParameters::MODULUS_BITS;

    #[test]
    fn hash_bls12_377_g1() {
        let g1 = hash_to_curve::<G1Affine, FIELD_BITS, 512>("Aleo BLS12-377 G1 in 2").unwrap();
        assert!(g1.is_on_curve());
        assert!(g1.is_in_correct_subgroup_assuming_on_curve());
        assert_eq!(
            try_hash_to_curve::<G1Affine, FIELD_BITS, 512>("Aleo BLS12-377 G1"),
            Some((g1, 2))
        );

        // String representation
        assert_eq!(
            g1.x.to_string(),
            "158156260823117507888693535084466232824779812809970945470560141963154412482435784320318186163184859520169843530551",
        );
        assert_eq!(
            g1.y.to_string(),
            "101649641885220841928061137108907365288027337468958757307620986090692360142741358761135939634151351398134601677025",
        );

        // Montgomery BigInteger representation
        assert_eq!(
            g1.x.to_repr(),
            BigInteger384::new([
                2774586903449475895,
                423263573669451800,
                11206930723317258628,
                12753691120262099807,
                4458377212156442360,
                74043642664212131
            ])
        );
        assert_eq!(
            g1.y.to_repr(),
            BigInteger384::new([
                5035842553512392929,
                1366085870489286170,
                1395534134415935333,
                16764851651524562552,
                18062160126717373041,
                47589072487696810
            ])
        );

        // Raw BigInteger representation
        assert_eq!(
            g1.x.to_repr_unchecked(),
            BigInteger384::new([
                10401513296453613476,
                1818244608987097434,
                4052942595619724991,
                6169356528924853781,
                8904133007154634493,
                112327492343176421
            ])
        );
        assert_eq!(
            g1.y.to_repr_unchecked(),
            BigInteger384::new([
                394851649514881096,
                14623811984333925737,
                10831936628026653845,
                763157746136055716,
                8818126587805414399,
                58295139739774058
            ])
        );
    }

    #[test]
    fn hash_bls12_377_g2() {
        let g2 = hash_to_curve::<G2Affine, FIELD_BITS, 512>("Aleo BLS12-377 G2 in 0").unwrap();
        assert!(g2.is_on_curve());
        assert!(g2.is_in_correct_subgroup_assuming_on_curve());
        assert_eq!(
            try_hash_to_curve::<G2Affine, FIELD_BITS, 512>("Aleo BLS12-377 G2"),
            Some((g2, 0)),
        );

        // String representation
        assert_eq!(
            g2.x.to_string(),
            "Fp2(18530967594235566243335187101752570107616176349552784968178098996818868750252111351392075324115454717813888945791 + 249109238597864858227740027174673999737378771274637856066753519982906549035341230102237437984449583493201730309992 * u)",
        );
        assert_eq!(
            g2.y.to_string(),
            "Fp2(162411681306238058413572478072422455473006141666469347219463629164083502715167424891268870041372045373590873694831 + 242659755380625277054368746168481273389655769841276332694767846852600636897945523549638958523260502082430026303549 * u)",
        );

        // Montgomery BigInteger representation
        assert_eq!(
            g2.x.c0.to_repr(),
            BigInteger384::new([
                9329452611231846015,
                8792950918801334804,
                4523229430185034581,
                15651789139955128560,
                3609164636334335099,
                8675599281550002
            ])
        );
        assert_eq!(
            g2.x.c1.to_repr(),
            BigInteger384::new([
                1376524554983280488,
                10406020321972407161,
                6426446564995126621,
                5671424054764911792,
                18241767988478723229,
                116624883207900101
            ])
        );
        assert_eq!(
            g2.y.c0.to_repr(),
            BigInteger384::new([
                8721605417431851631,
                2728844593816855904,
                15880900592053032399,
                6988067945587771436,
                9002822667476395852,
                76035892809721964
            ])
        );
        assert_eq!(
            g2.y.c1.to_repr(),
            BigInteger384::new([
                2375843859815909437,
                1458086711624452503,
                14704517538236290116,
                15600284112411805549,
                11183621525253435444,
                113605443900086630
            ])
        );

        // Raw BigInteger representation
        assert_eq!(
            g2.x.c0.to_repr_unchecked(),
            BigInteger384::new([
                1970932785276074212,
                5313855799015095490,
                13710104135971762041,
                9664454070231379298,
                1816926557282441884,
                71411904752082374
            ])
        );
        assert_eq!(
            g2.x.c1.to_repr_unchecked(),
            BigInteger384::new([
                5965396486954107648,
                16111224712623400997,
                2703376652134200593,
                18355826742575396191,
                9498717423363744549,
                4329860068425854
            ])
        );
        assert_eq!(
            g2.y.c0.to_repr_unchecked(),
            BigInteger384::new([
                767052386591684706,
                4046053330946717442,
                107140035638630851,
                1214901647713936718,
                3003138019557670046,
                114940352679405637
            ])
        );
        assert_eq!(
            g2.y.c1.to_repr_unchecked(),
            BigInteger384::new([
                9167062495373079174,
                3246999820078391825,
                17519529885154895381,
                1494814734789040059,
                17572559064583668264,
                118822025774994277
            ])
        );
    }
}

#[cfg(test)]
mod bw6_761 {
    use crate::hash_to_curve::{hash_to_curve, try_hash_to_curve};
    use snarkvm_curves::{
        bw6_761::{FqParameters, G1Affine, G2Affine},
        AffineCurve,
    };
    use snarkvm_fields::{FieldParameters, PrimeField};
    use snarkvm_utilities::BigInteger768;

    const FIELD_BITS: u32 = FqParameters::MODULUS_BITS;

    #[test]
    fn hash_bw6_761_g1() {
        let g1 = hash_to_curve::<G1Affine, FIELD_BITS, 768>("Aleo BW6-761 G1 in 6").unwrap();
        assert!(g1.is_on_curve());
        assert!(g1.is_in_correct_subgroup_assuming_on_curve());
        assert_eq!(
            try_hash_to_curve::<G1Affine, FIELD_BITS, 768>("Aleo BW6-761 G1"),
            Some((g1, 6)),
        );

        // String representation
        assert_eq!(
            g1.x.to_string(),
            "548300277143208370470678695366266429839120799927220954217122412131700072565385886272330413155172409119755953304536074890149359412851408456414505595462512752071492788194267343564981261358941753195544463428712608241091867757122460",
        );
        assert_eq!(
            g1.y.to_string(),
            "3317472076756963306110108609744156196308178143114799958012980985496220654686731782787499145400398246050358757683997715518181197214027285759025659330708904762639837257598196509175335127695930298616567045490708432441379668097317972",
        );

        // Montgomery BigInteger representation
        assert_eq!(
            g1.x.to_repr(),
            BigInteger768::new([
                13579198599579177884,
                2500255665044721259,
                13918998847643965790,
                5419926150672262902,
                11976600650548449586,
                8935185219208414527,
                12256217768720609774,
                12171479106780338253,
                16695703233586632852,
                10219504216236625260,
                6954107236629288832,
                6514806454214076
            ]),
        );
        assert_eq!(
            g1.y.to_repr(),
            BigInteger768::new([
                10564191201307999316,
                16160646607270735629,
                1301758656710250350,
                1487593118737100400,
                567820954530199392,
                15195072300028930515,
                11771520140344562058,
                4110821760151907916,
                7052126154745436425,
                12154321047374551837,
                5269908478860599984,
                39417613665889700
            ]),
        );

        // Raw BigInteger representation
        assert_eq!(
            g1.x.to_repr_unchecked(),
            BigInteger768::new([
                1157171927825290318,
                3235954916956490670,
                16812058207263087357,
                10080996767888849283,
                2794842962193595859,
                2388739970221432725,
                3320012158242682795,
                7161314541864567281,
                13032933159384664301,
                13076100984012168483,
                7612495201479106839,
                53584131847326574
            ]),
        );
        assert_eq!(
            g1.y.to_repr_unchecked(),
            BigInteger768::new([
                17537323669295701953,
                9286409947151450749,
                1543951643526860584,
                13597067073483526495,
                7544015345769907829,
                7005314844777489361,
                7007972873967911629,
                15155966061604017970,
                7045971538610474557,
                9672698377853857004,
                11582191816004119002,
                69728646693492388
            ]),
        );
    }

    #[test]
    fn hash_bw6_761_g2() {
        let g2 = hash_to_curve::<G2Affine, FIELD_BITS, 768>("Aleo BW6-761 G2 in 2").unwrap();
        assert!(g2.is_on_curve());
        assert!(g2.is_in_correct_subgroup_assuming_on_curve());
        assert_eq!(
            try_hash_to_curve::<G2Affine, FIELD_BITS, 768>("Aleo BW6-761 G2"),
            Some((g2, 2)),
        );

        // String representation
        assert_eq!(
            g2.x.to_string(),
            "12396641805492461664047944547716626246622726413716325514578855015708751941073354605391144730949906058792699340873351761792275567434439857256502535718512012900825516349253359648697192702793852676897460477790268766996191561426115",
        );
        assert_eq!(
            g2.y.to_string(),
            "3307524539084980175348192287168406817851662089289532346813191207846857431628300288777230178426528997214653383965949568784001180902638452916442590352134990654138684202694553699651807581643537139768612814360134519690660810904199476",
        );

        // Montgomery BigInteger representation
        assert_eq!(
            g2.x.to_repr(),
            BigInteger768::new([
                2201494757195687107,
                18328945831405402930,
                14950339849971702514,
                4978020437758378129,
                17323552614243995898,
                6566935094867504191,
                15155358173040786657,
                7214484026365812119,
                13188591670408108740,
                5228542183820318834,
                4836919448813850783,
                147294694917523
            ]),
        );
        assert_eq!(
            g2.y.to_repr(),
            BigInteger768::new([
                18174419409424507188,
                12753956117281638563,
                11256017256004465137,
                18304314616687125105,
                12847996337962071436,
                16042402710480045157,
                17352237559383218522,
                17547147095272221730,
                5321657193737760936,
                2379497709361297144,
                10129538758127257634,
                39299418791054634
            ]),
        );

        // Raw BigInteger representation
        assert_eq!(
            g2.x.to_repr_unchecked(),
            BigInteger768::new([
                13965385169003496293,
                15426759243192778648,
                17412803276392918350,
                17892104412223195454,
                7532942519447222464,
                11305536108286109734,
                13853093656533611357,
                3285564323678932468,
                9915794714252477034,
                3414216529059533261,
                11265573112366039784,
                5978735415715022
            ]),
        );
        assert_eq!(
            g2.y.to_repr_unchecked(),
            BigInteger768::new([
                122961968395967336,
                12501821910101597401,
                9993203793787452378,
                9589026370105636356,
                207488117621663087,
                18200370713322471620,
                17356725932671146122,
                18337093169776622661,
                3372949398219407920,
                12781993496907152533,
                16711250998472995857,
                20967738953282646
            ]),
        );
    }
}