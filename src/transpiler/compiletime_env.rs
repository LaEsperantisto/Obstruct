use crate::error;
use crate::span::Span;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::type_env::{nil_type, Type};
use std::collections::HashMap;

pub struct CompileTimeEnv {
    all_types: Vec<Type>,
    scopes: Vec<HashMap<String, (usize, bool, Type)>>, // variable: id, is_mutable, type
    current_scope: usize,

    these: Vec<String>,

    next_var_id: usize,
    next_type_id: usize,
}

impl CompileTimeEnv {
    pub fn new(ctx: &mut CodeGenContext) -> CompileTimeEnv {
        let mut this = CompileTimeEnv {
            all_types: Vec::new(),
            scopes: vec![HashMap::new()],
            current_scope: 0,

            these: Vec::new(),

            next_var_id: 0,
            next_type_id: 0,
        };

        this.register_type(Type::simple("i32"));
        this.register_type(Type::simple("arr"));
        this.register_type(Type::simple("f64"));
        this.register_type(Type::simple("bool"));
        this.register_type(Type::simple("char"));
        this.register_type(Type::simple("func"));

        // Declare and register _print: func(i32) -> void(arr)
        this.declare_var(
            "_print".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("i32"), nil_type()]),
        );
        this.add_func_type(nil_type(), vec![Type::simple("i32")], ctx, Span::empty());

        // Declare and register _add: func(i32, i32) -> i32
        this.declare_var(
            "_add".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![Type::simple("i32"), Type::simple("i32"), Type::simple("i32")],
            ),
        );
        this.add_func_type(
            Type::simple("i32"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );

        // Declare and register _less: func(i32, i32) -> i32
        this.declare_var(
            "_less".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![Type::simple("i32"), Type::simple("i32"), Type::simple("i32")],
            ),
        );
        this.add_func_type(
            Type::simple("i32"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );

        // Declare and register _sub: func(i32, i32) -> i32
        this.declare_var(
            "_sub".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![Type::simple("i32"), Type::simple("i32"), Type::simple("i32")],
            ),
        );
        this.add_func_type(
            Type::simple("i32"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );

        this
    }

    // Scope Management

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.current_scope += 1;
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
        self.current_scope -= 1;
    }

    // Variable Handling

    pub fn declare_var(&mut self, name: String, is_mutable: bool, var_type: Type) -> usize {
        let id = self.next_var_id;
        self.next_var_id += 1;

        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name, (id, is_mutable, var_type));

        id
    }

    fn resolve_var(&self, name: &str) -> Option<(usize, usize)> {
        for (idx, scope) in self.scopes.iter().enumerate().rev() {
            if let Some(id) = scope.get(name) {
                return Some((id.0, idx));
            }
        }
        None
    }

    pub fn get_var(&self, name: &str) -> Option<(bool, Type)> {
        for scope in self.scopes.iter().rev() {
            if let Some((_id, is_mutable, var_type)) = scope.get(name) {
                return Some((*is_mutable, var_type.clone()));
            }
        }
        None
    }

    pub fn var_exists(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if let Some(_) = scope.get(name) {
                return true;
            }
        }
        false
    }

    pub fn c_var_name(&self, name: &str, span: Span) -> String {
        let (id, scope) = self.resolve_var(name).unwrap_or_else(|| {
            error(span, format!("Could not find variable '{}'", name).as_str());
            (0, 0)
        });
        format!("v_{}s_{}", id, scope,)
    }

    // Type Handling

    pub fn register_type(&mut self, ty: Type) -> usize {
        if let Type::Conceptual(name) = &ty {
            panic!("Tried to register conceptual type: {}", name);
        }

        let canonical_ty = self.canonicalize_type(ty);

        if let Some(index) = self.all_types.iter().position(|t| t == &canonical_ty) {
            return index;
        }

        let id = self.next_type_id;
        self.next_type_id += 1;

        println!("register_type: {}, {}", canonical_ty, id);

        self.all_types.push(canonical_ty);
        id
    }

    fn get_simple_type_id(&self, name: &str) -> usize {
        if let Some(index) = self.all_types.iter().position(|t| t.name() == name) {
            return index;
        }

        0
    }

    fn get_type_id(&mut self, ty: &Type) -> Option<usize> {
        let canonical = self.canonicalize_type(ty.clone());
        self.all_types.iter().position(|t| t == &canonical)
    }

    fn canonicalize_type(&mut self, ty: Type) -> Type {
        let canonical_generics: Vec<Type> = ty
            .generics()
            .iter()
            .map(|g| {
                let id = self.register_type(g.clone());
                self.all_types[id].clone()
            })
            .collect();

        if canonical_generics.is_empty() {
            ty
        } else {
            Type::with_generics(ty.name(), canonical_generics)
        }
    }

    pub fn c_type_name(&mut self, ty: &Type, span: Span) -> String {
        // Check if this is a function type (has "func" as its name)
        if ty.name() == "func" {
            // For function types, use the mangled format: t_5C{args,ret}D
            // t_5 is the func marker, C...D wraps all generics (args + return)
            let gens = ty.generics();
            if gens.is_empty() {
                return "t_5".to_string();
            }

            let mut name = String::from("t_5C");

            // Add all generics (args + return type) separated by underscore
            for (i, g) in gens.iter().enumerate() {
                name.push_str(&self.c_type_name(g, span));
                if i != gens.len() - 1 {
                    name.push('_');
                }
            }

            // Close generics with D at the end
            name.push('D');

            name
        } else {
            // For non-function types, use the standard ID-based naming
            let type_id = self.get_type_id(ty).unwrap_or_else(|| {
                error(span, format!("Could not find type '{}'", ty).as_str());
                0
            });

            let mut name = format!("t_{}", type_id);

            let gens = ty.generics();

            if !gens.is_empty() {
                name.push('C');

                for (i, g) in gens.iter().enumerate() {
                    name.push_str(&self.c_type_name(g, span));

                    if i != gens.len() - 1 {
                        name.push('_');
                    }
                }

                name.push('D');
            }

            name
        }
    }

    pub fn add_func_type(
        &mut self,
        ret_type: Type,
        arg_types: Vec<Type>,
        ctx: &mut CodeGenContext,
        span: Span,
    ) {
        // Generate the function type name using the mangling scheme
        // Format: t_4C{args}D{ret}
        let mut gens = arg_types.clone();
        gens.push(ret_type.clone());
        let func_type = Type::with_generics("func", gens);

        let func_type_name = self.c_type_name(&func_type, span);

        // Generate the typedef for the function type
        ctx.types.push_str("typedef ");
        ctx.types.push_str(&self.c_type_name(&ret_type, span));
        ctx.types.push_str(&format!("(*{})", func_type_name));
        ctx.types.push('(');
        for ty in arg_types.iter() {
            ctx.types.push_str(&self.c_type_name(&ty, span));
            ctx.types.push(',');
        }
        if arg_types.len() >= 1 {
            ctx.types.pop();
        }
        ctx.types.push_str(");\n");
    }

    pub fn push_this(&mut self, this: &str) {
        self.these.push(this.to_string());
    }
    pub fn pop_this(&mut self) {
        self.these.pop();
    }
    pub fn this(&self) -> &str {
        &self.these.last().unwrap()
    }

    pub fn del_var(&mut self, name: &str) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(_id) = scope.get(name) {
                scope.remove(name);
            }
        }
    }
}
