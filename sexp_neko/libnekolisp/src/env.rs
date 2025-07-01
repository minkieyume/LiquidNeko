use crate::symbols::SymbolRef;
use alloc::rc::Rc;
use core::cell::RefCell;
use alloc::string::ToString;
use hashbrown::HashMap;
use core::hash::{Hash, Hasher};
use crate::nekocore::Core;
use crate::types::NekoType;
use crate::types::Symbol;

pub const NEKOS:&str = "*NEKOS*";
pub const CONTEXT:&str = "*CONTEXT*";

#[derive(Clone)]
pub struct EnvType {
    pub outer: Option<Env>,
    pub symbols:SymbolRef,
    pub data: HashMap<Symbol, NekoType>,
    pub tco:Option<Env>
}

#[derive(Clone)]
pub struct Env(Rc<RefCell<EnvType>>);

impl Env {
    pub fn new(outer:Option<Env>) -> Env {
        Env(Rc::new(RefCell::new(EnvType {
            outer: outer.clone().map(|e| e.clone()),
            symbols:outer.clone().map_or(SymbolRef::new(),
                                 |e| e.get_symbol()),
            data: HashMap::new(),
            tco:None,
        })))
    }

    pub fn get_symbol(&self) -> SymbolRef {
        return self.0.borrow().symbols.clone();
    }

    pub fn with_bindings(outer:Option<Env>,bindings:HashMap<Symbol, NekoType>) -> Env {
        let env = Env(Rc::new(RefCell::new(EnvType {
            outer: outer.clone().map(|e| e.clone()),
            symbols:outer.clone().map_or(SymbolRef::new(),
                                 |e| e.get_symbol()),
            data: HashMap::new(),
            tco:None,
        })));
        let keys = bindings.keys();
        for bind in keys {
            if let Some(val) = bindings.get(bind) {
                env.set(bind.clone(),val.clone());
            }
            
        }
        env
    }

    pub fn default() -> Env {
        let core = Core::default();
        Env::with_bindings(None,core.binddings)
    }

    pub fn set_tco(&self,tco_env:Env) {
        self.0.borrow_mut().tco = Some(tco_env.clone());
    }

    pub fn clear_tco(&self) {
        self.0.borrow_mut().tco = None;
    }

    pub fn get_tco(&self) -> Option<Env> {
        self.0.borrow().tco.clone()
    }

    pub fn set(&self,key:Symbol, val: NekoType) {
        self.0.borrow_mut().data.insert(key, val);
    }

    pub fn set_by_str(&self, key: &str, val: NekoType) {
        let skey = Symbol(key.to_string());
        self.0.borrow_mut().data.insert(skey, val);
    }

    pub fn find(&self,key:&Symbol) -> Option<Env> {
        let renv = self.0.borrow();
        let val = renv.data.get(key);
        match val {
            Some(_) => Some(self.clone()),
            None => {
                if let Some(o) = &renv.outer {
                    return Some(o.clone());
                } else {
                    return None
                }
            },
        }
    }

    pub fn get(&self,key:&Symbol) -> NekoType {
        let o_env = self.find(key);
        match o_env {
            Some(env) => {
                let renv = env.0.borrow();
                let val = renv.data.get(key);
                match val {
                    Some(neko) => neko.clone(),
                    None => NekoType::err("环境中不存在该键".to_string()),
                }
            },
            None => NekoType::err("环境中不存在该键".to_string()),
        }
    }

    pub fn get_by_str(&self,key:&str) -> NekoType {
        self.get(&Symbol(key.to_string()))
    }
}

impl Hash for EnvType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.outer.hash(state);
        self.symbols.hash(state);
        for k in self.data.keys() {
            k.hash(state);
        }
        for v in self.data.values() {
            v.hash(state);
        }
        self.tco.hash(state);
    }
}

impl Hash for Env {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state)
    }
}
