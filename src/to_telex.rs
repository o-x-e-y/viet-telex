use std::collections::HashSet;

pub fn viet_telex(source: &str) -> String {
    let decoder = VietTelex::new();
    decoder.decode_viet(source)
}

struct VietTelex {
    viet_letters: HashSet<char>,
}

impl VietTelex {
    pub fn new() -> VietTelex {
        let alphabet = concat!(
            "aàáảãạâầấẩẫậăằắẳẵặbcdđeèéẻẽẹêềếểễệfghiìíỉĩịjkln",
            "moòóỏõọôồốổỗộơờớởỡợpqrstuùúủũụưừứửữựvwxyỳýỷỹỵz"
        );

        let viet_letters = alphabet
            .chars()
            .chain(alphabet.to_uppercase().chars())
            .collect::<HashSet<_>>();

        Self { viet_letters }
    }

    fn is_viet_letter(&self, c: char) -> bool {
        self.viet_letters.contains(&c)
    }

    fn decode_viet(&self, source: &str) -> String {
        let mut res = String::new();
        let mut content_iter = source.chars().peekable();

        while let Some(c) = content_iter.peek() {
            if !self.is_viet_letter(*c) {
                res.push(*c);
                content_iter.next();
            } else {
                self.convert_word(&mut res, &mut content_iter);
            }
        }

        res
    }

    fn convert_word(&self, s: &mut String, content: impl Iterator<Item = char>) {
        let mut state = State::default();

        for c in content {
            if !self.is_viet_letter(c) {
                s.push_str(&state.to_string());
                s.push(c);

                return;
            }
            let sc: char = c.to_lowercase().next().unwrap();
            let up = c != sc;
            match sc {
                'd' => {
                    push_may_up(s, 'd', up);
                    if state.d {
                        s.push('d')
                    }
                    state.d = true;
                }
                'đ' => {
                    state.double_d = true;
                    state.d = true;
                    push_may_up(s, 'd', up);
                }
                // the a
                'a' => {
                    push_may_up(s, 'a', up);
                    if state.a {
                        s.push('a');
                    }
                    state.a = true;
                    state.vowels = true;
                }
                'à' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.vowels = true;
                    state.f = true;
                }
                'á' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.vowels = true;
                    state.s = true;
                }
                'ả' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.vowels = true;
                    state.r = true;
                }
                'ã' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.vowels = true;
                    state.x = true;
                }
                'ạ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.vowels = true;
                    state.j = true;
                }
                // the â
                'â' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.double_a = true;
                    state.vowels = true;
                }
                'ầ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.double_a = true;
                    state.vowels = true;
                    state.f = true;
                }
                'ấ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.double_a = true;
                    state.vowels = true;
                    state.s = true;
                }
                'ẩ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.double_a = true;
                    state.vowels = true;
                    state.r = true;
                }
                'ẫ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.double_a = true;
                    state.vowels = true;
                    state.x = true;
                }
                'ậ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.double_a = true;
                    state.vowels = true;
                    state.j = true;
                }
                // the ă
                'ă' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.w = true;
                    state.vowels = true;
                }
                'ằ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.w = true;
                    state.vowels = true;
                    state.f = true;
                }
                'ắ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.w = true;
                    state.vowels = true;
                    state.s = true;
                }
                'ẳ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.w = true;
                    state.vowels = true;
                    state.r = true;
                }
                'ẵ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.w = true;
                    state.vowels = true;
                    state.x = true;
                }
                'ặ' => {
                    push_may_up(s, 'a', up);
                    state.a = true;
                    state.w = true;
                    state.vowels = true;
                    state.j = true;
                }
                // the e
                'e' => {
                    push_may_up(s, 'e', up);
                    if state.e {
                        s.push('e')
                    }
                    state.e = true;
                    state.vowels = true;
                }
                'è' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.vowels = true;
                    state.f = true;
                }
                'é' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.vowels = true;
                    state.s = true;
                }
                'ẻ' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.vowels = true;
                    state.r = true;
                }
                'ẽ' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.vowels = true;
                    state.x = true;
                }
                'ẹ' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.vowels = true;
                    state.j = true;
                }
                // the ê
                'ê' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.double_e = true;
                    state.vowels = true;
                }
                'ề' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.double_e = true;
                    state.vowels = true;
                    state.f = true;
                }
                'ế' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.double_e = true;
                    state.vowels = true;
                    state.s = true;
                }
                'ể' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.double_e = true;
                    state.vowels = true;
                    state.r = true;
                }
                'ễ' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.double_e = true;
                    state.vowels = true;
                    state.x = true;
                }
                'ệ' => {
                    push_may_up(s, 'e', up);
                    state.e = true;
                    state.double_e = true;
                    state.vowels = true;
                    state.j = true;
                }
                // the o
                'o' => {
                    push_may_up(s, 'o', up);
                    if state.o {
                        s.push('o')
                    }
                    state.o = true;
                    state.vowels = true;
                }
                'ò' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.vowels = true;
                    state.f = true;
                }
                'ó' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.vowels = true;
                    state.s = true;
                }
                'ỏ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.vowels = true;
                    state.r = true;
                }
                'õ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.vowels = true;
                    state.x = true;
                }
                'ọ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.vowels = true;
                    state.j = true;
                }
                // the ô
                'ô' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.double_o = true;
                    state.vowels = true;
                }
                'ồ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.double_o = true;
                    state.vowels = true;
                    state.f = true;
                }
                'ố' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.double_o = true;
                    state.vowels = true;
                    state.s = true;
                }
                'ổ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.double_o = true;
                    state.vowels = true;
                    state.r = true;
                }
                'ỗ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.double_o = true;
                    state.vowels = true;
                    state.x = true;
                }
                'ộ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.double_o = true;
                    state.vowels = true;
                    state.j = true;
                }
                // the ơ
                'ơ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.w = true;
                    state.vowels = true;
                }
                'ờ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.w = true;
                    state.vowels = true;
                    state.f = true;
                }
                'ớ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.w = true;
                    state.vowels = true;
                    state.s = true;
                }
                'ở' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.w = true;
                    state.vowels = true;
                    state.r = true;
                }
                'ỡ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.w = true;
                    state.vowels = true;
                    state.x = true;
                }
                'ợ' => {
                    push_may_up(s, 'o', up);
                    state.o = true;
                    state.w = true;
                    state.vowels = true;
                    state.j = true;
                }
                // the u
                'u' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                }
                'ù' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.f = true;
                }
                'ú' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.s = true;
                }
                'ủ' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.r = true;
                }
                'ũ' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.x = true;
                }
                'ụ' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.j = true;
                }
                // the ư
                'ư' => {
                    if state.vowels {
                        push_may_up(s, 'u', up)
                    } else {
                        push_may_up(s, 'w', up);
                        state.already_w = true;
                    }
                    state.vowels = true;
                    state.w = true;
                }
                'ừ' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.w = true;
                    state.f = true;
                }
                'ứ' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.w = true;
                    state.s = true;
                }
                'ử' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.w = true;
                    state.r = true;
                }
                'ữ' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.w = true;
                    state.x = true;
                }
                'ự' => {
                    push_may_up(s, 'u', up);
                    state.vowels = true;
                    state.w = true;
                    state.j = true;
                }
                // the y
                'y' => {
                    push_may_up(s, 'y', up);
                    state.vowels = true;
                }
                'ỳ' => {
                    push_may_up(s, 'y', up);
                    state.vowels = true;
                    state.f = true;
                }
                'ý' => {
                    push_may_up(s, 'y', up);
                    state.vowels = true;
                    state.s = true;
                }
                'ỷ' => {
                    push_may_up(s, 'y', up);
                    state.vowels = true;
                    state.r = true;
                }
                'ỹ' => {
                    push_may_up(s, 'y', up);
                    state.vowels = true;
                    state.x = true;
                }
                'ỵ' => {
                    push_may_up(s, 'y', up);
                    state.vowels = true;
                    state.j = true;
                }
                // the i
                'i' => {
                    push_may_up(s, 'i', up);
                    state.vowels = true;
                }
                'ì' => {
                    push_may_up(s, 'i', up);
                    state.vowels = true;
                    state.f = true;
                }
                'í' => {
                    push_may_up(s, 'i', up);
                    state.vowels = true;
                    state.s = true;
                }
                'ỉ' => {
                    push_may_up(s, 'i', up);
                    state.vowels = true;
                    state.r = true;
                }
                'ĩ' => {
                    push_may_up(s, 'i', up);
                    state.vowels = true;
                    state.x = true;
                }
                'ị' => {
                    push_may_up(s, 'i', up);
                    state.vowels = true;
                    state.j = true;
                }
                'f' | 's' | 'x' | 'j' | 'w' => {
                    push_may_up(s, sc, up);
                    if state.vowels {
                        s.push(sc);
                    }
                }

                // the rest
                _ => push_may_up(s, sc, up),
            };
        }
        s.push_str(&state.to_string());
    }
}

