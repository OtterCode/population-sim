pub struct Curve (f32);

use std::f32::consts::PI;

fn compare (skew: f32, x: f32, num: f32) -> bool {
    if skew.is_sign_negative() {
        x > num/skew
    } else {
        x < num/skew
    }
}


fn natural_part (factor: f32, x: f32) -> f32 {
    let root = 1.0/(factor * (2.0 * PI).sqrt());
    let nat = (-x.powf(2.0)/2.0).exp();
    root*nat
}

#[inline]
fn squares (skew: f32, x: f32) -> f32 {
    3.0 * skew.powf(2.0) * x.powf(2.0)
}

#[inline]
fn cubes (skew: f32, x: f32) -> f32 {
    (1.0/3.0) * skew.powf(3.0) * x.powf(3.0)
}


impl Curve {
    pub fn new(skew: f32) -> Curve {
        Curve(skew)
    }

    pub fn density_at (&self, x: f32) -> f32 {
        let &Curve(skew) = self;

        if compare(skew, x, -3.0) {
            0.0
        } else if compare(skew, x, -1.0) {
            natural_part(8.0, x) * ( 9.0 * x * skew + squares(skew, x) + cubes(skew, x) + 9.0)
        } else if compare(skew, x, 1.0) {
            natural_part(4.0, x) * ( 3.0 * x * skew - cubes(skew, x) + 4.0 )
        } else if compare(skew, x, 3.0) {
            natural_part(8.0, x) * ( 9.0 * x * skew - squares(skew, x) + cubes(skew, x) + 7.0)
        } else {
            (2.0/PI).sqrt() * (-x.powf(2.0)/2.0).exp()
        }

    }

    #[test]
    fn domain (extent: f32, resolution: u16) -> Vec<f32> {
        let res = resolution as f32;
        let length = (extent * 2.0 * res + 1.0) as usize;

        (0..length).map(|x| ((x as f32)/res) - extent.abs() ).collect()

    }

    #[test]
    pub fn symmetric_range (&self, extent: f32, resolution: u16) -> Vec<f32> {
        let domain = Curve::domain(extent, resolution);
        domain.iter().map(|x| self.density_at(*x)).collect()
    }

    #[test]
    pub fn range_x_y_tuple (&self, extent: f32, resolution: u16) -> Vec<(f32, f32)> {
        let domain = Curve::domain(extent, resolution);
        domain.iter().map(|x| (*x, self.density_at(*x)) ).collect()
    }

    pub fn limits_range (&self, lower: f32, upper: f32, count: usize) -> Vec<f32> {
        let size = (upper - lower) / count as f32;

        (0..count).map(|x| self.density_at(lower + (size * x as f32))).collect()

    }

}






#[cfg(test)]
mod density_tests {
    use ::skew_normal::Curve;

    #[test]
    fn zero_skew_symmetry() {
        let curve = Curve::new(0.0);
        for x_ in 1..91 {
            let x = x_ as f32;
            assert_eq!(curve.density_at(x/30.0), curve.density_at(-x/30.0));
        }
    }

    #[test]
    fn three_skew_sign_symmetry() {
        let left = Curve::new(3.0);
        let right = Curve::new(-3.0);
        for x_ in 1..91 {
            let x = x_ as f32;
            assert_eq!(left.density_at(x/30.0), right.density_at(-x/30.0));
            assert_ne!(left.density_at(x/30.0), left.density_at(-x/30.0));
        }
    }

