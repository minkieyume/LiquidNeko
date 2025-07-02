use alloc::vec::Vec;
use core::borrow::Borrow;
use alloc::string::ToString;
use crate::types::NekoType;
use crate::types::NekoValue::*;
use crate::env::NEKOS;
use crate::env::Env;
use crate::nekocore::marco_expand;
use crate::types::Symbol;

pub fn eval_ast(n:NekoType,env:Env) -> NekoType {
    //对单个参数进行求值
    match n.get_ref().borrow() {
        NekoSymbol(s) => {env.get(&s)},
        NekoList(v) => {
            let mut nv:Vec<NekoType> = Vec::new();
            for e in v {
                let ne = eval(e.clone(),env.clone());
                nv.push(ne);
            }
            NekoType::list(nv)
        },
        _ => n,
    }
}

pub fn eval(n:NekoType,env:Env) -> NekoType {
    //对参数进行判定并决定是否执行或应用
    #[allow(unused_assignments)]
    let mut result = NekoType::nil();
    let mut v_n = n.clone();
    let mut v_env = env.clone();
    loop {
        match v_n.get_ref().borrow() {
            NekoList(v) => {
                if !v.is_empty() {
                    let marco_result = marco_expand(v.clone(),env.clone());
                    let nlist = NekoType::list(marco_result);
                    result = apply(nlist,v_env.clone());
                } else {
                    result = v_n;
                }
            },
            _ => {result = eval_ast(v_n,v_env.clone());}
        }
        if let Some(tco) = v_env.get_tco() {
            v_env = tco.clone();
            v_n = result.clone();
            v_env.clear_tco();
        } else {
            return result;
        }
    }
}

pub fn apply(list:NekoType,env:Env) -> NekoType {
    //对列表执行求值与应用操作    
    let mut v_list = list.clone();
    loop{
        if let NekoList(v) = v_list.get_ref().borrow() {
            let mut args:Vec<NekoType> = v.clone();
            let mut first_arg = args.remove(0);
            let namespace = eval_namespace(first_arg.clone(),env.clone());
            if let Some(n) = namespace {
                first_arg = n.clone()
            };
            match first_arg.get_ref().borrow() {
                NekoFn(f) => {
                    if !f.is_special(){
                        return f.call(args,env.clone());
                    } else {
                        return NekoType::err("不可在此调用特殊形式".to_string());
                    }
                },
                NekoSymbol(_) => {
                    let nfn
                        = eval_ast(first_arg.clone(),env.clone());
                    if let NekoFn(f) = nfn.copy_value() {
                        if f.is_special() {
                            return f.call(args,env.clone());
                        }
                    }
                    v_list = eval_ast(v_list.clone(),env.clone());
                },
                _ => {
                    //第一个值为其它值，则对每个值进行eval并返回
                    let mut result:Vec<NekoType> = Vec::new();
                    for n in v {
                        result.push(eval(n.clone(),env.clone()));
                    }
                    return NekoType::list(result);
                },
            };
        } else {
            return list;
        }
    }
}

pub fn eval_namespace(arg:NekoType,env:Env) -> Option<NekoType> {
    if let NekoSymbol(symb) = arg.copy_value() {
        let mut n_env = env.clone();        
        let nekos = n_env.get_by_str(NEKOS);
        let mut symbs = symb.split_namespace(n_env.clone());
        let end = symbs.pop().unwrap_or("");

        for sym in symbs {
            let nya = NekoType::symbol(sym.to_string());
            if let NekoDict(dict) = nekos.copy_value() {
                //验证并获取Neko
                let o_neko = dict.get(&nya);
                let mut _neko = NekoType::nil();
                match o_neko {
                    Some(n) => {_neko = n.clone()},
                    None => {
                        //事后可能需要加入对struct类的支持
                        return Some(NekoType::err("当前环境中找不到名为该键的Neko".to_string()));
                    },
                };

                if let NekoNeko(e) = _neko.copy_value() {
                    //将Neko设为当前环境
                    n_env = e.clone()
                } else {
                    return None;
                }
            }
        }
        let end_neko = Symbol(end.to_string());
        let neko = n_env.get(&end_neko);
        return  Some(neko)
    } else {
        return None
    }
}
