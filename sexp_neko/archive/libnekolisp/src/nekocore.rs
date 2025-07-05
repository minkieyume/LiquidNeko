use alloc::{boxed::Box,vec::Vec,rc::Rc};
use core::borrow::Borrow;
use alloc::string::ToString;
use hashbrown::HashMap;
use crate::types::Symbol;
use crate::types::NekoType;
use crate::types::Function;
use crate::types::NekoValue::*;
use crate::env::Env;
use crate::reader::*;
use crate::eval::*;
use crate::printer::*;

pub const ADD: &str = "+";
pub const SUB: &str = "-";
pub const MUL: &str = "*";
pub const DIV: &str = "/";
pub const LIST: &str = "list";
pub const LISTP: &str = "list?";
pub const EMPTYP: &str = "empty?";
pub const COUNT: &str = "count";
pub const CONS: &str = "cons";
pub const CONCAT: &str = "concat";
pub const READSTRING: &str = "read-string";
pub const ATOM: &str = "atom";
pub const ATOMP: &str = "atom?";
pub const DEREF: &str = "deref";
pub const RESET: &str = "reset";
pub const SWAP: &str = "swap";
pub const DEF: &str = "def";
pub const LET: &str = "let";
pub const IF: &str = "if";
pub const PROGN: &str = "progn";
pub const QUOTE: &str = "quote";
pub const QUASIQUOTE: &str = "quasiquote";
pub const LAMBDA: &str = "lambda";
pub const DEFMARCO: &str = "defmarco";
pub const MARCOEXPAND: &str = "marcoexpand";

pub struct Core {
    pub binddings:HashMap<Symbol, NekoType>
}

