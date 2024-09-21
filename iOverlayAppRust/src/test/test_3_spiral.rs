use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::f64::simplify::Simplify;
use crate::test::util::Util;

pub(crate) struct SpiralTest;

/*

// 3
// Intersection:


multithreading on

2     - 0.000003
4     - 0.000005
8     - 0.000010
16     - 0.000020
32     - 0.000051
64     - 0.000134
128     - 0.000308
256     - 0.000713
512     - 0.001686
1024     - 0.003754
2048     - 0.008199
4096     - 0.017006
8192     - 0.036680
16384     - 0.070433
32768     - 0.153460
65536     - 0.316911
131072     - 0.660288
262144     - 1.352219
524288     - 2.735063
1048576     - 5.481889

multithreading off

2     - 0.000003
4     - 0.000005
8     - 0.000010
16     - 0.000020
32     - 0.000050
64     - 0.000133
128     - 0.000305
256     - 0.000699
512     - 0.001723
1024     - 0.003801
2048     - 0.008436
4096     - 0.016422
8192     - 0.034739
16384     - 0.068730
32768     - 0.146693
65536     - 0.297990
131072     - 0.632649
262144     - 1.292599
524288     - 2.703499
1048576     - 5.407953

 */

// Two irregular self-intersecting polygons are generated, the vertices of which are defined by a fixed radius and angle.
impl SpiralTest {
    pub(crate) fn run(n: usize) {
        let subj_path = Util::spiral(n, 100.0);

        let it_count = ((1000.0 / (n as f64)) as usize).max(1);
        let sq_it_count= it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            let _ = subj_path.clone().simplify(FillRule::NonZero, 0.0);
        }

        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        let polygons_count = n;

        println!("{}     - {:.6}", polygons_count, time);
    }
}