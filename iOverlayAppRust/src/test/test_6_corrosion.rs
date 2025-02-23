use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::single::SingleFloatOverlay;
use std::f64::consts::PI;
use std::time::Instant;

pub(crate) struct CorrosionTest;

/*

// 6
// Difference:

// multithreading on
1     - 0.000010
2     - 0.000059
4     - 0.000361
8     - 0.001819
16     - 0.004534
32     - 0.018970
64     - 0.082330
128     - 0.335374
256     - 1.441913
512     - 5.635038
1024     - 26.365589


// multithreading off
1     - 0.000010
2     - 0.000060
4     - 0.000367
8     - 0.001838
16     - 0.004506
32     - 0.017891
64     - 0.078935
128     - 0.331489
256     - 1.360146
512     - 5.431161
1024     - 24.173921
*/

// A series of concentric squares, each progressively larger than the last.
impl CorrosionTest {
    pub(crate) fn run(n: usize, rule: OverlayRule, scale: f64) {
        // 500
        let (subj_paths, clip_paths) = Self::geometry(100.0, n);

        let it_count = ((scale / (n as f64)) as usize).max(1);
        let sq_it_count = it_count * it_count;

        let start = Instant::now();

        for _ in 0..sq_it_count {
            subj_paths.overlay(&clip_paths, rule, FillRule::NonZero);
        }
        let duration = start.elapsed();
        let time = duration.as_secs_f64() / sq_it_count as f64;

        println!("{}     - {:.6}", n, time);
    }

    fn geometry(size: f64, count: usize) -> (Vec<Vec<[f64; 2]>>, Vec<Vec<[f64; 2]>>) {
        let subj_radius = 0.4 * size;
        let subj_step = size;

        let clip_radius = 0.4 * subj_radius;
        let clip_step = 0.4 * subj_step;
        let clip_count = ((count as f64) * 2.5).round() as usize;

        let subj = Self::shapes(0.0, subj_step, subj_radius, count);
        let clip = Self::shapes(subj_radius, clip_step, clip_radius, clip_count);

        (subj, clip)
    }

    fn shapes(offset: f64, step: f64, radius: f64, count: usize) -> Vec<Vec<[f64; 2]>> {
        let mut y = -offset;

        let mut paths = Vec::with_capacity(count);
        for i in 0..count {
            let mut index = i;
            let mut x = -offset;
            for _ in 0..count {
                let count = (index % 5) + 3;
                paths.push(Self::shape([x, y], radius, count));
                x += step;
                index += 1;
            }
            y += step;
        }

        paths
    }

    fn shape(center: [f64; 2], radius: f64, count: usize) -> Vec<[f64; 2]> {
        let da: f64 = 2.0 * PI / (count as f64);
        let mut points = Vec::with_capacity(count);

        let mut a = 0.0f64;

        for _ in 0..count {
            let x = a.cos() * radius + center[0];
            let y = a.sin() * radius + center[1];
            points.push([x, y]);
            a += da;
        }

        points
    }
}
