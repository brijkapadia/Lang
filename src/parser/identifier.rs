use std::collections::{HashSet,HashMap};
use std::rc::Rc;

struct Variables(HashSet<Rc<String>>);

struct Functions(HashSet<Rc<String>>);

struct ScopeIdentifiers{
    variables: Variables,
    functions: Functions
}


enum Identfiers{
    Variable{
        name: Rc<String>
    },
    Function{
        name: String
    }
}

impl Identfiers{
    fn new_var(name: Rc<String>) -> Self{
        return Self::Variable{name}
    }
    
    fn new_fn(name: String) -> Self{
        return Self::Function{name}
    }
}