    // Skew -3
    const REFERENCE_ARRAY: [(f32, f32); 161] =
        [ ( -5.0, 0.0000029734390294685954 )
        , ( -4.9375, 0.000004056279119311928 )
        , ( -4.875, 0.000005511885195099944 )
        , ( -4.8125, 0.000007460639096508988 )
        , ( -4.75, 0.00001005901457718489 )
        , ( -4.6875, 0.000013509472152285912 )
        , ( -4.625, 0.000018072775778102747 )
        , ( -4.5625, 0.000024083238011570993 )
        , ( -4.5, 0.00003196748221381095 )
        , ( -4.4375, 0.00004226739902689095 )
        , ( -4.375, 0.00005566806845842976 )
        , ( -4.3125, 0.00007303151646087455 )
        , ( -4.25, 0.0000954372730824099 )
        , ( -4.1875, 0.00012423079434083234 )
        , ( -4.125, 0.00016108089711118828 )
        , ( -4.0625, 0.0002080474295353678 )
        , ( -4.0, 0.00026766045152977074 )
        , ( -3.9375, 0.000343012222389447 )
        , ( -3.875, 0.0004378632755292242 )
        , ( -3.8125, 0.0005567637931967242 )
        , ( -3.75, 0.0007051913647348908 )
        , ( -3.6875, 0.0008897060082256206 )
        , ( -3.625, 0.0011181230444643297 )
        , ( -3.5625, 0.0013997040218938855 )
        , ( -3.5, 0.0017453653900915203 )
        , ( -3.4375, 0.002167903998229304 )
        , ( -3.375, 0.0026822377469807555 )
        , ( -3.3125, 0.0033056588448125163 )
        , ( -3.25, 0.004058096114599536 )
        , ( -3.1875, 0.0049623816722066 )
        , ( -3.125, 0.006044516070397512 )
        , ( -3.0625, 0.007333924692588452 )
        , ( -3.0, 0.008863696823876015 )
        , ( -2.9375, 0.01067079746317263 )
        , ( -2.875, 0.012796240621447113 )
        , ( -2.8125, 0.015285211637492803 )
        , ( -2.75, 0.01818712500318211 )
        , ( -2.6875, 0.021555603400541808 )
        , ( -2.625, 0.025448363193662865 )
        , ( -2.5625, 0.029926991571827894 )
        , ( -2.5, 0.03505660098713708 )
        , ( -2.4375, 0.0409053475455628 )
        , ( -2.375, 0.04754380165982761 )
        , ( -2.3125, 0.05504416160580894 )
        , ( -2.25, 0.06347930367133484 )
        , ( -2.1875, 0.07292166635238428 )
        , ( -2.125, 0.08344197051267722 )
        , ( -2.0625, 0.09510778252127926 )
        , ( -2.0, 0.10798193302637613 )
        , ( -1.9375, 0.12212081008213269 )
        , ( -1.875, 0.1375725516533838 )
        , ( -1.8125, 0.15437516887942143 )
        , ( -1.75, 0.17255463765302304 )
        , ( -1.6875, 0.1921230018102267 )
        , ( -1.625, 0.21307653626117015 )
        , ( -1.5625, 0.2353940224486401 )
        , ( -1.5, 0.2590351913317835 )
        , ( -1.4375, 0.2839393904104311 )
        , ( -1.375, 0.31002453091658644 )
        , ( -1.3125, 0.3371863690362302 )
        , ( -1.25, 0.36529817077804383 )
        , ( -1.1875, 0.3942108038371746 )
        , ( -1.125, 0.42375329155139896 )
        , ( -1.0625, 0.4537338539376254 )
        , ( -1.0, 0.48394144903828673 )
        , ( -0.9375, 0.5140772072360072 )
        , ( -0.875, 0.5435122196610405 )
        , ( -0.8125, 0.571446590741391 )
        , ( -0.75, 0.5969814328850127 )
        , ( -0.6875, 0.619133312694162 )
        , ( -0.625, 0.6368534030778641 )
        , ( -0.5625, 0.6490510771996798 )
        , ( -0.5, 0.6546214669523694 )
        , ( -0.4375, 0.6524762999713213 )
        , ( -0.375, 0.6415771358343889 )
        , ( -0.3125, 0.6209815492793612 )
        , ( -0.25, 0.5905751315231018 )
        , ( -0.1875, 0.5515482286020837 )
        , ( -0.125, 0.5054275153127539 )
        , ( -0.0625, 0.4539369310946262 )
        , ( 0.0, 0.3989422804014327 )
        , ( 0.0625, 0.3423907822791471 )
        , ( 0.125, 0.2862478585767451 )
        , ( 0.1875, 0.23243356790306008 )
        , ( 0.25, 0.18276110208259672 )
        , ( 0.3125, 0.1388796631178944 )
        , ( 0.375, 0.10213305190514894 )
        , ( 0.4375, 0.07258833410956912 )
        , ( 0.5, 0.04950918657622962 )
        , ( 0.5625, 0.032084111439981605 )
        , ( 0.625, 0.019468534022886022 )
        , ( 0.6875, 0.010813758159024581 )
        , ( 0.75, 0.005293431424596172 )
        , ( 0.8125, 0.002126744391437949 )
        , ( 0.875, 0.0005977770960466044 )
        , ( 0.9375, 0.00007060745746217689 )
        , ( 1.0, 0.0 )
        , ( 1.0625, 0.0 )
        , ( 1.125, 0.0 )
        , ( 1.1875, 0.0 )
        , ( 1.25, 0.0 )
        , ( 1.3125, 0.0 )
        , ( 1.375, 0.0 )
        , ( 1.4375, 0.0 )
        , ( 1.5, 0.0 )
        , ( 1.5625, 0.0 )
        , ( 1.625, 0.0 )
        , ( 1.6875, 0.0 )
        , ( 1.75, 0.0 )
        , ( 1.8125, 0.0 )
        , ( 1.875, 0.0 )
        , ( 1.9375, 0.0 )
        , ( 2.0, 0.0 )
        , ( 2.0625, 0.0 )
        , ( 2.125, 0.0 )
        , ( 2.1875, 0.0 )
        , ( 2.25, 0.0 )
        , ( 2.3125, 0.0 )
        , ( 2.375, 0.0 )
        , ( 2.4375, 0.0 )
        , ( 2.5, 0.0 )
        , ( 2.5625, 0.0 )
        , ( 2.625, 0.0 )
        , ( 2.6875, 0.0 )
        , ( 2.75, 0.0 )
        , ( 2.8125, 0.0 )
        , ( 2.875, 0.0 )
        , ( 2.9375, 0.0 )
        , ( 3.0, 0.0 )
        , ( 3.0625, 0.0 )
        , ( 3.125, 0.0 )
        , ( 3.1875, 0.0 )
        , ( 3.25, 0.0 )
        , ( 3.3125, 0.0 )
        , ( 3.375, 0.0 )
        , ( 3.4375, 0.0 )
        , ( 3.5, 0.0 )
        , ( 3.5625, 0.0 )
        , ( 3.625, 0.0 )
        , ( 3.6875, 0.0 )
        , ( 3.75, 0.0 )
        , ( 3.8125, 0.0 )
        , ( 3.875, 0.0 )
        , ( 3.9375, 0.0 )
        , ( 4.0, 0.0 )
        , ( 4.0625, 0.0 )
        , ( 4.125, 0.0 )
        , ( 4.1875, 0.0 )
        , ( 4.25, 0.0 )
        , ( 4.3125, 0.0 )
        , ( 4.375, 0.0 )
        , ( 4.4375, 0.0 )
        , ( 4.5, 0.0 )
        , ( 4.5625, 0.0 )
        , ( 4.625, 0.0 )
        , ( 4.6875, 0.0 )
        , ( 4.75, 0.0 )
        , ( 4.8125, 0.0 )
        , ( 4.875, 0.0 )
        , ( 4.9375, 0.0 )
        , ( 5.0, 0.0 )];

