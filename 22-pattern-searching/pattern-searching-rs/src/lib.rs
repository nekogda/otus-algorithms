#![feature(test)]

mod helpers {
    #![allow(dead_code)]

    #[inline]
    pub fn max(a: usize, b: usize) -> usize {
        if a > b {
            a
        } else {
            b
        }
    }

    #[inline]
    pub fn find_empty(t: &str, p: &str, start: usize) -> Option<usize> {
        debug_assert!(p.len() == 0);
        if start <= t.len() {
            Some(start)
        } else {
            None
        }
    }

    pub fn find_bf(t: &str, p: &str, start: usize) -> Option<usize> {
        debug_assert!(p.len() != 0);
        let t = t.as_bytes();
        let p = p.as_bytes();
        for i in start..(t.len() - p.len() + 1) {
            let mut matches = 0;
            for j in 0..p.len() {
                if t[i + j] == p[j] {
                    matches += 1;
                } else {
                    matches = 0;
                    break;
                }
            }
            if matches == p.len() {
                return Some(i);
            }
        }
        None
    }
}

mod bmh {
    #![allow(dead_code)]
    // http://www-igm.univ-mlv.fr/~lecroq/string/node16.html#SECTION00160
    use crate::bm;
    use crate::helpers;
    use std::iter::Iterator;

    pub struct BMHit<'a> {
        pattern: &'a str,
        text: &'a str,
        bct: [usize; 256],
        start: usize,
    }

    impl<'a> BMHit<'a> {
        fn _find(&mut self) -> Option<usize> {
            let t = self.text.as_bytes();
            let p = self.pattern.as_bytes();
            let mut i = self.start;
            while i <= t.len() - p.len() {
                let mut j = p.len() - 1;
                while p[j] == t[i + j] {
                    if j == 0 {
                        return Some(i);
                    }
                    j -= 1;
                }
                i += self.bct[t[i + p.len() - 1] as usize];
            }
            None
        }
    }

    impl<'a> Iterator for BMHit<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let result;
            if self.text.len() < self.pattern.len() {
                result = None;
            } else if self.pattern.len() == 0 {
                result = helpers::find_empty(self.text, self.pattern, self.start);
                self.start += 1;
            } else if self.pattern.len() == 1 {
                result = helpers::find_bf(self.text, self.pattern, self.start);
                if result.is_some() {
                    self.start = result.unwrap() + self.pattern.len();
                }
            } else {
                result = self._find();
                if result.is_some() {
                    self.start = result.unwrap() + self.pattern.len();
                }
            }
            result
        }
    }

    pub fn find(t: &str, p: &str) -> Option<usize> {
        find_all(t, p).nth(0)
    }

    pub fn find_all<'a>(t: &'a str, p: &'a str) -> BMHit<'a> {
        BMHit {
            start: 0,
            pattern: p,
            text: t,
            bct: bm::bct_prep(p),
        }
    }

}

mod kmp {
    #![allow(dead_code)]
    use crate::helpers;

