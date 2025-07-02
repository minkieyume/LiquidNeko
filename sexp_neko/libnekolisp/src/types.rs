use alloc::{vec::Vec, string::String, boxed::Box ,rc::Rc};
use alloc::string::ToString;
use hashbrown::HashMap;
use core::hash::{Hash, Hasher};
use core::ops::{Add,Sub,Mul,Div,Fn};
use core::cmp::{Eq,PartialEq};
use core::cell::RefCell;
use core::borrow::Borrow;
use NekoValue::*;
use crate::env::Env;
use crate::nekocore;

#[derive(Clone)]
pub enum NekoValue {
    NekoSymbol(Symbol),
    Nekoi64(i64),
    Nekof64(f64),
    NekoString(String),
    NekoChar(char),
    NekoBool(Option<bool>),
    NekoKeyword(String),
    NekoList(Vec<NekoType>),
    NekoDict(HashMap<NekoType,NekoType>),
    NekoFn(Function),
    NekoErr(String),
    NekoAtom(Rc<RefCell<Atom>>),
    NekoStruct,
    NekoNeko(Env),
    NekoTrue,
    NekoNil
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Symbol(pub String);

#[derive(Clone, Hash)]
pub struct NekoType(pub Rc<NekoValue>);

#[derive(Clone,Hash,PartialEq)]
pub struct Atom(pub NekoType);

#[derive(Clone)]
pub struct Function {
    boxed:Option<Rc<Box<dyn Fn(Vec<NekoType>,Env) -> NekoType>>>,
    ast:Option<Vec<NekoType>>,
    is_box:bool,
    is_special_form:bool,
    is_marco:bool,
    print:String,
}

impl Atom {
    pub fn set_neko(&mut self,neko:NekoType) {
        self.0 = neko
    }
    
    pub fn get_neko(&self) -> NekoType{
        return self.0.clone()
    }
}

impl NekoType {
    pub fn nil() -> NekoType {
        NekoType(Rc::new(NekoNil))
    }

    pub fn is_nil(&self) -> bool {
        match *self.0 {
            NekoNil => true,
            _ => false,
        }
    }

    pub fn symbol(s:String) -> NekoType {
        NekoType(Rc::new(NekoSymbol(Symbol(s))))
    }

    pub fn from_symbol(s:Symbol) -> NekoType {
        NekoType(Rc::new(NekoSymbol(s)))
    }

    pub fn is_symbol(&self) -> bool {
        match *self.0 {
            NekoSymbol(_) => true,
            _ => false,
        }
    }

    pub fn atom(n:NekoType) -> NekoType {
        let neko_atom = NekoAtom(Rc::new(RefCell::new(Atom(n))));
        NekoType(Rc::new(neko_atom))
    }

    pub fn is_atom(&self) -> bool {
        match *self.0 {
            NekoAtom(_) => true,
            _ => false,
        }
    }    

    pub fn int_64(i:i64) -> NekoType {
        NekoType(Rc::new(Nekoi64(i)))
    }

    pub fn is_int(&self) -> bool {
        match *self.0 {
            Nekoi64(_) => true,
            _ => false,
        }
    }
    
    pub fn float_64(f:f64) -> NekoType {
        NekoType(Rc::new(Nekof64(f)))
    }

    pub fn is_float(&self) -> bool {
        match *self.0 {
            Nekof64(_) => true,
            _ => false,
        }
    }

    pub fn string(s:String) -> NekoType {
        NekoType(Rc::new(NekoString(s)))
    }

    pub fn is_string(&self) -> bool {
        match *self.0 {
            NekoString(_) => true,
            _ => false,
        }
    }

    pub fn neko_char(c:char) -> NekoType {
        NekoType(Rc::new(NekoChar(c)))
    }

    pub fn is_char(&self) -> bool {
        match *self.0 {
            NekoChar(_) => true,
            _ => false,
        }
    }

    pub fn list(v:Vec<NekoType>) -> NekoType {
        NekoType(Rc::new(NekoList(v)))
    }