    // The algorithm only has a precision of about 10^-3, so we only test that deeply,
    fn trunc(x: f32) -> f32 {
        (x * 1000.0).trunc()
    }

    #[test]
    fn reference_values_test() {
        let curve = Curve::new(-3.0);
        for &(x, y) in REFERENCE_ARRAY.iter() {
            let result = trunc(curve.density_at(x));
            let reference = trunc(y);
            assert_eq!(result, reference)
        }
    }

    #[test]
    fn range_test() {
        let curve = Curve::new(2.0);
        let range = curve.range_x_y_tuple(3.0, 120);
        let &(first, _) = range.first().unwrap();
        let &(last, _) = range.last().unwrap();
        assert_eq!(first, -3.0);
        assert_eq!(last, 3.0);
    }

    #[test]
    fn range_reference_test() {
        let curve = Curve::new(-3.0);
        let range = curve.symmetric_range(5.0, 16);
        let zip = REFERENCE_ARRAY.iter().zip(range.iter());
        for (&(_, reference), actual) in zip {
            assert_eq!(trunc(reference), trunc(*actual));
        }
    }

    #[test]
    fn limits_range() {
        let curve = Curve::new(-2.0);
        let range = curve.limits_range(-3.0, 3.0, 10);
        assert_eq!(range.len(), 10);
        assert!( range[0] < 0.01 );
        assert!( range[3] > range [6] );
    }
}