impl Core {
    pub fn default() -> Core {
        let mut binds:HashMap<Symbol, NekoType> = HashMap::new();
        let add =
            Function::new_box(Rc::new(Box::new(|v,_e| add_v(v))),"ADD",false);
        binds.insert(Symbol(ADD.to_string()),NekoType::func(add));
        let sub =
            Function::new_box(Rc::new(Box::new(|v,_e| sub_v(v))),"SUB",false);
        binds.insert(Symbol(SUB.to_string()),NekoType::func(sub));
        let mul =
            Function::new_box(Rc::new(Box::new(|v,_e| mul_v(v))),"MUL",false);
        binds.insert(Symbol(MUL.to_string()),NekoType::func(mul));
        let div =
            Function::new_box(Rc::new(Box::new(|v,_e| div_v(v))),"DIV",false);
        binds.insert(Symbol(DIV.to_string()),NekoType::func(div));
        let list =
            Function::new_box(Rc::new(Box::new(|v,_e| list(v))),"LIST",false);
        binds.insert(Symbol(LIST.to_string()),NekoType::func(list));
        let listp =
            Function::new_box(Rc::new(Box::new(|v,_e| listp(v))),"LIST?",false);
        binds.insert(Symbol(LISTP.to_string()),NekoType::func(listp));
        let emptyp =
            Function::new_box(Rc::new(Box::new(|v,_e| emptyp(v))),"EMPTY?",false);
        binds.insert(Symbol(EMPTYP.to_string()),NekoType::func(emptyp));
        let count =
            Function::new_box(Rc::new(Box::new(|v,_e| count(v))),"COUNT",false);
        binds.insert(Symbol(COUNT.to_string()),NekoType::func(count));
        let cons =
            Function::new_box(Rc::new(Box::new(|v,_e| cons(v))),"CONS",false);
        binds.insert(Symbol(CONS.to_string()),NekoType::func(cons));
        let concat =
            Function::new_box(Rc::new(Box::new(|v,_e| concat(v))),"CONCAT",false);
        binds.insert(Symbol(CONCAT.to_string()),NekoType::func(concat));
        let read_string =
            Function::new_box(Rc::new(Box::new(|v,e| read_string(v,e))),"READSTRING",false);
        binds.insert(Symbol(READSTRING.to_string()),NekoType::func(read_string));
        let atom =
            Function::new_box(Rc::new(Box::new(|v,e| atom(v,e))),"ATOM",false);
        binds.insert(Symbol(ATOM.to_string()),NekoType::func(atom));
        let atomp =
            Function::new_box(Rc::new(Box::new(|v,e| atomp(v,e))),"ATOM?",false);
        binds.insert(Symbol(ATOMP.to_string()),NekoType::func(atomp));
        let deref =
            Function::new_box(Rc::new(Box::new(|v,e| deref(v,e))),"DEREF",false);
        binds.insert(Symbol(DEREF.to_string()),NekoType::func(deref));
        let reset =
            Function::new_box(Rc::new(Box::new(|v,e| reset(v,e))),"RESET",false);
        binds.insert(Symbol(RESET.to_string()),NekoType::func(reset));
        let swap =
            Function::new_box(Rc::new(Box::new(|v,e| swap(v,e))),"SWAP",false);
        binds.insert(Symbol(SWAP.to_string()),NekoType::func(swap));
        let def =
            Function::new_box(Rc::new(
                Box::new(|v,e| def(v,e))),"DEF",true);
        binds.insert(Symbol(DEF.to_string()),
                     NekoType::func(def));
        let let_ =
            Function::new_box(Rc::new(
                Box::new(|v,e| let_(v,e))),"LET",true);
        binds.insert(Symbol(LET.to_string()),
                     NekoType::func(let_));
        let if_ =
            Function::new_box(Rc::new(
                Box::new(|v,e| if_(v,e))),"IF",true);
        binds.insert(Symbol(IF.to_string()),
                     NekoType::func(if_));
        let progn =
            Function::new_box(Rc::new(
                Box::new(|v,e| progn(v,e))),"PROGN",true);
        binds.insert(Symbol(PROGN.to_string()),
                     NekoType::func(progn));
        let quote =
            Function::new_box(Rc::new(
                Box::new(|v,e| quote(v,e))),"QUOTE",true);
        binds.insert(Symbol(QUOTE.to_string()),
                     NekoType::func(quote));
        let quasiquote =
            Function::new_box(Rc::new(
                Box::new(|v,e| quasiquote(v,e))),"QUASIQUOTE",true);
        binds.insert(Symbol(QUASIQUOTE.to_string()),
                     NekoType::func(quasiquote));
        let lambda =
            Function::new_box(Rc::new(
                Box::new(|v,e| lambda(v,e))),"LAMBDA",true);
        binds.insert(Symbol(LAMBDA.to_string()),
                     NekoType::func(lambda));
        let defmarco =
            Function::new_box(Rc::new(
                Box::new(|v,e| defmarco(v,e))),"DEFMARCO",true);
        binds.insert(Symbol(DEFMARCO.to_string()),
                     NekoType::func(defmarco));
        let marcoexpand =
            Function::new_box(Rc::new(
                Box::new(|v,e| marcoexpand(v,e))),"MARCO_EXPAND",true);
        binds.insert(Symbol(MARCOEXPAND.to_string()),
                     NekoType::func(marcoexpand));
        Core{
            binddings:binds,
        }
    }
}

fn add_v(mut v:Vec<NekoType>) -> NekoType {
    if v.len() < 1 {
        NekoType::err("参数不足".to_string())
    } else {
        let mut n1 = v.remove(0);
        for n in v {
            n1=n1+n;
        }
        n1                
    }
}

fn sub_v(mut v:Vec<NekoType>) -> NekoType {
    if v.len() < 1 {
        NekoType::err("参数不足".to_string())
    } else {
        let mut n1 = v.remove(0);
        for n in v {
            n1=n1-n;
        }
        n1                
    }
}

fn mul_v(mut v:Vec<NekoType>) -> NekoType {
    if v.len() < 1 {
        NekoType::err("参数不足".to_string())
    } else {
        let mut n1 = v.remove(0);
        for n in v {
            n1=n1*n;
        }
        n1
    }
}