    pub fn is_list(&self) -> bool {
        match *self.0 {
            NekoList(_) => true,
            _ => false,
        }
    }

    pub fn is_no_empty_list(&self) -> bool {
        let n = self.get_ref();
        match n.borrow() {
            NekoList(l) => {
                if l.len() > 0 {
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }

    pub fn keyword(s:String) -> NekoType {
        NekoType(Rc::new(NekoKeyword(s)))
    }

    pub fn is_keyword(&self) -> bool {
        match *self.0 {
            NekoKeyword(_) => true,
            _ => false,
        }
    }

    pub fn err(s:String) -> NekoType {
        NekoType(Rc::new(NekoErr(s)))
    }

    pub fn dict(d:HashMap<NekoType,NekoType>) -> NekoType {
        NekoType(Rc::new(NekoDict(d)))
    }

    pub fn is_dict(&self) -> bool {
        match *self.0 {
            NekoDict(_) => true,
            _ => false,
        }
    }

    pub fn neko(e:Env) -> NekoType {
        NekoType(Rc::new(NekoNeko(e)))
    }

    pub fn is_neko(&self) -> bool {
        match *self.0 {
            NekoNeko(_) => true,
            _ => false,
        }
    }

    pub fn is_err(&self) -> bool {
        match *self.0 {
            NekoErr(_) => true,
            _ => false,
        }
    }

    pub fn neko_bool(o:Option<bool>) -> NekoType {
        NekoType(Rc::new(NekoBool(o)))
    }

    pub fn is_bool(&self) -> bool {
        match *self.0 {
            NekoBool(_) => true,
            _ => false,
        }
    }

    pub fn neko_true() -> NekoType {
        NekoType(Rc::new(NekoTrue))
    }

    pub fn is_true(&self) -> bool {
        match *self.0 {
            NekoTrue => true,
            _ => false,
        }
    }

    pub fn func(func:Function) -> NekoType {
        NekoType(Rc::new(NekoFn(func)))
    }

    pub fn is_fn(&self) -> bool{
        match *self.0 {
            NekoFn(_) => true,
            _ => false,
        }
    }

    pub fn get_type(&self) -> NekoType {
        let s:&str = self.get_type_str();
        NekoType(Rc::new(NekoString(s.to_string())))
    }

    pub fn get_type_str(&self) -> &str {
        match *self.0 {
            NekoList(_) => "list",
            NekoBool(_) => "bool",
            NekoChar(_) => "char",
            NekoSymbol(_) => "symbol",
            Nekoi64(_) => "int",
            Nekof64(_) => "float",
            NekoString(_) => "string",
            NekoErr(_) => "err",
            NekoKeyword(_) => "keyword",
            NekoNeko(_) => "neko",
            NekoDict(_) => "dictionary",
            NekoFn(_) => "function",
            NekoTrue => "true",
            NekoNil => "nil",
            _ => "unknown",
        }
    }

    pub fn copy_value(&self) -> NekoValue {
        return (*self.0).clone();
    }

    pub fn get_ref(&self) -> Rc<NekoValue> {
        return self.0.clone();
    }
}

impl Symbol {
    pub fn val(&self) -> String {
        self.0.clone()
    }
    
    pub fn is_fuction_identifier(&self,env:Env) -> bool {
        let symbol = env.get_symbol();
        let mut symbol_chars = self.0.chars();
        let first = symbol_chars.next();
        if let Some(c) = first {
            return symbol.pair_char(c,"function_identifier");
        }
        return false;
    }

    pub fn split_namespace(&self,env:Env) -> Vec<&str> {
        let symbols = env.get_symbol();
        let nc = symbols.get_char("namespace");
        let namespace = nc.unwrap();
        self.0.split(namespace).collect()
    }
}

impl Function {
    pub fn is_special(&self) -> bool{
        return self.is_special_form
    }

    pub fn is_box(&self) -> bool {
        return self.is_box
    }

    pub fn is_marco(&self) -> bool {
        return self.is_marco
    }

    pub fn set_is_marco(&mut self,i_marco:bool) {
        self.is_marco = i_marco;
    }

    pub fn new_box(_box:Rc<Box<dyn Fn
                                   (Vec<NekoType>,Env) -> NekoType>>,
    pr:&str,special:bool) -> Function {
        Function {
            boxed:Some(_box),
            ast:None,
            is_box:true,
            is_special_form:special,
            is_marco:false,
            print:pr.to_string(),
        }
    }

    pub fn new_marco_box(_box:Rc<Box<dyn Fn
                                   (Vec<NekoType>,Env) -> NekoType>>,
    pr:&str) -> Function {
        Function {
            boxed:Some(_box),
            ast:None,
            is_box:true,
            is_special_form:true,
            is_marco:true,
            print:pr.to_string(),
        }
    }

    pub fn print(&self) -> &str {
        return self.print.as_str()
    }

    pub fn new_ast(_ast:Vec<NekoType>,pr:&str,special:bool) -> Function {
        Function {
            boxed:None,
            ast:Some(_ast),
            is_box:false,
            is_special_form:special,
            is_marco:false,
            print:pr.to_string(),
        }
    }

    pub fn new_marco(_ast:Vec<NekoType>,pr:&str) -> Function {
        Function {
            boxed:None,
            ast:Some(_ast),
            is_box:false,
            is_special_form:true,
            is_marco:true,
            print:pr.to_string(),
        }
    }

    fn call_ast(&self,mut args:Vec<NekoType>,env:Env) -> NekoType {
        //调用ast类型的函数时会运行的方法
        //ast类型函数代表的是由NekoLisp脚本声明和自定义的函数。
        let unwrap = self.ast.as_ref().unwrap();
        let mut ast = unwrap.clone();
        let n_env = Env::new(Some(env.clone()));
        //println!("{}",pr_str(NekoType::list(ast.clone())));
        //println!("{}",pr_str(NekoType::list(args.clone())));
        let params = ast.remove(0);
        if let NekoList(v) = params.copy_value() {
            let mut cfid = "".to_string();
            for p in v { //循环：遍历所有形参
                if let NekoSymbol(s) = p.copy_value() {
                    if s.is_fuction_identifier(n_env.clone()) {
                        //读取函数标识符
                        cfid = s.val();
                        continue;
                    }
                }
                
                match cfid.as_str() {
                    "&rest" => { //可变参数的处理
                        if let NekoSymbol(s) = p.copy_value() {
                            let val = NekoType::list(args);
                            n_env.set(s.clone(),val);
                            break;
                        }
                    },
                    "&optional" => { //可选参数的处理
                        if !args.is_empty() { //参数不为空
                            let mut ns = p.copy_value(); //形参
                            if let NekoList(mut l) = p.copy_value() {
                                if l.len() == 2 {
                                    let n = l.remove(0);
                                    ns = n.copy_value();
                                }
                            }
                            if let NekoSymbol(s) = ns {
                                let val = args.remove(0);
                                n_env.set(s.clone(),val);
                            }
                        } else if let NekoList(mut l) = p.copy_value() {
                            //参数已空但是还有带默认值的形参没处理
                            if l.len() == 2 {
                                let n = l.remove(0);
                                if let NekoSymbol(s) = n.copy_value() {
                                    let val = l.remove(0);
                                    n_env.set(s.clone(),val);
                                }
                            }
                        }
                    },
                    "&key" => {
                        if !args.is_empty() {
                            let mut ns = p.copy_value(); //形参
                            if let NekoList(mut l) = p.copy_value() {
                                //获取默认值列表中的形参
                                if l.len() == 2 {
                                    let n = l.remove(0);
                                    let val = l.remove(0);
                                    ns = n.copy_value();
                                    if let NekoSymbol(symb) = n.copy_value() {
                                        n_env.set(symb.clone(),val);
                                    }
                                }
                            }
                            if let NekoSymbol(s) = ns { //处理形参的符号
                                let mut key = s.val();
                                key.insert(0,':');
                                let mut position:usize = 0;
                                for arg in args.clone() {
                                    if let NekoKeyword(keyword) = arg.copy_value() {
                                        if keyword != key {
                                            continue;
                                        }
                                        let mut vals:Vec<NekoType> = Vec::new();
                                        //到下一个关键字为止，将所有参数提取出来
                                        for val in &args[position+1..] {
                                            if !val.is_keyword() {
                                                vals.push(val.clone());
                                            } else {
                                                break;
                                            }
                                        }
                                        if vals.len() > 1 {
                                            let val = NekoType::list(vals);
                                            n_env.set(s.clone(),val);
                                        } else if vals.len() == 1 {
                                            n_env.set(s.clone(),vals.remove(0));
                                        } else {
                                            return NekoType::err("关键字后至少应有一个参数".to_string())
                                        }
                                    }
                                    position = position + 1;
                                }
                            }
                        } else if let NekoList(mut l) = p.copy_value() {
                            //参数已空但是还有带默认值的形参没处理
                            if l.len() == 2 {
                                let n = l.remove(0);
                                if let NekoSymbol(s) = n.copy_value() {
                                    let val = l.remove(0);
                                    n_env.set(s.clone(),val);
                                }
                            }
                        }
                    },
                    _ => {
                        if args.is_empty() { //循环内参数为空报错
                            return NekoType::err("函数接收的参数量不足".to_string());
                        }
                        if let NekoSymbol(s) = p.copy_value() {
                            let val = args.remove(0);
                            n_env.set(s.clone(),val);
                        }
                    }
                }
            }
        };
        let progn = NekoType::symbol(
            nekocore::PROGN.to_string());        
        ast.insert(0,progn);
        let result = NekoType::list(ast);
        env.set_tco(n_env.clone());
//        for body in ast {
//            result = eval(body,n_env.clone());
//        }
        return result;
    }
    
    pub fn call(&self,v:Vec<NekoType>,e:Env) -> NekoType {
        if self.is_box() {
            let l =
                self.boxed.as_ref().unwrap();
            (*l)(v,e.clone())
        } else {
            self.call_ast(v,e.clone())
        }
    }
}

impl Add for NekoType {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        match self.copy_value() {
            NekoList(mut a) => {
                if let NekoList(mut b) = other.copy_value() {
                    a.append(&mut b);
                    return Self::list(a);
                }
            },
            NekoSymbol(a) => {
                if let NekoSymbol(b) = other.copy_value() {
                    let mut c = a.val();
                    c.push_str(b.val().as_str());
                    return Self::symbol(c);
                }
            },
            Nekoi64(a) => {
                if let Nekoi64(b) = other.copy_value() {
                    return Self::int_64(a+b);
                }
            },
            Nekof64(a) => {
                if let Nekof64(b) = other.copy_value() {
                    return Self::float_64(a+b);
                }
            },
            NekoString(mut a) => {
                if let NekoString(b) = other.copy_value() {
                    a.push_str(b.as_str());
                    return Self::string(a);
                }
            },
            NekoKeyword(mut a) => {
                if let NekoKeyword(b) =  other.copy_value() {
                    a.push_str(b.as_str());
                    return Self::keyword(a);
                }
            }
            _ => {}
        }
        Self::err("不支持的操作".to_string())
    }
}

impl Sub for NekoType {
        type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        match self.copy_value() {
            Nekoi64(a) => {
                if let Nekoi64(b) = other.copy_value() {
                    return Self::int_64(a-b);
                }
            },
            Nekof64(a) => {
                if let Nekof64(b) = other.copy_value() {
                    return Self::float_64(a-b);
                }
            },
            _ => {}
        }
        Self::err("不支持的操作".to_string())
    }
}

impl Mul for NekoType {
        type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        match self.copy_value() {
            Nekoi64(a) => {
                if let Nekoi64(b) = other.copy_value() {
                    return Self::int_64(a*b);
                }
            },
            Nekof64(a) => {
                if let Nekof64(b) = other.copy_value() {
                    return Self::float_64(a*b);
                }
            },
            _ => {}
        }
        Self::err("不支持的操作".to_string())
    }
}

impl Div for NekoType {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        match self.copy_value() {
            Nekoi64(a) => {
                if let Nekoi64(b) = other.copy_value() {
                    if b == 0 {
                        return Self::err("0不能被除".to_string());
                    } else {
                        let c:f64 = a as f64 / b as f64;
                        if fract_f64(c) != 0.0 {
                            return Self::float_64(c)
                        } else{
                            return Self::int_64(c as i64);
                        }
                    }
                } else if let Nekof64(b) = other.copy_value() {
                    if b == 0.0 {
                        return Self::err("0不能被除".to_string());
                    } else {
                        let c:f64 = a as f64 / b as f64;
                        if fract_f64(c) != 0.0 {
                            return Self::float_64(c)
                        } else{
                            return Self::int_64(c as i64);
                        }
                    }
                }
            },
            Nekof64(a) => {
                if let Nekof64(b) = other.copy_value() {
                    if b == 0.0 {
                        return Self::err("0不能被除".to_string());
                    } else {
                        return Self::float_64(a/b);
                    }
                } else if let Nekoi64(b) = other.copy_value() {
                    if b == 0 {
                        return Self::err("0不能被除".to_string());
                    } else {
                        let c:f64 = a as f64 / b as f64;
                        if fract_f64(c) != 0.0 {
                            return Self::float_64(c)
                        } else{
                            return Self::int_64(c as i64);
                        }
                    }
                }
            },
            _ => {}
        }
        Self::err("不支持的操作".to_string())
    }
}