#[derive(Debug, Clone, Default)]
struct State {
    d: bool,
    o: bool,
    e: bool,
    a: bool,
    double_d: bool,
    double_o: bool,
    double_e: bool,
    double_a: bool,
    f: bool,
    s: bool,
    r: bool,
    x: bool,
    j: bool,
    w: bool,
    already_w: bool,
    vowels: bool,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if self.double_d {
            s.push('d')
        }
        if self.double_o {
            s.push('o')
        }
        if self.double_e {
            s.push('e')
        }
        if self.double_a {
            s.push('a')
        }
        if self.f {
            s.push('f')
        }
        if self.s {
            s.push('s')
        }
        if self.r {
            s.push('r')
        }
        if self.j {
            s.push('j')
        }
        if self.x {
            s.push('x')
        }
        if self.w && !self.already_w {
            s.push('w')
        }
        write!(f, "{}", s)
    }
}

fn push_may_up(s: &mut String, c: char, up: bool) {
    if up {
        let cup: char = c.to_uppercase().next().unwrap();
        s.push(cup);
    } else {
        s.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        let start = include_str!("../example-output/25_lines_start.txt");
        let reference = include_str!("../example-output/25_lines_res.txt");

        let decoder = VietTelex::new();
        let converted = decoder.decode_viet(start);

        assert_eq!(converted, reference);
    }
}
