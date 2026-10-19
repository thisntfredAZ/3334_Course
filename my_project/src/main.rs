use std::fs;
use std::path::Path;

const RECORD_BITS: u64 = 64; // fixed by the specification

#[derive(Clone)]
struct DayScore { day: usize, penalty: u64, useful: u64, reserved: u64, records: u64, pages: u64 }

fn score_program(useful: u64, page_bits: u64) -> (u64, u64, u64, u64) {
    let pages = (useful + page_bits - 1) / page_bits;      // whole pages, enough to hold data
    let reserved = pages * page_bits;                       // full page size per page
    let wasted = reserved - useful;                         // reserved space with no useful data
    let records = pages * RECORD_BITS;                      // one 64-bit record per page
    (wasted + records, reserved, records, pages)
}

fn score_day(day: usize, loads: &[u64], page_bits: u64) -> DayScore {
    let mut s = DayScore { day, penalty: 0, useful: 0, reserved: 0, records: 0, pages: 0 };
    for &u in loads {
        let (p, r, b, pg) = score_program(u, page_bits);
        s.penalty += p; s.useful += u; s.reserved += r; s.records += b; s.pages += pg;
    }
    s
}

fn read_loads(path: &Path) -> Vec<u64> {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {:?}: {}", path, e));
    let v: Vec<u64> = text.lines().filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().parse::<u64>().unwrap_or_else(|e| panic!("bad integer {:?}: {}", l, e)))
        .collect();
    assert_eq!(v.len(), 128, "expected 128 loads in {:?}, got {}", path, v.len());
    v
}

fn load_all(dir: &Path) -> Vec<Vec<u64>> {
    (0..32).map(|d| read_loads(&dir.join(format!("day_{:02}", d)).join("loads"))).collect()
}

fn avg_penalty(days: &[Vec<u64>], page_bits: u64) -> f64 {
    let total: u64 = days.iter().enumerate()
        .map(|(i, loads)| score_day(i, loads, page_bits).penalty).sum();
    total as f64 / 32.0
}

fn self_check() {
    // Your illustration: 23 useful bits, 6-bit pages -> 4 pages, 1 wasted bit, 4 records.
    let (p, r, b, pg) = score_program(23, 6);
    assert_eq!((pg, r, b, p), (4, 24, 256, 257));
    // Day 1 hand-checks at page_bits = 4096.
    assert_eq!(score_program(20_447_232, 4096).0, 319_488);
    assert_eq!(score_program(4_302, 4096).0, 4_018);
    assert_eq!(score_program(32_768, 4096).0, 512);
    // Day 2 hand-checks at page_bits = 4096.
    assert_eq!(score_program(15_400_960, 4096).0, 240_640);
    assert_eq!(score_program(25_165_824, 4096).0, 393_216);
    assert_eq!(score_program(29, 4096).0, 4_131);
    assert_eq!(score_program(165, 4096).0, 3_995);
    assert_eq!(score_program(16_385, 4096).0, 4_415);
    println!("self-check passed");
}

fn main() {
    self_check();
    let days = load_all(Path::new("workloads"));

    // Pass 1: powers of two, plus a few non-powers as a sanity spread.
    let mut cands: Vec<u64> = Vec::new();
    let mut p = 1u64;
    while p <= 2_097_152 { cands.push(p); p *= 2; }

    let mut best = (0u64, f64::INFINITY);
    println!("Pass 1:");
    for &pb in &cands {
        let a = avg_penalty(&days, pb);
        println!("  page_bits={:9}  avg={:.6}", pb, a);
        if a < best.1 { best = (pb, a); }
    }

    // Pass 2: every integer size within a factor of 2 of the pass-1 minimum.
    let lo = (best.0 / 2).max(1);
    let hi = (best.0 * 2).min(4_194_304);
    let mut best2 = (0u64, f64::INFINITY);
    for pb in lo..=hi {
        let a = avg_penalty(&days, pb);
        if a < best2.1 { best2 = (pb, a); }
    }
    println!("\nPass 2 best: page_bits={} avg={:.6}", best2.0, best2.1);
    println!("Neighbors (convexity check):");
    let start = best2.0.saturating_sub(4);
    for pb in start..=best2.0 + 4 {
        let a = avg_penalty(&days, pb);
        println!("  page_bits={:9}  avg={:.6}{}", pb, a, if pb == best2.0 { "  <-- best" } else { "" });
    }

    // Per-day breakdown at the best size.
    println!("\nPer-day at best size:");
    let mut tot = 0u64;
    for (i, loads) in days.iter().enumerate() {
        let s = score_day(i, loads, best2.0);
        tot += s.penalty;
        println!("  day {:02}: penalty={:10} pages={:9} useful={:12} reserved={:12} records={:10}",
                 s.day, s.penalty, s.pages, s.useful, s.reserved, s.records);
    }
    println!("  sum={}  average={:.6}", tot, tot as f64 / 32.0);
}