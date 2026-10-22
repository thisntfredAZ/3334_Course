use std::collections::BTreeSet;
use std::fs;

fn main() -> std::io::Result<()> {
    eprintln!("[0] start");

    let mut days: Vec<Vec<u64>> = Vec::with_capacity(32);
    for d in 1..=32 {
        let path = format!("workloads/day_{:02}/loads", d);
        let text = fs::read_to_string(&path)?;
        let reqs: Vec<u64> = text.split_whitespace()
            .map(|s| s.parse().expect("bad integer"))
            .collect();
        assert_eq!(reqs.len(), 128, "day {} has {} programs", d, reqs.len());
        days.push(reqs);
    }
    let max_r = days.iter().flatten().copied().max().unwrap();
    let min_r = days.iter().flatten().copied().min().unwrap();
    eprintln!("[1] loaded 32 days; max={} min={}", max_r, min_r);

    let mut cands: BTreeSet<u64> = BTreeSet::new();

    let lo = (min_r / 256).max(1);
    let mut p = lo;
    while p <= 4096 && p <= max_r {
        cands.insert(p);
        p += 1;
    }

    for day in &days {
        for &r in day {
            if r == 0 { continue; }
            let mut k = 1u64;
            while k <= r {
                let p = r / k + u64::from(r % k != 0);
                if p <= 4096 { break; } // grid handles small P
                cands.insert(p);
                // jump to next distinct value
                k = r / p + 1;
            }
        }
    }
    eprintln!("[2] {} candidates", cands.len());

    let mut best: Option<(f64, u64)> = None;
    let mut results: Vec<(u64, f64)> = Vec::with_capacity(cands.len());
    for &p in &cands {
        let mut total = 0.0f64;
        for day in &days {
            let mut pen = 0u64;
            for &r in day {
                let n = r / p + u64::from(r % p != 0);
                pen += 64 * n + (n * p - r);
            }
            total += pen as f64;
        }
        let mean = total / 32.0;
        results.push((p, mean));
        if best.map_or(true, |(bm, _)| mean < bm) { best = Some((mean, p)); }
    }
    eprintln!("[3] evaluated");

    let (best_mean, best_p) = best.unwrap();
    let ties: Vec<u64> = results.iter()
        .filter(|(_, m)| (m - best_mean).abs() < 1e-9)
        .map(|(p, _)| *p).collect();

    println!("BEST PAGE SIZE: {} bits", best_p);
    println!("AVERAGE PENALTY: {:.6} bits", best_mean);
    if ties.len() > 1 { println!("TIES: {:?}", ties); }

    println!("\nDay | Programs | Bookkeeping | Unused | Penalty");
    let mut grand = 0u64;
    for (i, day) in days.iter().enumerate() {
        let (mut bk, mut un) = (0u64, 0u64);
        for &r in day {
            let n = r / best_p + u64::from(r % best_p != 0);
            bk += 64 * n;
            un += n * best_p - r;
        }
        grand += bk + un;
        println!("{:3} | {:8} | {:11} | {:6} | {}",
                 i + 1, day.len(), bk, un, bk + un);
    }
    println!("Grand total penalty: {}", grand);

    let mut top = results;
    top.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("\nTop 5 candidates (P, mean penalty):");
    for &(p, m) in top.iter().take(5) {
        println!("  P={:5}  mean={:.6}", p, m);
    }
    Ok(())
}