fn div_v(mut v:Vec<NekoType>) -> NekoType {
    if v.len() < 1 {
        NekoType::err("参数不足".to_string())
    } else {
        let mut n1 = v.remove(0);
        for n in v {
            n1=n1/n;
        }
        n1
    }
}

fn list(v:Vec<NekoType>) -> NekoType {
    return NekoType::list(v);
}

fn listp(mut v:Vec<NekoType>) -> NekoType {
    if v.len() == 1 {
        let l = v.remove(0);
        if let NekoList(_) = *l.get_ref(){
            return NekoType::neko_true();
        } else {
            return NekoType::nil();
        }
    } else {
        return NekoType::err("只支持单一参数判定".to_string());
    }
}

fn emptyp(mut v:Vec<NekoType>) -> NekoType {
    let l = v.remove(0);
    if let NekoList(nv) = l.get_ref().borrow() {
        if nv.is_empty(){
            return NekoType::neko_true();
        } else {
            return NekoType::nil();
        }
    } else {
        return NekoType::err("参数不是列表".to_string());
    }
}

fn count(mut v:Vec<NekoType>) -> NekoType {
    let l = v.remove(0);
    if let NekoList(nv) = l.get_ref().borrow() {
        return NekoType::int_64(nv.len() as i64);
    } else {
        return NekoType::err("参数不是列表".to_string());
    }
}

fn read_string(v:Vec<NekoType>,env:Env) -> NekoType {
    let mut results:Vec<NekoType> = Vec::new();
    for arg in v {
        if let NekoString(s) = arg.get_ref().borrow() {
            results.push(read_str(s.as_str(),env.clone()));
        }
    }
    if results.len() != 1 {
        return NekoType::list(results);
    } else {
        return results.remove(0);
    }
    
}

fn def(args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len()%2 == 0 {
        let mut last_arg = NekoType::nil();
        let mut result_args:Vec<NekoType> = Vec::new();
        for arg in args {
            if let NekoSymbol(a) = last_arg.copy_value() {
                let new_val = eval(arg,env.clone());
                env.set(a,new_val.clone());
                result_args.push(new_val.clone());
                last_arg = NekoType::nil();
            } else {
                last_arg = arg.clone();
            }
        }
        if result_args.len() == 1 {
            return result_args.remove(0);
        } else {
            return NekoType::list(result_args)
        }
    } else {
        return NekoType::err("参数不匹配".to_string());
    }
}

fn defmarco(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() >= 3 {
        let name = args.remove(0);
        if name.is_symbol() {
            let n_func = _lambda(args,env.clone(),true);
            let mut def_args:Vec<NekoType> = Vec::new();
            def_args.push(name);
            def_args.push(n_func);
            return def(def_args,env.clone());
        }
        return NekoType::err("宏的名称必须为Symbol，且宏的参数必须在括号内".to_string());
    } else {
        return NekoType::err("一个完整的宏应至少包含名称、参数和代码".to_string());
    }
}

pub fn marcoexpand(args:Vec<NekoType>,env:Env) -> NekoType {
    let mut results:Vec<NekoType> = Vec::new();
    if args.len() >= 1 {
        for arg in args {
            if let NekoList(ast) = arg.copy_value() {
                let marco = marco_expand(ast,env.clone());
                results.push(NekoType::list(marco));
            }
        }
        if results.len() >= 1 {
            return NekoType::list(results);
        } else if results.len() == 1 {
            return results.remove(0);
        }
    }
    return NekoType::err("参数不能为空".to_string());
}

pub fn marco_expand(ast:Vec<NekoType>,env:Env) -> Vec<NekoType> {
    let mut v_ast = ast;
    loop {
        if is_marco_call(v_ast.clone(),env.clone()) {
            let ns = v_ast.remove(0);
            if ns.is_symbol() {
//                let marco = env.get(symbol);
                if let NekoFn(f) = ns.get_ref().borrow() {
                    if f.is_marco() {
                        let n_ast = f.call(v_ast.clone(),env.clone());
                        if let NekoList(list) = n_ast.get_ref().borrow() {
                            v_ast = list.clone();
                        }
                    }
                }
            }
        } else {
            return v_ast;
        }
    }
}

