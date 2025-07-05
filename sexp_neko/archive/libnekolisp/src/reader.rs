use alloc::{vec::Vec, string::String, boxed::Box ,str::Chars};
use alloc::vec;
use alloc::borrow::ToOwned;
use alloc::string::ToString;
use core::fmt::Write;
use crate::env::Env;
use crate::types::NekoType;

pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    pub fn peek(&self) -> Option<String> {
        if self.tokens.len() > self.position {
            Some(self.tokens[self.position].to_owned())
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<String> {
        //println!("{}",self.position);
        if let Some(token) = self.peek() {
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
}

pub fn pre_read_str(s:&str,env:Env) -> Vec<String> {
    let mut s_chars = s.chars();
    return pre_read_form(&mut s_chars,env);
}

fn pre_read_form(s_chars:&mut Chars<'_>,env:Env) -> Vec<String> {
    let mut sexps:Vec<String> = Vec::new();
    let mut sexp = String::new();
    let symb = env.get_symbol();
    let mut s_exp_in_reader_marco = false;
    while let Some(c) = s_chars.next() {
        if symb.pair_char(c,"s_exp_begin") {
            if s_exp_in_reader_marco {
                s_exp_in_reader_marco = false
            } else if !sexp.is_empty() {
                sexps.push(sexp.clone());
                sexp.clear();
            }
            sexp.push(c);
            sexp.push_str(pre_read_list(s_chars,env.clone()).as_str());
            sexps.push(sexp.clone());
            sexp.clear();
        } else if symb.pair_char(c,"split") {
            if !sexp.is_empty() {
                sexps.push(sexp.clone());
                sexp.clear();
            }
        } else {
            sexp.push(c);
            if symb.is_str_reader_marco(&sexp) {
                s_exp_in_reader_marco = true;
            }
        }
    }
    return sexps;
}

fn pre_read_list(s_chars:&mut Chars<'_>,env:Env) -> String {
    let mut sexp = String::new();
    let symb = env.get_symbol();
    while let Some(c) = s_chars.next() {
        sexp.push(c);
        if symb.pair_char(c,"s_exp_begin") {
            sexp.push_str(pre_read_list(s_chars,env.clone()).as_str());
        } else if symb.pair_char(c,"s_exp_end") {
            return sexp;
        }
    }
    return sexp;
}

pub fn read_str(s:&str,env:Env) -> NekoType {
    let t = tokenize(s,env.clone());
    if !t.is_empty() {
        let mut r = Reader {
            tokens: t,
            position: 0,
        };
        read_form(&mut r,env.clone())
    } else {
        NekoType::nil()
    }
}
pub fn tokenize(s:&str,env:Env) -> Vec<String> {
    #[allow(unused_assignments)]
    let mut last_tokens:Vec<String> = Vec::new();
    let mut tokens:Vec<String> = Vec::new();
    let mut token:String = String::new();
    let symb = env.get_symbol();

    //循环1,处理空格、逗号、引号、分号与换行
    let mut last_char = ' ';
    let mut quoting = false;
    let mut commenting = false;
    for c in s.chars() {
        if symb.pair_char(c,"quote_symbol") {
            //处理引号
            if !quoting && !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
            token.push(c);  // quote 本身加进去
            if !symb.pair_char(last_char,"change_mean") {
                quoting = !quoting;
            }

            // 如果刚结束字符串，提交 token
            if !quoting {
                tokens.push(token.clone());
                token.clear();
            }
            continue;
        } else if symb.pair_char(c,"comment_begin") && !quoting {
            //处理分号
            tokens.push(token.clone());
            token.clear();
            commenting = true;
            continue;
        } else if symb.pair_char(c,"comment_end") && !quoting {
            //处理comment结束符号
            commenting = false;   
        }
        
        if commenting {
            last_char = ' ';
            continue;
        } else if quoting {
            // 在引号时，照单全收
            token.push(c);
        } else if symb.pair_char(c,"split") {
            // 分隔符，提交 token（如果非空）
            if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
        } else {
            // 普通字符
            token.push(c);
        }
        //println!("{}",c);
        if symb.pair_char(c,"change_mean") && symb.pair_char(last_char,"change_mean") {
            last_char = ' ';
        } else{
            last_char = c;
        }
    }
    // 处理最后一个 token
    if !token.is_empty() {
        tokens.push(token.clone());
        token.clear();
    }

    //println!("{:?}", tokens);
    // 准备下一组匹配循环
    last_tokens = tokens.clone();
    tokens.clear();
    last_char = ' ';

    //循环2:分离宏字符与普通字符（特殊的宏字符与普通字符区分）
    for last_token in last_tokens {
        let mut pushed = false;
        for c in last_token.chars() {
            if symb.pair_char(c,"quote_symbol") || symb.pair_char(c,"comment_begin") {
                //判断是否是引号或注释符号，是的话忽略本token
                tokens.push(last_token.clone());
                pushed = true;
                break;
            } else if let Some(_) = symb.sexp_direction(c) {
                //匹配S表达式符号（括号）并单独分开
                if !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                token.push(c);
                tokens.push(token.clone());
                token.clear();
            } else if symb.is_char_reader_marco(c) {
                //匹配宏符号
                if !symb.is_char_reader_marco(last_char) && !token.is_empty() {
                    tokens.push(token.clone());
                    token.clear();
                }
                token.push(c);
            } else {
                //普通字符处理
                if symb.is_char_reader_marco(last_char) {
                    tokens.push(token.clone());
                    token.clear();
                }
                token.push(c);
            }
            last_char = c;
        }
        if !pushed {
            if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
        }
    }
    
    //println!("{:?}", tokens);

     // 准备下一组匹配循环
    last_tokens = tokens.clone();
    tokens.clear();
//    last_char = ' ';

    // 循环3:区分单字符与宏字符组合
    for last_token in last_tokens {
        if symb.is_str_reader_marco(&last_token) {
            tokens.push(last_token.clone());
        } else {
            let mut pushed = false;
            for c in last_token.chars() {
                if symb.pair_char(c,"quote_symbol") || symb.pair_char(c,"comment_begin") {
                    //判断是否是引号或注释符号，是的话忽略本token
                    tokens.push(last_token.clone());
                    pushed = true;
                    break;
                } else if symb.is_char_reader_marco(c) {
                    token.push(c);
                    tokens.push(token.clone());
                    token.clear();
                    pushed = true; //前面已经处理过字母了，所以只要一个字符是符号，后面也必定是
                }
            }
            if !pushed && !last_token.is_empty() {
                tokens.push(last_token.clone());
                token.clear();
            }
        }
        
    }
    
    //println!("{:?}", tokens);
    return tokens;
}

pub fn read_form(r:&mut Reader,env:Env) -> NekoType {
    //解析Sexp表达式形式
    let symb = env.get_symbol();
    if let Some(c) = r.peek().and_then(|token| token.chars().next()) {
        if let Some(true) = symb.sexp_direction(c) {
            return read_list(r,env.clone());
        } else {
            return read_atom(r,env.clone());
        }
    } else {
        return NekoType::nil();
    }
}

pub fn read_list(r:&mut Reader,env:Env) -> NekoType {
    //解析Sexp表达式本身
    let mut list:Vec<NekoType> = Vec::new();
    let mut last_s:String = " ".to_string();
    let symb = env.get_symbol();
    //反复读入元素：
    while let Some(_) = r.next() {
        if let Some(s) = r.peek() {
            last_s = s.clone();
            let c = s.chars().next().unwrap();
            //println!("{:?}",r.peek());
            if let Some(false) = symb.sexp_direction(c) {
                return NekoType::list(list);
            }
            list.push(read_form(r,env.clone()));
        } else {
            //错误判定条件与判定循环
            let mut err = String::new();
            let mut quoting = false;
            let mut last_char = ' ';
            for c in last_s.chars() {
                if symb.pair_char(c,"quote_symbol") && !symb.pair_char(last_char,"change_mean") {
                    quoting = !quoting
                } else if symb.pair_char(c,"change_mean") && symb.pair_char(last_char,"change_mean") {
                    last_char = ' ';
                } else {
                    last_char = c;
                }
            }

            if quoting {
                write!(&mut err,"引号越界：{}",last_s).unwrap();
                return NekoType::err(err);
            }
            
            return NekoType::err("解析失败，未知错误".to_string());
        }
    }
    return NekoType::err("Sexp表达式没有结尾".to_string());
}

pub fn read_atom(r:&mut Reader,env:Env) -> NekoType {
    //解析Sexp表达式内容
    if let Some(s) = r.peek() {
        let symbol = env.get_symbol();
        if symbol.is_str_reader_marco(s.as_str()){
            return read_reader_marco(r,env.clone());
        } else{
            let result = try_parse(&s,env.clone());
            return result.unwrap_or(NekoType::symbol(s));
        }
    } else {
        return NekoType::symbol("解析失败，未知错误".to_string())
    }
}

pub fn read_reader_marco(r:&mut Reader,env:Env) -> NekoType {
    let mut list:Vec<NekoType> = Vec::new();
    let symbol = env.get_symbol();
    let s = r.next().unwrap();
    //let sp = r.peek().unwrap_or("none".to_string());
    let arg = read_form(r,env.clone());
    //println!("{}",s);
    //println!("{}",sp);
    let marco = symbol.parse_str_reader_marco(s.as_str()).unwrap();
    list.push(marco);
    //println!("{}",arg.get_type_str());
    list.push(arg);
    return NekoType::list(list);
}

fn try_parse(s:&str,env:Env) -> Option<NekoType>{
    let parsers: Vec<Box<dyn Fn(&str,Env) -> Option<NekoType>>> = vec![
        Box::new(|s,env| parse_integer(s,env.clone()).map(NekoType::int_64)),
        Box::new(|s,env| parse_float(s,env.clone()).map(NekoType::float_64)),
        Box::new(|s,env| parse_keyword(s,env.clone()).map(NekoType::keyword)),
        Box::new(|s,env| parse_string(s,env.clone()).map(NekoType::string)),
    ];

    match s {
        "nil" => {
            return Some(NekoType::nil())
        },
        "T" => {
            return Some(NekoType::neko_true())
        },
        _ => {}
    };
    
    for parser in parsers {
        if let Some(val) = parser(s,env.clone()) {
            return Some(val);
        }
    }
    None
}

#[allow(dead_code)]
fn parse_symbol(_s: &str,_env:Env) -> Option<String> {
    None
}

fn parse_keyword(s: &str,env:Env) -> Option<String> {
    if let Some(c) = s.chars().next() {
        let symb = env.get_symbol();
        if symb.pair_char(c,"keyword") {
            return Some(s.to_string());
        }
    }
    None
}

fn parse_string(s: &str,env:Env) -> Option<String> {
    if let Some(c) = s.chars().next() {
        let symb = env.get_symbol();
        //第一个字符是引号
        if symb.pair_char(c,"quote_symbol") {
            let mut t =  String::new();
            let mut last_char = ' ';
            for tc in s.chars() {
                if symb.pair_char(last_char,"change_mean") {
                    if symb.pair_char(tc,"change_mean"){
                        t.push('\\');
                    } else if tc == 'n' {
                        t.push('\n');
                    } else if symb.pair_char(tc,"quote_symbol") {
                        t.push('"');
                    }
                } else if !symb.pair_char(tc,"change_mean") {
                    t.push(tc);
                }
                last_char = tc
            }
            t.remove(0);
            t.pop();
            return Some(t);
        }
    }
    return None;
}

fn parse_float(s: &str,_env:Env) -> Option<f64> {
    s.parse::<f64>().ok()
}

fn parse_integer(s: &str,_env:Env) -> Option<i64> {
    s.parse::<i64>().ok()
}
