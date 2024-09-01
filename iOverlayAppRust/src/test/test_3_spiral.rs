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
32     - 0.000052
64     - 0.000139
128     - 0.000316
256     - 0.000703
512     - 0.001603
1024     - 0.004210
2048     - 0.009138
4096     - 0.017956
8192     - 0.037518
16384     - 0.071209
32768     - 0.152288
65536     - 0.318895
131072     - 0.662601
262144     - 1.338568
524288     - 2.789197
1048576     - 5.621691

multithreading off

2     - 0.000003
4     - 0.000005
8     - 0.000010
16     - 0.000020
32     - 0.000052
64     - 0.000139
128     - 0.000315
256     - 0.000691
512     - 0.001671
1024     - 0.004286
2048     - 0.009244
4096     - 0.018103
8192     - 0.037788
16384     - 0.071094
32768     - 0.152864
65536     - 0.306582
131072     - 0.659484
262144     - 1.339929
524288     - 2.794440
1048576     - 5.624586
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