    pub struct KMPdfait<'a> {
        pattern: &'a str,
        text: &'a str,
        dfa: Vec<Vec<usize>>,
        start: usize,
    }

    impl<'a> KMPdfait<'a> {
        fn _find(&mut self) -> Option<usize> {
            let t = self.text.as_bytes();
            let p = self.pattern.as_bytes();

            let mut i = self.start;
            let mut j = 0;
            while i < t.len() && j < p.len() {
                j = self.dfa[t[i] as usize][j];
                i += 1;
            }
            if j == p.len() {
                return Some(i - p.len());
            } else {
                return None;
            }
        }
    }

    impl<'a> Iterator for KMPdfait<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let result;
            if self.text.len() < self.pattern.len() {
                result = None;
            } else if self.pattern.len() == 0 {
                result = helpers::find_empty(self.text, self.pattern, self.start);
                self.start += 1;
            } else if self.pattern.len() == 1 {
                result = helpers::find_bf(self.text, self.pattern, self.start);
                if result.is_some() {
                    self.start = result.unwrap() + self.pattern.len();
                }
            } else {
                result = self._find();
                if result.is_some() {
                    self.start = result.unwrap() + 1;
                }
            }
            result
        }
    }

    fn dfa_prep(p: &str) -> Vec<Vec<usize>> {
        if p.len() == 0 {
            return vec![vec![]];
        }
        let p = p.as_bytes();
        let mut dfa = vec![vec![0; p.len()]; 256];
        dfa[p[0] as usize][0] = 1;
        let mut x = 0;
        for j in 1..p.len() {
            for c in 0..256 {
                dfa[c][j] = dfa[c][x]
            }
            dfa[p[j] as usize][j] = j + 1;
            x = dfa[p[j] as usize][x];
        }
        dfa
    }

    pub fn find(t: &str, p: &str) -> Option<usize> {
        find_all(t, p).nth(0)
    }

    pub fn find_all<'a>(t: &'a str, p: &'a str) -> KMPdfait<'a> {
        KMPdfait {
            start: 0,
            pattern: p,
            text: t,
            dfa: dfa_prep(p),
        }
    }

}

mod bm {
    #![allow(dead_code)]
    use crate::helpers;
    use std::iter::Iterator;

