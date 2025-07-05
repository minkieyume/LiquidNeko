use alloc::{vec::Vec, string::String};
use core::fmt::Write;
use core::borrow::Borrow;
use alloc::string::ToString;
use crate::types::NekoType;
use crate::types::NekoValue::*;

pub fn pr_str(neko:NekoType) -> String {
    let mut output = String::new();
    let value = neko.get_ref();
    match value.borrow() {
        Nekoi64(n) => {output = n.to_string();},
        Nekof64(f) => {output = f.to_string();},
        NekoSymbol(s) => {output = s.val();},
        NekoString(s) => {output = s.clone();},
        NekoNil => {output = "nil".to_string();},
        NekoTrue => {output = "T".to_string();},
        NekoKeyword(k) => {output = k.clone()},
        NekoAtom(a) => {
            let atom = a.borrow_mut();
            let neko = atom.get_neko();
            let result = write!(&mut output,"ATOM({})", pr_str(neko));
            match result {
                Ok(_) => {},
                Err(e) => {output = e.to_string()}
            };
        },
        NekoErr(e) => {
            let result = write!(&mut output,"Error: {}", e.clone());
            match result {
                Ok(_) => {},
                Err(e) => {output = e.to_string()}
            }
        },
        NekoFn(f) => {
            let result = write!(&mut output,"#<{}>", f.print());
            match result {
                Ok(_) => {},
                Err(e) => {output = e.to_string()}
            }
        }
        NekoList(v) => {
            let mut sv:Vec<String> = Vec::new();
            for n in v {
                sv.push(pr_str(n.clone()));
            }
            let result = write!(&mut output,"({})", sv.join(" "));
            match result {
                Ok(_) => {},
                Err(e) => {output = e.to_string()}
            }
            
        },
        _ => {output = "未实现".to_string();}
    }
    return output;
}