fn fract_f64(x:f64) -> f64 {
    let fl = floor_f64(x) as f64;
    return x - fl;

}

fn floor_f64(x:f64) -> i64 {
    unsafe {
        let truncated = x.to_int_unchecked::<i64>();
        if x < 0.0 && x - truncated as f64 != 0.0 {
            return truncated - 1;
        }
        return truncated;
    }
}

impl PartialEq for NekoType {
    fn eq(&self, other: &Self) -> bool {
        match self.copy_value() {
            Nekoi64(a) => {
                if let Nekoi64(b) = other.copy_value() {
                    return a==b;
                } else if let Nekof64(b) = other.copy_value() {
                    if fract_f64(b) == 0.0 {
                        return a == b as i64;
                    }
                }
            },
            Nekof64(a) => {
                if let Nekof64(b) = other.copy_value() {
                    return a==b;
                } else if let Nekoi64(b) = other.copy_value() {
                    if fract_f64(a) == 0.0 {
                        return a as i64 == b;
                    }
                }
            },
            NekoString(a) => {
                if let NekoString(b)  = other.copy_value() {
                    return a == b;
                }
            },
            NekoSymbol(a) => {
                if let NekoSymbol(b)  = other.copy_value() {
                    return a == b;
                }
            },
            NekoKeyword(a) => {
                if let NekoKeyword(b)  = other.copy_value() {
                    return a == b;
                }
            },
            NekoList(al) => {
                if let NekoList(bl)  = other.copy_value() {
                    return al == bl;
                }
            },
            _ => {}
        }
        false
    }
}

impl Eq for NekoType {}

impl Hash for NekoValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self{
            Self::NekoSymbol(s) => {s.hash(state);},
            Self::NekoList(l) => {
                for n in l {
                    n.hash(state);
                }
            },
            Self::NekoString(s) => {s.hash(state);},
            Self::NekoChar(s) => {s.hash(state);},
            Self::NekoKeyword(s) => {s.hash(state);},
            Self::Nekoi64(i) => {i.to_string().hash(state);},
            Self::Nekof64(i) => {i.to_string().hash(state);},
            Self::NekoErr(e) => {e.hash(state);},
            Self::NekoBool(b) => {b.hash(state);},
            Self::NekoTrue => {true.hash(state);},
            Self::NekoNil => {false.hash(state);},
            Self::NekoAtom(atom) => {
                let a = atom.borrow_mut();
                a.hash(state)
            },
            Self::NekoDict(d) => {
                for key in d.keys() {
                    key.hash(state)
                }
                for val in d.values() {
                    val.hash(state)
                }
            },
            _ => {},
        }
    }
}