pub fn is_marco_call(ast:Vec<NekoType>,env:Env) -> bool {
    let nil = NekoType::nil();
    let symb = ast.get(0).unwrap_or(&nil);
    if let NekoSymbol(symbol) = symb.get_ref().borrow() {
        let nf = env.get(symbol);
        if let NekoFn(f) = nf.get_ref().borrow() {
            if f.is_marco() {
                return true;
            }
        }
    }
    return false;
}

#[allow(dead_code)]
fn defun(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() >= 3 {
        let name = args.remove(0);
        if name.is_symbol() {
            let n_func = _lambda(args,env.clone(),false);
            let mut def_args:Vec<NekoType> = Vec::new();
            def_args.push(name);
            def_args.push(n_func);
            return def(def_args,env.clone());
        }
        return NekoType::err("函数的名称必须为Symbol，且宏的参数必须在括号内".to_string());
    } else {
        return NekoType::err("一个完整的函数应至少包含名称、参数和代码".to_string());
    }
}

fn let_(mut args:Vec<NekoType>,env:Env) -> NekoType {
    let bindings = args.remove(0);
    let n_env = Env::new(Some(env.clone()));
    if let NekoList(bs) = bindings.copy_value() {
        let r_env = let_set_bindings(bs,n_env.clone());
        if let Some(e) = r_env {
            let n_env = e;
            let list = NekoType::list(args);
            env.set_tco(n_env.clone());
            return list;
        }
    };
    return NekoType::err("let的绑定不合规".to_string());
}

