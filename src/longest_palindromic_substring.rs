#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut palindrome = "";
        let mut palindrome_len = 0;
        for i in 1..=s.len() {
            let (mut l, mut r) = (i, i);
            while l > 0 && r <= s.len() && s.as_bytes()[l - 1] == s.as_bytes()[r - 1] {
                l -= 1;
                if palindrome_len < r - l {
                    palindrome = &s[l..r];
                    palindrome_len = r - l;
                }
                r += 1;
            }
            let (mut l, mut r) = (i, i + 1);
            while l > 0 && r <= s.len() && s.as_bytes()[l - 1] == s.as_bytes()[r - 1] {
                l -= 1;
                if palindrome_len < r - l {
                    palindrome = &s[l..r];
                    palindrome_len = r - l;
                }
                r += 1;
            }
        }
        palindrome.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!("bab", Solution::longest_palindrome("babad".into()));
    }

    #[test]
    fn case_2() {
        assert_eq!("bb", Solution::longest_palindrome("cbbd".into()));
    }

    #[test]
    fn case_3() {
        assert_eq!("a", Solution::longest_palindrome("a".into()));
    }

    #[test]
    fn case_4() {
        assert_eq!("khvhk", Solution::longest_palindrome("mwwfjysbkebpdjyabcfkgprtxpwvhglddhmvaprcvrnuxifcrjpdgnktvmggmguiiquibmtviwjsqwtchkqgxqwljouunurcdtoeygdqmijdympcamawnlzsxucbpqtuwkjfqnzvvvigifyvymfhtppqamlgjozvebygkxawcbwtouaankxsjrteeijpuzbsfsjwxejtfrancoekxgfyangvzjkdskhssdjvkvdskjtiybqgsmpxmghvvicmjxqtxdowkjhmlnfcpbtwvtmjhnzntxyfxyinmqzivxkwigkondghzmbioelmepgfttczskvqfejfiibxjcuyevvpawybcvvxtxycrfbcnpvkzryrqujqaqhoagdmofgdcbhvlwgwmsmhomknbanvntspvvhvccedzzngdywuccxrnzbtchisdwsrfdqpcwknwqvalczznilujdrlevncdsyuhnpmheukottewtkuzhookcsvctsqwwdvfjxifpfsqxpmpwospndozcdbfhselfdltmpujlnhfzjcgnbgprvopxklmlgrlbldzpnkhvhkybpgtzipzotrgzkdrqntnuaqyaplcybqyvidwcfcuxinchretgvfaepmgilbrtxgqoddzyjmmupkjqcypdpfhpkhitfegickfszermqhkwmffdizeoprmnlzbjcwfnqyvmhtdekmfhqwaftlyydirjnojbrieutjhymfpflsfemkqsoewbojwluqdckmzixwxufrdpqnwvwpbavosnvjqxqbosctttxvsbmqpnolfmapywtpfaotzmyjwnd".into()));
    }
}
