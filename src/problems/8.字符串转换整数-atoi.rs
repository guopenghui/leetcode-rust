struct Solution;
/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();

        let mut auto = Auto {
            state: State::Sig,
            sig: 1,
            value: 0,
        };

        for c in s.chars() {
            match auto.parse(c) {
                State::End => break,
                _ => continue,
            }
        }

        auto.value
    }
}

#[derive(Clone, Copy)]
enum State {
    Sig,
    Digit,
    End,
}

struct Auto {
    state: State,
    sig: i32,
    value: i32,
}

impl Auto {
    fn parse(&mut self, c: char) -> State {
        self.state = match self.state {
            State::Sig => match c {
                '-' => {
                    self.sig = -1;
                    State::Digit
                }
                '+' => {
                    self.sig = 1;
                    State::Digit
                }
                '.' | 'a'..='z' | 'A'..='Z' => State::End,
                digit => {
                    self.sig = 1;
                    self.value = digit.to_digit(10).unwrap() as i32;
                    State::Digit
                }
            },
            State::Digit => {
                if c.is_digit(10) {
                    let digit = c.to_digit(10).unwrap() as i32;
                    if self.sig == 1
                        && (self.value > i32::MAX / 10 || self.value == i32::MAX / 10 && digit >= 7)
                    {
                        self.value = i32::MAX;
                        State::End
                    } else if self.sig == -1
                        && (self.value < i32::MIN / 10 || self.value == i32::MIN / 10 && digit >= 8)
                    {
                        self.value = i32::MIN;
                        State::End
                    } else {
                        self.value = self.value * 10 + digit * self.sig;
                        State::Digit
                    }
                } else {
                    State::End
                }
            }
            State::End => State::End,
        };

        self.state
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }

    #[test]
    fn test_invalid() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("+.".to_string()), 0);
        assert_eq!(Solution::my_atoi(".+222".to_string()), 0);
    }

    #[test]
    fn test_edge() {
        assert_eq!(Solution::my_atoi("2147483647".to_string()), 2147483647);
        assert_eq!(Solution::my_atoi(" -2147483648".to_string()), -2147483648);
    }
}
