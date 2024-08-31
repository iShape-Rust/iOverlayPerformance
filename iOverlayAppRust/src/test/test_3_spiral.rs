use std::time::Instant;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::f64::simplify::Simplify;
use crate::test::util::Util;

pub(crate) struct SpiralTest;

/*

// 3
// Intersection:


multithreading on

2(4 0.6)     - 0.000003(-5.6)
4(8 0.9)     - 0.000005(-5.3)
8(16 1.2)     - 0.000010(-5.0)
16(32 1.5)     - 0.000019(-4.7)
32(64 1.8)     - 0.000048(-4.3)
64(128 2.1)     - 0.000127(-3.9)
128(256 2.4)     - 0.000289(-3.5)
256(512 2.7)     - 0.000602(-3.2)
512(1024 3.0)     - 0.001390(-2.9)
1024(2048 3.3)     - 0.003753(-2.4)
2048(4096 3.6)     - 0.008427(-2.1)
4096(8192 3.9)     - 0.016339(-1.8)
8192(16384 4.2)     - 0.033950(-1.5)
16384(32768 4.5)     - 0.063569(-1.2)
32768(65536 4.8)     - 0.126560(-0.9)
65536(131072 5.1)     - 0.261169(-0.6)
131072(262144 5.4)     - 0.565251(-0.2)
262144(524288 5.7)     - 1.146704(0.1)
524288(1048576 6.0)     - 10.988824(1.0)


multithreading off

2(4 0.6)     - 0.000003(-5.6)
4(8 0.9)     - 0.000005(-5.3)
8(16 1.2)     - 0.000010(-5.0)
16(32 1.5)     - 0.000019(-4.7)
32(64 1.8)     - 0.000047(-4.3)
64(128 2.1)     - 0.000130(-3.9)
128(256 2.4)     - 0.000284(-3.5)
256(512 2.7)     - 0.000618(-3.2)
512(1024 3.0)     - 0.001463(-2.8)
1024(2048 3.3)     - 0.003738(-2.4)
2048(4096 3.6)     - 0.007943(-2.1)
4096(8192 3.9)     - 0.016288(-1.8)
8192(16384 4.2)     - 0.032871(-1.5)
16384(32768 4.5)     - 0.063310(-1.2)
32768(65536 4.8)     - 0.126852(-0.9)
65536(131072 5.1)     - 0.261077(-0.6)
131072(262144 5.4)     - 0.568775(-0.2)
262144(524288 5.7)     - 1.160852(0.1)
524288(1048576 6.0)     - 10.897343(1.0)
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

        let polygons_count = 2 * n;
        let count_log = (polygons_count as f64).log10();
        let time_log = time.log10();

        println!("{}({} {:.1})     - {:.6}({:.1})", n, polygons_count, count_log, time, time_log);
    }
}