fn let_set_bindings(mut args:Vec<NekoType>,env:Env) -> Option<Env> {
    let first_arg = args.remove(0);
    match first_arg.copy_value() {
        NekoList(b) => {
            let mut n_env = env;
            let oenv = let_set_bindings(b,n_env);
            if let Some(e) = oenv{
                n_env = e;
                for arg in args {
                    if let NekoList(ab) = arg.copy_value() {
                        let n_oenv =
                            let_set_bindings(ab,n_env);
                        if let Some(ne) = n_oenv {
                            n_env = ne
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                return Some(n_env);
            }
            return None;
        },
        NekoSymbol(s) => {
            if args.len() != 1 {
                return None;
            } else {
                let secend_arg = args.remove(0);
                let n = eval_ast(secend_arg,env.clone());
                env.set(s,n);
                return Some(env)
            }
        },
        _ => {None}
    }
}

fn if_(args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() >= 2 && args.len() <= 3 {
        let arg = args.get(0).
            unwrap_or(&NekoType::nil()).clone();
        let condition = eval(arg.clone(),env.clone());
        if let NekoNil = *condition.get_ref() {
            if args.len() == 3 {
                let arg3 = args.get(2).
                    unwrap_or(&NekoType::nil()).clone();
                env.set_tco(env.clone());
                return arg3.clone();
            } else {
                NekoType::nil()
            }
        } else {
            let arg2 = args.get(1).
                unwrap_or(&NekoType::nil()).clone();
            env.set_tco(env.clone());
            return arg2.clone();
        }
    } else {
        NekoType::err("不合法的if语句".to_string())
    }
}

fn progn(mut args:Vec<NekoType>,env:Env) -> NekoType {
    #[allow(unused_assignments)]
    let mut result = NekoType::nil();
    let arg = args.remove(0);
    result = eval(arg,env.clone());
    if args.len() > 0 {
        let progn_fn = NekoType::symbol("progn".to_string());
        env.set_tco(env.clone());
        args.insert(0,progn_fn);
        return NekoType::list(args);
    } else {
        return result;
    }
}


fn lambda(args:Vec<NekoType>,env:Env) -> NekoType {
    return _lambda(args,env.clone(),false);
}

fn _lambda(args:Vec<NekoType>,_env:Env,marco:bool) -> NekoType {
    if let Some(s) = args.get(0){
        if let NekoList(_) = *s.get_ref() {
            
        } else {
            return NekoType::err("参数必须在列表内".to_string())
        }
    } else {
        return NekoType::err("必须项不存在".to_string())
    }
    let mut prlist:Vec<NekoType> = Vec::new();
    if marco {
        prlist.push(NekoType::symbol("MARCO".to_string()));
    } else {
        prlist.push(NekoType::symbol("FUNCTION".to_string()));
    }    
    prlist.append(&mut args.clone());
    let pralist = NekoType::list(prlist);
    let mut f = Function::
    new_ast(args.clone(),pr_str(pralist.clone()).as_str(),false);
    if marco {
        f = Function::
        new_marco(args.clone(),pr_str(pralist.clone()).as_str());
    }
    return NekoType::func(f);
}

fn atom(args:Vec<NekoType>,_env:Env) -> NekoType {
    if args.len() > 0{
        let mut results:Vec<NekoType> = Vec::new();
        for arg in args {
            results.push(NekoType::atom(arg.clone()));
        }
        if results.len() == 1{
            return results.remove(0);
        } else {
            return NekoType::list(results);
        }
    } else {
        return NekoType::err("参数不能为空".to_string())
    }
    
}

fn atomp(mut args:Vec<NekoType>,_env:Env) -> NekoType {
    if args.len() == 1 {
        let l = args.remove(0);
        if let NekoAtom(_) = *l.get_ref() {
            return NekoType::neko_true();
        } else {
            return NekoType::nil();
        }
    } else {
        return NekoType::err("只支持单一参数判定".to_string());
    }
}

fn deref(args:Vec<NekoType>,_env:Env) -> NekoType {
    if args.len() > 0 {
        let mut results:Vec<NekoType> = Vec::new();
        for arg in args {
            //println!("{}",arg.get_type_str());
            if let NekoAtom(a) = arg.get_ref().borrow() {
                let atom = a.borrow_mut();
                let n = atom.get_neko();
                results.push(n);
            } else {
                return NekoType::err("参数不为原子".to_string())
            }
        }
        if results.len() == 1{
            return results.remove(0);
        } else {
            return NekoType::list(results);
        }
    } else {
        return NekoType::err("参数不能为空".to_string())
    }
}

fn reset(mut args:Vec<NekoType>,_env:Env) -> NekoType {
    if args.len() == 2 {
        let atom_n = args.remove(0);
        let neko = args.remove(0);
        if let NekoAtom(a) = atom_n.get_ref().borrow() {
            let mut atom = a.borrow_mut();
            atom.set_neko(neko.clone());
            return neko;
        } else {
            return NekoType::err("奇参数不是Atom".to_string())
        }
    } else {
        return NekoType::err("参数数量不对".to_string());
    }
}

fn quote(mut args:Vec<NekoType>,_env:Env) -> NekoType {
    if args.len() == 1 {
        return args.remove(0)
    } else if args.len() > 1{
        return NekoType::list(args);
    } else {
        return NekoType::err("参数不能为空".to_string());
    }
}

fn quasiquote(args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() >= 1 {
        let mut results:Vec<NekoType> = Vec::new();
        for arg in args {
            results.push(_quasiquote(arg,env.clone()));
        }
        env.set_tco(env.clone());
        if results.len() == 1 {
            return results.remove(0);
        } else if results.len() > 1{
            return NekoType::list(results);
        } else {
            return NekoType::nil();
        }
    } else {
        return NekoType::err("参数不能为空".to_string());
    }
}

fn _quasiquote(ast:NekoType,env:Env) -> NekoType {
    let mut result:Vec<NekoType> = Vec::new();
    if !ast.is_no_empty_list() {
        result.push(NekoType::symbol("quote".to_string()));
        result.push(ast);
        return NekoType::list(result);
    } else {
        if let NekoList(mut list) = ast.copy_value() {
            let first = list.remove(0);
            if let NekoSymbol(symbol) = first.get_ref().borrow() {
                if symbol.val() == "unquote".to_string() {
                    //ast第一个参数为符号且为unquote的情况
                    if list.len() >= 1{
                        return NekoType::list(list);
                    }
                } else if symbol.val() == "quote".to_string() {
                    return ast;
                } else if symbol.val() == "splice-unquote".to_string() {
                    if list.len() >= 1{
                        list.push(NekoType::err("splice-unquote必须置于列表中。".to_string()));
                        return NekoType::list(list);
                    }
                }
            } else if let NekoList(mut s_list) = first.copy_value() {
                //ast第一个参数为NekoList的情况
                let s_first = s_list.remove(0);
                if let NekoSymbol(symbol) = s_first.get_ref().borrow() {
                    if symbol.val() == "splice-unquote".to_string() {
                        //ast的第一个参数的第一个参数为splice-unquote的情况
                        result.push(NekoType::symbol("concat".to_string()));
                        let mut nr_list:Vec<NekoType> = Vec::new();
                        if s_list.len() >= 1 {
                            let empty:Vec<NekoType> = Vec::new();
                            nr_list.push(NekoType::symbol("concat".to_string()));
                            for ss in s_list {
                                if ss.is_list() {
                                    nr_list.push(ss.clone());
                                } else {
                                    let mut nrn_list:Vec<NekoType> = Vec::new();
                                    nrn_list.push(ss.clone());
                                    nr_list.push(NekoType::list(nrn_list));
                                }
                            }
                            nr_list.push(NekoType::list(empty));
                            result.push(NekoType::list(nr_list));
                            
                        } else {
                            result.push(NekoType::list(s_list));
                        }
                        result.push(_quasiquote(NekoType::list(list),env.clone()));
                        return NekoType::list(result);
                        
                    }
                }
                
            }
            result.push(NekoType::symbol("cons".to_string()));
            result.push(_quasiquote(first,env.clone()));
            result.push(_quasiquote(NekoType::list(list),env.clone()));
            return NekoType::list(result);
        }
    }
    return NekoType::err("quasiquote表达式不正确".to_string());
}

fn swap(mut args:Vec<NekoType>,env:Env) -> NekoType {
    if args.len() >= 2 {
        let atom_n = args.remove(0);
        let func_n = args.remove(0);
        if let NekoAtom(a) = atom_n.get_ref().borrow() {
            let mut atom = a.borrow_mut();
            if let NekoFn(f) = func_n.get_ref().borrow(){
                args.insert(0,atom.get_neko());
                let r = f.call(args,env);
                atom.set_neko(r.clone());
                return r
            }
        }
        return NekoType::err("输入参数类型不正确".to_string());
    } else {
        return NekoType::err("参数数量不对".to_string());
    }

    
}

fn cons(mut args:Vec<NekoType>) -> NekoType {
    //将所有参数按顺序插入最后传入的列表前面
    if args.len() >= 2 {
        let list = args.pop();
        if let Some(neko) = list {
            if let NekoList(nl) = neko.get_ref().borrow() {
                let mut n_list = nl.clone();
                n_list.splice(0..0,args);
                return NekoType::list(n_list);
            } else {
            return NekoType::err("最后一个参数不是列表".to_string());
            }
        }
    }
    return NekoType::err("传入参数数量小于2".to_string());

}

fn concat(args:Vec<NekoType>) -> NekoType {
    if args.len() >= 2 {
        let mut result:Vec<NekoType> = Vec::new();
        for arg in args {
            if let NekoList(c) = arg.get_ref().borrow() {
                result.append(&mut c.clone())
            } else {
                return NekoType::err("传入参数必须为列表".to_string());
            }
        }
        return NekoType::list(result);
    }
    return NekoType::err("传入参数数量小于2".to_string());
}

