use crate::error;
use crate::expr::Expr;
use crate::span::Span;
use crate::transpiler::code_gen_context::CodeGenContext;
use crate::type_env::{nil_type, Type};
use std::collections::{HashMap, HashSet};

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

    /// Set of ref<T> type names that have already been emitted as typedefs
    ref_typedefs_emitted: HashSet<String>,
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

            ref_typedefs_emitted: HashSet::new(),
        };

        this.register_type(Type::simple("i32"));
        this.register_type(Type::simple("arr"));
        this.register_type(Type::simple("f64"));
        this.register_type(Type::simple("bool"));
        this.register_type(Type::simple("char"));
        this.register_type(Type::simple("func"));
        this.register_type(Type::simple("strlit"));
        this.register_type(Type::simple("ref"));

        // Declare and register _print: func(i32) -> arr
        this.declare_global_var(
            "_print".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("i32"), nil_type()]),
        );
        this.add_func_type(nil_type(), vec![Type::simple("i32")], ctx, Span::empty());

        // Declare and register _add: func(type, type, return)
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
        this.add_func_type(
            Type::simple("f64"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("strlit"),
            vec![Type::simple("strlit"), Type::simple("strlit")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("strlit"),
            vec![Type::simple("strlit"), Type::simple("char")],
            ctx,
            Span::empty(),
        );

        // Declare and register _less: func(arg1, arg2, return)
        this.declare_global_var(
            "_less".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _sub: func(arg1, arg2, return)
        this.declare_global_var(
            "_sub".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("f64"),
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
        this.add_func_type(
            Type::simple("f64"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _mult: func(arg1, arg2, return)
        this.declare_global_var(
            "_mult".to_string(),
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
        this.add_func_type(
            Type::simple("f64"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _pow: func(arg1, arg2, return)
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

        // Declare and register _div: func(arg1, arg2, return)
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

        // Declare and register _equal: func(arg1, arg2, return)
        this.declare_global_var(
            "_equal".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("strlit"), Type::simple("strlit")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("char"), Type::simple("char")],
            ctx,
            Span::empty(),
        );

        // Declare and register _greater: func(arg1, arg2, return)
        this.declare_global_var(
            "_greater".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _bang_equal: func(arg1, arg2, return)
        this.declare_global_var(
            "_bang_equal".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("strlit"), Type::simple("strlit")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("char"), Type::simple("char")],
            ctx,
            Span::empty(),
        );

        // Declare and register _greater_equal: func(arg1, arg2, return)
        this.declare_global_var(
            "_greater_equal".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        // Declare and register _less_equal: func(arg1, arg2, return)
        this.declare_global_var(
            "_less_equal".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("i32"),
                    Type::simple("f64"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("i32"), Type::simple("i32")],
            ctx,
            Span::empty(),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("f64"), Type::simple("f64")],
            ctx,
            Span::empty(),
        );

        this.declare_global_var(
            "or".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("bool"),
                    Type::simple("bool"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("bool"), Type::simple("bool")],
            ctx,
            Span::empty(),
        );

        this.declare_global_var(
            "and".to_string(),
            false,
            Type::with_generics(
                "func",
                vec![
                    Type::simple("bool"),
                    Type::simple("bool"),
                    Type::simple("bool"),
                ],
            ),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("bool"), Type::simple("bool")],
            ctx,
            Span::empty(),
	);

        this.declare_global_var(
            "not".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("bool"), Type::simple("bool")]),
        );
        this.add_func_type(
            Type::simple("bool"),
            vec![Type::simple("bool")],
            ctx,
            Span::empty(),
        );

        this.declare_global_var(
            "terminal_width".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("i32")]),
        );
        this.add_func_type(Type::simple("i32"), vec![], ctx, Span::empty());

        this.declare_global_var(
            "terminal_height".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("i32")]),
        );
        this.add_func_type(Type::simple("i32"), vec![], ctx, Span::empty());
	
        this.declare_global_var(
            "get_pressed_key".to_string(),
            false,
            Type::with_generics("func", vec![Type::simple("char")]),
        );
        this.add_func_type(Type::simple("char"), vec![], ctx, Span::empty());

	this.declare_global_var(
	    "sleep".to_string(),
	    false,
	    Type::with_generics("func", vec![Type::simple("i32"), nil_type()]),
	);
	this.add_func_type(nil_type(), vec![], ctx, Span::empty());

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
        self.declare_var_inner(name, is_mutable, var_type)
    }

    fn declare_var_inner(&mut self, name: String, is_mutable: bool, var_type: Type) -> usize {
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
            // Match inline convention: C opens the mangling bracket,
            // each type uses its raw name (e.g. t_0 for i32), types
            // separated by C, ending with DD (= last type's CD + closing D).
            // e.g. v_1s_0Ct_0Ct_0CDD for _add(i32, i32)
            result.push('C');
            for g in generics.iter() {
                result.push_str(&self.c_type_name_raw(g, span));
                result.push('C');
            }
            result.push_str("DD");
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

    /// Returns the ID of a type. If the type is generic, it registers its generic types
    fn get_type_id(&mut self, ty: &Type) -> Option<usize> {
        let canonical = self.canonicalize_type(ty.clone());
        if !ty.is_conceptual() {
            self.all_types.iter().position(|t| t == &canonical)
        } else {
            self.all_types
                .iter()
                .position(|t| t.name() == ty.name() && !t.has_generics())
        }
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
            // For non-function types, use the standard ID-based naming.
            // For generic types, find the base type's ID (without generics)
            // instead of the position of the full canonicalized type in all_types.
            // e.g. ref<i32> should use the base ref type's ID (t_7), not the
            // position of the instantiated type (which could be t_16).
            let base_id = self
                .all_types
                .iter()
                .position(|t| t.name() == ty.name() && !t.has_generics());
            let type_id = if let Some(id) = base_id {
                id
            } else {
                self.get_type_id(ty).unwrap_or_else(|| {
                    error(
                        span,
                        format!("Could not find type '{}'", ty).as_str(),
                        "transpiling",
                    );
                    0
                })
            };

            let mut name = format!("t_{}", type_id);

            let gens = ty.generics();
            if !gens.is_empty() {
                name.push('C');

                for (i, g) in gens.iter().enumerate() {
                    name.push_str(&self.c_type_name_raw(g, span));
                    if i != gens.len() - 1 {
                        name.push('_');
                    }
                }

                name.push('D');
            }

            name
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
    pub fn c_type_name(&mut self, ty: &Type, ctx: &mut CodeGenContext, span: Span) -> String {
        let name = self.c_type_name_raw(ty, span);

        // For function types, the raw name already ends with D, so don't add CD
        if ty.name() == "func" {
            name
        } else if ty.generics().is_empty() {
            let mut result = name;
            result.push_str("CD");
            result
        } else if ty.name() == "ref" {
            // Ensure a typedef like: typedef t_15CD* t_7Ct_15CD; exists for ref<SomeType>
            let inner_type = ty.generics()[0].clone();
            let inner_raw = self.c_type_name_raw(&inner_type, span);
            let inner_c_type = if inner_type.name() == "func" {
                inner_raw
            } else {
                inner_raw + "CD"
            };
            let typedef_name = name.clone();
            let typedef_key = format!("ref_{}", inner_c_type);
            if !self.ref_typedefs_emitted.contains(&typedef_key) {
                self.ref_typedefs_emitted.insert(typedef_key);
                ctx.types
                    .push_str(&format!("typedef {}* {};\n", inner_c_type, typedef_name));
            }
            name
        } else {
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
        // Format: t_5C{args, ret}DC{gens}D
        let mut gens = arg_types.clone();
        gens.push(ret_type.clone());
        let func_type = Type::with_generics("func", gens);

        if self.get_type_id(&func_type).is_some() {
            return;
        }

        self.register_type(func_type.clone());

        let func_type_name = self.c_type_name(&func_type, ctx, span);
        let ret_type_name = self.c_type_name(&ret_type, ctx, span);

        // Generate the typedef for the function type
        let mut buf = String::from("typedef ");
        buf.push_str(&ret_type_name);
        buf.push_str(&format!("(*{})", func_type_name));
        buf.push('(');
        for ty in arg_types.iter() {
            buf.push_str(&self.c_type_name(ty, ctx, span));
            buf.push(',');
        }
        if arg_types.len() >= 1 {
            buf.pop();
        }
        buf.push_str(");\n");
        ctx.types.push_str(&buf);
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

    /// Returns true if the type is a registered class (struct type)
    pub fn is_class(&self, ty: &Type) -> bool {
        self.members.contains_key(ty)
    }

    /// Returns the C type name for a parameter, using pointer for structs
    pub fn c_param_type(&mut self, ty: &Type, ctx: &mut CodeGenContext, span: Span) -> String {
        if self.is_class(ty) {
            format!("{}*", self.c_type_name(ty, ctx, span))
        } else {
            self.c_type_name(ty, ctx, span)
        }
    }

    pub fn complete(&mut self, _ctx: &mut CodeGenContext) {
        todo!("This function is currently not implemented");
    }
}
