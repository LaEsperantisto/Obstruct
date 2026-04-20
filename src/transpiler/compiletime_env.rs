use crate::error;
use crate::expr::Expr;
use crate::span::Span;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::type_env::{nil_type, Type};
use std::collections::HashMap;

pub struct CompileTimeEnv {
    all_types: Vec<Type>,
    scopes: Vec<HashMap<String, (usize, bool, Type)>>, // variable: id, is_mutable, type
    current_scope: usize,

    these: Vec<String>,

    /// HashMap<class name, (Vec<(member name, type)>, next member id)>
    members: HashMap<Type, (Vec<(String, Type)>, usize)>,

    next_var_id: usize,
    next_type_id: usize,

    generic_functions: Vec<(Type, Expr)>,
}

impl CompileTimeEnv {
    pub fn new(ctx: &mut CodeGenContext) -> CompileTimeEnv {
        let mut this = CompileTimeEnv {
            all_types: Vec::new(),
            scopes: vec![HashMap::new()],
            current_scope: 0,

            these: Vec::new(),

            members: HashMap::new(),

            next_var_id: 0,
            next_type_id: 0,

            generic_functions: Vec::new(),
        };

        this.register_type(Type::simple("i32"));
        this.register_type(Type::simple("arr"));
        this.register_type(Type::simple("f64"));
        this.register_type(Type::simple("bool"));
        this.register_type(Type::simple("char"));
        this.register_type(Type::simple("func"));
        this.register_type(Type::simple("strlit"));

        // Declare and register _print: func(i32) -> arr
        this.declare_global_var(
            "_print".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("i32"), nil_type()]),
        );
        this.add_func_type(nil_type(), vec![Type::simple("i32")], ctx, Span::empty());

        // Declare and register _add: func(i32, i32) -> i32
        this.declare_global_var(
            "_add".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("i32"),
                    Type::simple("i32"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("i32"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );

        // Declare and register _less: func(i32, i32) -> i32
        this.declare_global_var(
            "_less".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("i32"),
                    Type::simple("i32"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("i32"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );

        // Declare and register _sub: func(i32, i32) -> i32
        this.declare_global_var(
            "_sub".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("i32"),
                    Type::simple("i32"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("i32"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );

        // Declare and register _pow: func(f64, f64) -> f64
        this.declare_global_var(
            "_pow".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("f64"),
                    Type::simple("f64"),
                    Type::simple("f64"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("f64"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _div: func(f64, f64) -> f64
        this.declare_global_var(
            "_div".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("f64"),
                    Type::simple("f64"),
                    Type::simple("f64"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("f64"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register intput: func() -> i32
        this.declare_global_var(
            "intput".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("i32")]),
        );
        this.add_func_type(Type::simple("i32"), vec![], ctx, Span::empty());

        // Declare and register fput: func() -> f64
        this.declare_global_var(
            "fput".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("f64")]),
        );
        this.add_func_type(Type::simple("f64"), vec![], ctx, Span::empty());

        // Declare and register strput: func() -> strlit
        this.declare_global_var(
            "strput".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("strlit")]),
        );
        this.add_func_type(Type::simple("strlit"), vec![], ctx, Span::empty());

        // Declare and register _equal: func(f64, f64) -> bool
        this.declare_global_var(
            "_equal".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("f64"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _greater: func(f64, f64) -> bool
        this.declare_global_var(
            "_greater".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("f64"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _bang_equal: func(f64, f64) -> bool
        this.declare_global_var(
            "_bang_equal".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("f64"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _greater_equal: func(f64, f64) -> bool
        this.declare_global_var(
            "_greater_equal".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("f64"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _less_equal: func(f64, f64) -> bool
        this.declare_global_var(
            "_less_equal".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("f64"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
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

    // Class handling

    pub fn declare_member(&mut self, name: String, t: Type, class_type: Type) -> usize {
        let class = self.members.get_mut(&class_type).unwrap();
        let id = class.1;

        class.0.push((name, t));

        id
    }

    pub fn c_member_name(&mut self, ty: &Type, name: &str, span: Span) -> String {
        let class = self.members.get(ty);

        match class {
            Some(class) => {
                format!("m_{}", class.1)
            }
            None => {
                error(
                    span,
                    &format!(
                        "Could not find type '{}' when fetching member '{}'.",
                        ty, name
                    ),
                    "fetching mangled member name",
                );
                String::new()
            }
        }
    }

    pub fn get_member_type(&self, ty: &Type, name: &str) -> Option<Type> {
        let class = self.members.get(ty);
        match class {
            Some(class) => {
                let mut output = None;
                for member in class.0.iter() {
                    if member.0 == name {
                        output = Some(member.1.clone());
                        break;
                    }
                }
                output
            }
            None => None,
        }
    }

    pub fn register_class(&mut self, ty: Type) -> usize {
        self.members.insert(ty.clone(), (vec![], 0));
        self.register_type(ty.clone())
    }

    // Variable Handling

    pub fn declare_var(&mut self, name: String, is_mutable: bool, var_type: Type) -> usize {
        let id = self.next_var_id;
        self.next_var_id += 1;

        let scope = self.scopes.last_mut().unwrap();
        scope.insert(name, (id, is_mutable, var_type));

        id
    }

    pub fn declare_global_var(&mut self, name: String, is_mutable: bool, var_type: Type) -> usize {
        let id = self.next_var_id;
        self.next_var_id += 1;

        let scope = self.scopes.first_mut().unwrap();
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

    /// Returns the C variable name for variables.\
    /// Format: v_{id}s_{scope}
    pub fn c_var_name(&self, name: &str, span: Span) -> String {
        let (id, scope) = self.resolve_var(name).unwrap_or_else(|| {
            error(
                span,
                format!("Could not find variable '{}'", name).as_str(),
                "fetching variable name",
            );
            (0, 0)
        });
        format!("v_{}s_{}", id, scope)
    }

    /// Returns the C variable name for a function instance with generics.\
    /// Format: v_{id}s_{scope}C{generic_types}D for generic functions, v_{id}s_{scope}CD for non-generic
    pub fn c_func_instance_name(&mut self, name: &str, generics: &[Type], span: Span) -> String {
        let (id, scope) = self.resolve_var(name).unwrap_or_else(|| {
            error(
                span,
                format!("Could not find variable '{}'", name).as_str(),
                "fetching function name",
            );
            (0, 0)
        });

        let mut result = format!("v_{}s_{}", id, scope);

        if generics.is_empty() {
            // Non-generic function: add CD suffix
            result.push_str("CD");
        } else {
            // Generic function: add C{generics}D
            result.push('C');
            for (i, g) in generics.iter().enumerate() {
                result.push_str(&self.c_type_name(g, span));
                if i != generics.len() - 1 {
                    result.push('_');
                }
            }
            result.push('D');
        }

        result
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

        self.all_types.push(canonical_ty);
        id
    }

    /// Returns the position of a type. Does not check if the type is generic
    fn get_simple_type_id(&self, name: &str) -> usize {
        if let Some(index) = self.all_types.iter().position(|t| t.name() == name) {
            return index;
        }

        0
    }
    /// Returns the ID of a type. If the type is generic, it registers its generic types
    fn get_type_id(&mut self, ty: &Type) -> Option<usize> {
        let canonical = self.canonicalize_type(ty.clone());
        self.all_types.iter().position(|t| t == &canonical)
    }

    /// Registers the generic types in a generic type
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

    /// Internal helper that generates the type name without the CD suffix
    fn c_type_name_raw(&mut self, ty: &Type, span: Span) -> String {
        // Check if this is a function type (has "func" as its name)
        if ty.name() == "func" {
            // For function types, use the mangled format: t_5C{param_types,return_type}D
            let gens = ty.generics();
            let mut name = String::from("t_5C");

            // Add all generics (args + return type) separated by underscore
            for (i, g) in gens.iter().enumerate() {
                name.push_str(&self.c_type_name_raw(g, span));
                if i != gens.len() - 1 {
                    name.push('_');
                }
            }

            // Close with D
            name.push('D');

            name
        } else {
            // For non-function types, use the standard ID-based naming
            let type_id = self.get_type_id(ty).unwrap_or_else(|| {
                error(
                    span,
                    format!("Could not find type '{}'", ty).as_str(),
                    "transpiling",
                );
                0
            });

            format!("t_{}", type_id)
        }
    }

    /// Public API: returns type name with CD suffix for non-generic consistency.\
    /// For function types, the mangling already ends with D, so no additional CD.\
    /// For non-function types, add CD suffix.\
    /// \
    /// Format for simple types: t_{id}\
    /// Format for generic types and for simple functions: t_{id}C{generic types}D\
    /// Format for generic functions: t_{id}C{argument types, return type}DC{generic types}D\
    /// ^^^Note that generic types, argument types and return type are separated by "_".
    pub fn c_type_name(&mut self, ty: &Type, span: Span) -> String {
        let name = self.c_type_name_raw(ty, span);

        // For function types, the raw name already ends with D, so don't add CD
        if ty.name() == "func" {
            name
        } else {
            // For non-function types, add CD suffix for consistency
            let mut result = name;
            result.push_str("CD");
            result
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
        // Format: t_5C{args, ret}DC{gens}D
        let mut gens = arg_types.clone();
        gens.push(ret_type.clone());
        let func_type = Type::with_generics("func", gens);

        if self.get_type_id(&func_type).is_some() {
            return;
        }

        self.register_type(func_type.clone());

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

    pub fn complete(&mut self, _ctx: &mut CodeGenContext) {
        todo!("This function is currently not implemented");
    }
}