    pub struct BMit<'a> {
        pattern: &'a str,
        text: &'a str,
        bct: [usize; 256],
        gst: Vec<usize>,
        start: usize,
    }

    impl<'a> BMit<'a> {
        fn _find(&mut self) -> Option<usize> {
            let t = self.text.as_bytes();
            let p = self.pattern.as_bytes();
            let mut i = self.start + p.len() - 1;
            while i < t.len() {
                let mut j = p.len() - 1;
                while j > 0 && t[i] == p[j] {
                    i -= 1;
                    j -= 1;
                }
                if j == 0 && t[i] == p[j] {
                    return Some(i);
                }
                i += helpers::max(self.bct[t[i] as usize], self.gst[j]);
            }
            None
        }
    }

    impl<'a> Iterator for BMit<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let result;
            if self.text.len() < self.pattern.len() {
                result = None;
            } else if self.pattern.len() == 0 {
                result = helpers::find_empty(self.text, self.pattern, self.start);
                self.start += 1;
            } else if self.pattern.len() == 1 {
                result = helpers::find_bf(self.text, self.pattern, self.start);
                if result.is_some() {
                    self.start = result.unwrap() + self.pattern.len();
                }
            } else {
                result = self._find();
                if result.is_some() {
                    self.start = result.unwrap() + 1;
                }
            }
            result
        }
    }

    pub fn bct_prep(p: &str) -> [usize; 256] {
        let p = p.as_bytes();
        let mut table = [p.len(); 256];
        if p.len() == 0 {
            return table;
        }
        let last = p.len() - 1;
        for i in 0..last {
            table[p[i] as usize] = last - i;
        }
        table
    }

    fn gst_prep(p: &str) -> Vec<usize> {
        let mut table = vec![0; p.len()];
        if p.len() == 0 {
            return table;
        }
        let p = p.as_bytes();
        let last = p.len() - 1;
        let mut last_prefix = last;
        for i in (0..p.len()).rev() {
            if p.starts_with(&p[i + 1..]) {
                last_prefix = i + 1;
            }
            table[i] = last_prefix + last - i;
        }
        for i in 0..last {
            let len_suffix = longest_common_suffix(p, &p[1..i + 1]);
            if p[i - len_suffix] != p[last - len_suffix] {
                table[last - len_suffix] = len_suffix + last - i;
            }
        }
        table
    }

    fn longest_common_suffix(a: &[u8], b: &[u8]) -> usize {
        let mut i = 0;
        while i < a.len() && i < b.len() {
            if a[a.len() - 1 - i] != b[b.len() - 1 - i] {
                break;
            }
            i += 1;
        }
        i
    }

    pub fn find(t: &str, p: &str) -> Option<usize> {
        find_all(t, p).nth(0)
    }

    pub fn find_all<'a>(t: &'a str, p: &'a str) -> BMit<'a> {
        BMit {
            start: 0,
            pattern: p,
            text: t,
            bct: bct_prep(p),
            gst: gst_prep(p),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bm;
    use crate::bmh;
    use crate::kmp;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::iter::Iterator;
    extern crate test;
    use test::Bencher;

    const FNAME: &str = "string_matching_test_cases-31272-751472.tsv";

    struct TestCase {
        text: String,
        pattern: String,
        expected: Vec<usize>,
    }

    fn prep_from_file(file_name: &str) -> Vec<TestCase> {
        let f = File::open(file_name).unwrap();
        let reader = BufReader::new(f);
        let mut dataset = Vec::new();

        for line in reader.lines().skip(1) {
            let l = match line {
                Ok(l) => l,
                Err(e) => panic!("{:?}", e),
            };
            let mut it = l.split("\t");
            dataset.push(TestCase {
                text: String::from(it.next().unwrap()),
                pattern: String::from(it.next().unwrap()),
                expected: match it.next() {
                    Some(r) => {
                        let r = r.trim();
                        if r.len() > 0 {
                            r.split(' ').map(|x| x.parse().unwrap()).collect()
                        } else {
                            Vec::new()
                        }
                    }
                    None => Vec::new(),
                },
            });
        }
        dataset
    }

    #[test]
    fn dataset_tests() {
        let dataset = prep_from_file(FNAME);
        for (idx, tc) in dataset.iter().enumerate() {
            let results: Vec<(&str, Vec<usize>)> = vec![
                ("bm", bm::find_all(&tc.text, &tc.pattern).collect()),
                ("kmp_dfa", kmp::find_all(&tc.text, &tc.pattern).collect()),
                ("bmh", bmh::find_all(&tc.text, &tc.pattern).collect()),
            ];
            for (name, result) in results {
                assert!(
                    result == tc.expected,
                    format!("fn name = {}, tc id = {}", name, idx)
                );
            }
        }
    }

    #[test]
    fn base_tests() {
        let ds = vec![
            ("", "", Some(0)),
            ("a", "", Some(0)),
            ("a", "a", Some(0)),
            ("a", "b", None),
            ("qwerty", "q", Some(0)),
            ("qwerty", "e", Some(2)),
            ("qwerty", "y", Some(5)),
            ("hello", "he", Some(0)),
            ("qweЖЯty", "Я", Some(5)),
            ("---abcd---", "abcd", Some(3)),
            ("Hoola-Hoola girls like Hooligans", "Hooligan", Some(23)),
            ("abcabeabcabcabd", "abcabd", Some(9)),
            ("aaa", "a", Some(0)),
            ("abacabacak-abacak", "abacak", Some(4)),
            ("abacabac", "abacak", None),
        ];
        for (text, pattern, expected) in ds {
            assert_eq!(kmp::find(text, pattern), expected);
            assert_eq!(bm::find(text, pattern), expected);
            assert_eq!(bmh::find(text, pattern), expected);
        }
    }

    fn bench_prep_kmp() -> (String, String) {
        // F1 = b, F2 = a, Fn = Fn–1Fn–2 for n > 2
        fn fib_gen(n: u32) -> String {
            match n {
                1 => "b".to_string(),
                2 => "a".to_string(),
                _ => format!("{}{}", fib_gen(n - 1), fib_gen(n - 2)),
            }
        }
        let text = fib_gen(30);
        (text, "abaababaabac".to_string())
    }

    fn bench_prep_start() -> (String, String) {
        let mut text = "".to_string();
        while text.len() < 100_000 {
            text.push_str("aaaaaaaaaa");
        }
        let p = "baaaaaaaaa".to_string();
        (text, p)
    }

    fn bench_prep_middle() -> (String, String) {
        let mut text = "".to_string();
        while text.len() < 100_000 {
            text.push_str("aaaaaaaaaa");
        }
        let p = "aaaaabaaaaa".to_string();
        (text, p)
    }

    fn bench_prep_end() -> (String, String) {
        let mut text = "".to_string();
        while text.len() < 100_000 {
            text.push_str("aaaaaaaaaa");
        }
        let p = "aaaaaaaaab".to_string();
        (text, p)
    }

    fn bench_prep_bm_full_suff() -> (String, String) {
        let mut text = "----------bcd".to_string();
        while text.len() < 100_000 {
            text.push_str("------bcd");
        }
        let p = "abcd......abcd".to_string();
        (text, p)
    }

    fn bench_prep_bm_part_suff() -> (String, String) {
        let mut text = ".................Mabcd".to_string();
        while text.len() < 100_000 {
            text.push_str(".........Mabcd");
        }
        let p = "...Zabcd..Xabcd..Xabcd".to_string();
        (text, p)
    }

    macro_rules! bench_test {
        ($dataset:expr, $fn_name:ident, $fn_path:path) => {
            #[bench]
            fn $fn_name(b: &mut Bencher) {
                let (t, p) = $dataset;
                b.iter(|| $fn_path(&t, &p));
            }
        };
    }

    macro_rules! bench_group {
        ($ds_name: ident, $dataset:expr, $(($fn_name:ident, $fn_path:path)), *) => {
            mod $ds_name {
                extern crate test;
                use test::Bencher;
                $(
                    bench_test!($dataset, $fn_name, $fn_path);
                )*
            }
        };
    }

    #[bench]
    fn bench_bm_regular(b: &mut Bencher) {
        let v: Vec<(String, String)> = prep_from_file(FNAME)
            .iter()
            .map(|tc| (tc.text.clone(), tc.pattern.clone()))
            .collect();
        b.iter(|| {
            for (t, p) in &v {
                crate::bm::find(&t, &p);
            }
        });
    }

    #[bench]
    fn bench_kmp_regular(b: &mut Bencher) {
        let v: Vec<(String, String)> = prep_from_file(FNAME)
            .iter()
            .map(|tc| (tc.text.clone(), tc.pattern.clone()))
            .collect();
        b.iter(|| {
            for (t, p) in &v {
                crate::kmp::find(&t, &p);
            }
        });
    }

    #[bench]
    fn bench_bmh_regular(b: &mut Bencher) {
        let v: Vec<(String, String)> = prep_from_file(FNAME)
            .iter()
            .map(|tc| (tc.text.clone(), tc.pattern.clone()))
            .collect();
        b.iter(|| {
            for (t, p) in &v {
                crate::bmh::find(&t, &p);
            }
        });
    }

    bench_group!(
        fib,
        crate::tests::bench_prep_kmp(),
        (bmh, crate::bmh::find),
        (bm, crate::bm::find),
        (kmp, crate::kmp::find)
    );

    bench_group!(
        full_suffix,
        crate::tests::bench_prep_bm_full_suff(),
        (bmh, crate::bmh::find),
        (bm, crate::bm::find),
        (kmp, crate::kmp::find)
    );

    bench_group!(
        partial_suffix,
        crate::tests::bench_prep_bm_part_suff(),
        (bmh, crate::bmh::find),
        (bm, crate::bm::find),
        (kmp, crate::kmp::find)
    );

    bench_group!(
        synth_start,
        crate::tests::bench_prep_start(),
        (bmh, crate::bmh::find),
        (bm, crate::bm::find),
        (kmp, crate::kmp::find)
    );

    bench_group!(
        synth_middle,
        crate::tests::bench_prep_middle(),
        (bmh, crate::bmh::find),
        (bm, crate::bm::find),
        (kmp, crate::kmp::find)
    );

    bench_group!(
        synth_end,
        crate::tests::bench_prep_end(),
        (bmh, crate::bmh::find),
        (bm, crate::bm::find),
        (kmp, crate::kmp::find)
    );
}
