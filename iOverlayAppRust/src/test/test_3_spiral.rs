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
8     - 0.000009
16     - 0.000018
32     - 0.000045
64     - 0.000127
128     - 0.000286
256     - 0.000720
512     - 0.001583
1024     - 0.003588
2048     - 0.007770
4096     - 0.015644
8192     - 0.032830
16384     - 0.063838
32768     - 0.139423
65536     - 0.272777
131072     - 0.573041
262144     - 1.184501
524288     - 2.468238
1048576     - 5.003705

multithreading off

2     - 0.000002
4     - 0.000005
8     - 0.000009
16     - 0.000018
32     - 0.000045
64     - 0.000124
128     - 0.000289
256     - 0.000670
512     - 0.001659
1024     - 0.003518
2048     - 0.008056
4096     - 0.015971
8192     - 0.033272
16384     - 0.064134
32768     - 0.137379
65536     - 0.276421
131072     - 0.588327
262144     - 1.184652
524288     - 2.492921
1048576     - 4.973198

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