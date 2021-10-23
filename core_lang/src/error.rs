use crate::parser::Rule;
use crate::span::Span;
use crate::type_engine::{IntegerBits, TypeInfo};
use inflector::cases::classcase::to_class_case;
use inflector::cases::snakecase::to_snake_case;
use thiserror::Error;

macro_rules! check {
    ($fn_expr: expr, $error_recovery: expr, $warnings: ident, $errors: ident) => {{
        let mut res = $fn_expr;
        $warnings.append(&mut res.warnings);
        $errors.append(&mut res.errors);
        match res.value {
            None => $error_recovery,
            Some(value) => value,
        }
    }};
}

macro_rules! assert_or_warn {
    ($bool_expr: expr, $warnings: ident, $span: expr, $warning: expr) => {
        if !$bool_expr {
            use crate::error::CompileWarning;
            $warnings.push(CompileWarning {
                warning_content: $warning,
                span: $span,
            });
        }
    };
}

/// Denotes a non-recoverable state
pub(crate) fn err<T>(
    warnings: Vec<CompileWarning>,
    errors: Vec<CompileError>,
) -> CompileResult<T> {
    CompileResult {
        value: None,
        warnings,
        errors,
    }
}

/// Denotes a recovered or non-error state
pub(crate) fn ok< T>(
    value: T,
    warnings: Vec<CompileWarning>,
    errors: Vec<CompileError>,
) -> CompileResult<T> {
    CompileResult {
        value: Some(value),
        warnings,
        errors,
    }
}

#[derive(Debug, Clone)]
pub struct CompileResult< T> {
    pub value: Option<T>,
    pub warnings: Vec<CompileWarning>,
    pub errors: Vec<CompileError>,
}

impl<T> CompileResult< T> {
    pub fn ok(
        mut self,
        warnings: &mut Vec<CompileWarning>,
        errors: &mut Vec<CompileError>,
    ) -> Option<T> {
        warnings.append(&mut self.warnings);
        errors.append(&mut self.errors);
        self.value
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> CompileResult< U> {
        match self.value {
            None => err(self.warnings, self.errors),
            Some(value) => ok(f(value), self.warnings, self.errors),
        }
    }

    pub fn unwrap(
        self,
        warnings: &mut Vec<CompileWarning>,
        errors: &mut Vec<CompileError>,
    ) -> T {
        let panic_msg = format!("Unwrapped an err {:?}", self.errors);
        self.unwrap_or_else(warnings, errors, || panic!("{}", panic_msg))
    }

    pub fn unwrap_or_else<F: FnOnce() -> T>(
        self,
        warnings: &mut Vec<CompileWarning>,
        errors: &mut Vec<CompileError>,
        or_else: F,
    ) -> T {
        self.ok(warnings, errors).unwrap_or_else(or_else)
    }
}

#[derive(Debug, Clone)]
pub struct CompileWarning {
    pub span: Span,
    pub warning_content: Warning,
}

pub struct LineCol {
    pub line: usize,
    pub col: usize,
}

impl From<(usize, usize)> for LineCol {
    fn from(o: (usize, usize)) -> Self {
        LineCol {
            line: o.0,
            col: o.1,
        }
    }
}

impl CompileWarning {
    pub fn to_friendly_warning_string(&self) -> String {
        self.warning_content.to_string()
    }

    pub fn span(&self) -> (usize, usize) {
        (self.span.start(), self.span.end())
    }

    pub fn path(&self) -> String {
        self.span.path()
    }

    /// Returns the line and column start and end
    pub fn line_col(&self) -> (LineCol, LineCol) {
        (
            self.span.start_pos().line_col().into(),
            self.span.end_pos().line_col().into(),
        )
    }
}

#[derive(Debug, Clone)]
pub enum Warning {
    NonClassCaseStructName {
        struct_name: &'static str,
    },
    NonClassCaseTraitName {
        name: &'static str,
    },
    NonClassCaseEnumName {
        enum_name: &'static str,
    },
    NonClassCaseEnumVariantName {
        variant_name: &'static str,
    },
    NonSnakeCaseStructFieldName {
        field_name: &'static str,
    },
    NonSnakeCaseFunctionName {
        name: &'static str,
    },
    LossOfPrecision {
        initial_type: IntegerBits,
        cast_to: IntegerBits,
    },
    UnusedReturnValue {
        r#type: TypeInfo,
    },
    SimilarMethodFound {
        lib: &'static str,
        module: &'static str,
        name: &'static str,
    },
    OverridesOtherSymbol {
        name: String,
    },
    OverridingTraitImplementation,
    DeadDeclaration,
    DeadFunctionDeclaration,
    DeadStructDeclaration,
    DeadTrait,
    UnreachableCode,
    DeadEnumVariant {
        variant_name: String,
    },
    DeadMethod,
    StructFieldNeverRead,
    ShadowingReservedRegister {
        reg_name: &'static str,
    },
}

impl Warning {
    fn to_string(&self) -> String {
        use Warning::*;
        match self {
            NonClassCaseStructName { struct_name } => format!(
                "Struct name \"{}\" is not idiomatic. Structs should have a ClassCase name, like \
                 \"{}\".",
                struct_name,
                to_class_case(struct_name)
            ),
            NonClassCaseTraitName { name } => format!(
                "Trait name \"{}\" is not idiomatic. Traits should have a ClassCase name, like \
                 \"{}\".",
                name,
                to_class_case(name)
            ),
            NonClassCaseEnumName { enum_name } => format!(
                "Enum \"{}\"'s capitalization is not idiomatic. Enums should have a ClassCase \
                 name, like \"{}\".",
                enum_name,
                to_class_case(enum_name)
            ),
            NonSnakeCaseStructFieldName { field_name } => format!(
                "Struct field name \"{}\" is not idiomatic. Struct field names should have a \
                 snake_case name, like \"{}\".",
                field_name,
                to_snake_case(field_name)
            ),
            NonClassCaseEnumVariantName { variant_name } => format!(
                "Enum variant name \"{}\" is not idiomatic. Enum variant names should be \
                 ClassCase, like \"{}\".",
                variant_name,
                to_class_case(variant_name)
            ),
            NonSnakeCaseFunctionName { name } => format!(
                "Function name \"{}\" is not idiomatic. Function names should be snake_case, like \
                 \"{}\".",
                name,
                to_snake_case(name)
            ),
            LossOfPrecision {
                initial_type,
                cast_to,
            } => format!(
                "This cast, from integer type of width {} to integer type of width {}, will lose precision.",
                initial_type.friendly_str(),
                cast_to.friendly_str()
            ),
            UnusedReturnValue { r#type } => format!(
                "This returns a value of type {}, which is not assigned to anything and is \
                 ignored.",
                r#type.friendly_type_str()
            ),
            SimilarMethodFound { lib, module, name } => format!(
                "A method with the same name was found for type {} in dependency \"{}::{}\". \
                 Traits must be in scope in order to access their methods. ",
                name, lib, module
            ),
            OverridesOtherSymbol { name } => format!(
                "This import would override another symbol with the same name \"{}\" in this \
                 namespace.",
                name
            ),
            OverridingTraitImplementation => format!(
                "This trait implementation overrides another one that was previously defined."
            ),
            DeadDeclaration => "This declaration is never used.".into(),
            DeadStructDeclaration => "This struct is never instantiated.".into(),
            DeadFunctionDeclaration => "This function is never called.".into(),
            UnreachableCode => "This code is unreachable.".into(),
            DeadEnumVariant { variant_name } => {
                format!("Enum variant {} is never constructed.", variant_name)
            }
            DeadTrait => "This trait is never implemented.".into(),
            DeadMethod => "This method is never called.".into(),
            StructFieldNeverRead => "This struct field is never accessed.".into(),
            ShadowingReservedRegister { reg_name } => format!(
                "This register declaration shadows the reserved register, \"{}\".",
                reg_name
            ),
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum CompileError {
    #[error("Variable \"{var_name}\" does not exist in this scope.")]
    UnknownVariable { var_name: String, span: Span },
    #[error("Variable \"{var_name}\" does not exist in this scope.")]
    UnknownVariablePath { var_name: &'static str, span: Span },
    #[error("Function \"{name}\" does not exist in this scope.")]
    UnknownFunction { name: &'static str, span: Span },
    #[error("Identifier \"{name}\" was used as a variable, but it is actually a {what_it_is}.")]
    NotAVariable {
        name: String,
        span: Span,
        what_it_is: &'static str,
    },
    #[error(
        "Identifier \"{name}\" was called as if it was a function, but it is actually a \
         {what_it_is}."
    )]
    NotAFunction {
        name: String,
        span: Span,
        what_it_is: &'static str,
    },
    #[error("Unimplemented feature: {0}")]
    Unimplemented(&'static str, Span),
    #[error("{0}")]
    TypeError(TypeError),
    #[error("Error parsing input: expected {err:?}")]
    ParseFailure {
        span: Span,
        err: pest::error::Error<Rule>,
    },
    #[error(
        "Invalid top-level item: {0:?}. A program should consist of a contract, script, or \
         predicate at the top level."
    )]
    InvalidTopLevelItem(Rule, Span),
    #[error(
        "Internal compiler error: {0}\nPlease file an issue on the repository and include the \
         code that triggered this error."
    )]
    Internal(&'static str, Span),
    #[error("Unimplemented feature: {0:?}")]
    UnimplementedRule(Rule, Span),
    #[error(
        "Byte literal had length of {byte_length}. Byte literals must be either one byte long (8 \
         binary digits or 2 hex digits) or 32 bytes long (256 binary digits or 64 hex digits)"
    )]
    InvalidByteLiteralLength { byte_length: usize, span: Span },
    #[error("Expected an expression to follow operator \"{op}\"")]
    ExpectedExprAfterOp { op: &'static str, span: Span },
    #[error("Expected an operator, but \"{op}\" is not a recognized operator. ")]
    ExpectedOp { op: &'static str, span: Span },
    #[error(
        "Where clause was specified but there are no generic type parameters. Where clauses can \
         only be applied to generic type parameters."
    )]
    UnexpectedWhereClause(Span),
    #[error(
        "Specified generic type in where clause \"{type_name}\" not found in generic type \
         arguments of function."
    )]
    UndeclaredGenericTypeInWhereClause {
        type_name: &'static str,
        span: Span,
    },
    #[error(
        "Program contains multiple contracts. A valid program should only contain at most one \
         contract."
    )]
    MultipleContracts(Span),
    #[error(
        "Program contains multiple scripts. A valid program should only contain at most one \
         script."
    )]
    MultipleScripts(Span),
    #[error(
        "Program contains multiple predicates. A valid program should only contain at most one \
         predicate."
    )]
    MultiplePredicates(Span),
    #[error(
        "Trait constraint was applied to generic type that is not in scope. Trait \
         \"{trait_name}\" cannot constrain type \"{type_name}\" because that type does not exist \
         in this scope."
    )]
    ConstrainedNonExistentType {
        trait_name: &'static str,
        type_name: &'static str,
        span: Span,
    },
    #[error(
        "Predicate definition contains multiple main functions. Multiple functions in the same \
         scope cannot have the same name."
    )]
    MultiplePredicateMainFunctions(Span),
    #[error(
        "Predicate declaration contains no main function. Predicates require a main function."
    )]
    NoPredicateMainFunction(Span),
    #[error("A predicate's main function must return a boolean.")]
    PredicateMainDoesNotReturnBool(Span),
    #[error("Script declaration contains no main function. Scripts require a main function.")]
    NoScriptMainFunction(Span),
    #[error(
        "Script definition contains multiple main functions. Multiple functions in the same scope \
         cannot have the same name."
    )]
    MultipleScriptMainFunctions(Span),
    #[error(
        "Attempted to reassign to a symbol that is not a variable. Symbol {name} is not a mutable \
         variable, it is a {kind}."
    )]
    ReassignmentToNonVariable {
        name: &'static str,
        kind: &'static str,
        span: Span,
    },
    #[error("Assignment to immutable variable. Variable {0} is not declared as mutable.")]
    AssignmentToNonMutable(String, Span),
    #[error(
        "Generic type \"{name}\" is not in scope. Perhaps you meant to specify type parameters in \
         the function signature? For example: \n`fn \
         {fn_name}<{comma_separated_generic_params}>({args}) -> ... `"
    )]
    TypeParameterNotInTypeScope {
        name: &'static str,
        span: Span,
        comma_separated_generic_params: String,
        fn_name: &'static str,
        args: String,
    },
    #[error(
        "Asm opcode has multiple immediates specified, when any opcode has at most one immediate."
    )]
    MultipleImmediates(Span),
    #[error(
        "Expected type {expected}, but found type {given}. The definition of this function must \
         match the one in the trait declaration."
    )]
    MismatchedTypeInTrait {
        span: Span,
        given: String,
        expected: String,
    },
    #[error("\"{name}\" is not a trait, so it cannot be \"impl'd\". ")]
    NotATrait { span: Span, name: &'static str },
    #[error("Trait \"{name}\" cannot be found in the current scope.")]
    UnknownTrait { span: Span, name: &'static str },
    #[error("Function \"{name}\" is not a part of trait \"{trait_name}\"'s interface surface.")]
    FunctionNotAPartOfInterfaceSurface {
        name: &'static str,
        trait_name: String,
        span: Span,
    },
    #[error("Functions are missing from this trait implementation: {missing_functions}")]
    MissingInterfaceSurfaceMethods {
        missing_functions: String,
        span: Span,
    },
    #[error("Expected {expected} type arguments, but instead found {given}.")]
    IncorrectNumberOfTypeArguments {
        given: usize,
        expected: usize,
        span: Span,
    },
    #[error(
        "Struct with name \"{name}\" could not be found in this scope. Perhaps you need to import \
         it?"
    )]
    StructNotFound { name: &'static str, span: Span },
    #[error(
        "The name \"{name}\" does not refer to a struct, but this is an attempted struct \
         declaration."
    )]
    DeclaredNonStructAsStruct { name: &'static str, span: Span },
    #[error(
        "Attempted to access field \"{field_name}\" of non-struct \"{name}\". Field accesses are \
         only valid on structs."
    )]
    AccessedFieldOfNonStruct {
        field_name: &'static str,
        name: &'static str,
        span: Span,
    },
    #[error(
        "Attempted to access a method on something that has no methods. \"{name}\" is a {thing}, \
         not a type with methods."
    )]
    MethodOnNonValue {
        name: &'static str,
        thing: &'static str,
        span: Span,
    },
    #[error("Initialization of struct \"{struct_name}\" is missing field \"{field_name}\".")]
    StructMissingField {
        field_name: &'static str,
        struct_name: &'static str,
        span: Span,
    },
    #[error("Struct \"{struct_name}\" does not have field \"{field_name}\".")]
    StructDoesNotHaveField {
        field_name: &'static str,
        struct_name: &'static str,
        span: Span,
    },
    #[error("No method named \"{method_name}\" found for type \"{type_name}\".")]
    MethodNotFound {
        span: Span,
        method_name: String,
        type_name: String,
    },
    #[error("The asterisk, if present, must be the last part of a path. E.g., `use foo::bar::*`.")]
    NonFinalAsteriskInPath { span: Span },
    #[error("Module \"{name}\" could not be found.")]
    ModuleNotFound { span: Span, name: String },
    #[error("\"{name}\" is a {actually}, not a struct. Fields can only be accessed on structs.")]
    NotAStruct {
        name: String,
        span: Span,
        actually: String,
    },
    #[error(
        "Field \"{field_name}\" not found on struct \"{struct_name}\". Available fields are:\n \
         {available_fields}"
    )]
    FieldNotFound {
        field_name: &'static str,
        available_fields: String,
        struct_name: &'static str,
        span: Span,
    },
    #[error("Could not find symbol \"{name}\" in this scope.")]
    SymbolNotFound { span: Span, name: &'static str },
    #[error(
        "Because this if expression's value is used, an \"else\" branch is required and it must \
         return type \"{r#type}\""
    )]
    NoElseBranch { span: Span, r#type: String },
    #[error("Use of type `Self` outside of a context in which `Self` refers to a type.")]
    UnqualifiedSelfType { span: Span },
    #[error(
        "Symbol \"{name}\" does not refer to a type, it refers to a {actually_is}. It cannot be \
         used in this position."
    )]
    NotAType {
        span: Span,
        name: String,
        actually_is: &'static str,
    },
    #[error(
        "This enum variant requires an instantiation expression. Try initializing it with \
         arguments in parentheses."
    )]
    MissingEnumInstantiator { span: Span },
    #[error(
        "This path must return a value of type \"{ty}\" from function \"{function_name}\", but it \
         does not."
    )]
    PathDoesNotReturn {
        span: Span,
        ty: String,
        function_name: &'static str,
    },
    #[error("Expected block to implicitly return a value of type \"{ty}\".")]
    ExpectedImplicitReturnFromBlockWithType { span: Span, ty: String },
    #[error("Expected block to implicitly return a value.")]
    ExpectedImplicitReturnFromBlock { span: Span },
    #[error(
        "This register was not initialized in the initialization section of the ASM expression. \
         Initialized registers are: {initialized_registers}"
    )]
    UnknownRegister {
        span: Span,
        initialized_registers: String,
    },
    #[error("This opcode takes an immediate value but none was provided.")]
    MissingImmediate { span: Span },
    #[error("This immediate value is invalid.")]
    InvalidImmediateValue { span: Span },
    #[error(
        "This expression was expected to return a value but no return register was specified. \
         Provide a register in the implicit return position of this asm expression to return it."
    )]
    InvalidAssemblyMismatchedReturn { span: Span },
    #[error("Variant \"{variant_name}\" does not exist on enum \"{enum_name}\"")]
    UnknownEnumVariant {
        enum_name: &'static str,
        variant_name: &'static str,
        span: Span,
    },
    #[error("Unknown opcode: \"{op_name}\".")]
    UnrecognizedOp { op_name: &'static str, span: Span },
    #[error("Unknown type \"{ty}\".")]
    TypeMustBeKnown { ty: String, span: Span },
    #[error("The value \"{val}\" is too large to fit in this 6-bit immediate spot.")]
    Immediate06TooLarge { val: u64, span: Span },
    #[error("The value \"{val}\" is too large to fit in this 12-bit immediate spot.")]
    Immediate12TooLarge { val: u64, span: Span },
    #[error("The value \"{val}\" is too large to fit in this 18-bit immediate spot.")]
    Immediate18TooLarge { val: u64, span: Span },
    #[error("The value \"{val}\" is too large to fit in this 24-bit immediate spot.")]
    Immediate24TooLarge { val: u64, span: Span },
    #[error("The opcode \"jnei\" is not valid in inline assembly. Use an enclosing if expression instead.")]
    DisallowedJnei { span: Span },
    #[error(
        "The opcode \"ji\" is not valid in inline assembly. Try using function calls instead."
    )]
    DisallowedJi { span: Span },
    #[error(
        "The opcode \"lw\" is not valid in inline assembly. Try assigning a static value to a variable instead."
    )]
    DisallowedLw { span: Span },
    #[error(
        "This op expects {expected} register(s) as arguments, but you provided {received} register(s)."
    )]
    IncorrectNumberOfAsmRegisters {
        span: Span,
        expected: usize,
        received: usize,
    },
    #[error("This op does not take an immediate value.")]
    UnnecessaryImmediate { span: Span },
    #[error("This reference is ambiguous, and could refer to either a module or an enum of the same name. Try qualifying the name with a path.")]
    AmbiguousPath { span: Span },
    #[error("This value is not valid within a \"str\" type.")]
    InvalidStrType { raw: String, span: Span },
    #[error("Unknown type name.")]
    UnknownType { span: Span },
    #[error("Bytecode can only support programs with up to 2^12 words worth of opcodes. Try refactoring into contract calls? This is a temporary error and will be implemented in the future.")]
    TooManyInstructions { span: Span },
    #[error(
        "No valid {} file (.{}) was found at {file_path}",
        crate::constants::LANGUAGE_NAME,
        crate::constants::DEFAULT_FILE_EXTENSION
    )]
    FileNotFound { span: Span, file_path: String },
    #[error("The file {file_path} could not be read: {stringified_error}")]
    FileCouldNotBeRead {
        span: Span,
        file_path: String,
        stringified_error: String,
    },
    #[error("This imported file must be a library. It must start with \"library <name>\", where \"name\" is the name of the library this file contains.")]
    ImportMustBeLibrary { span: Span },
    #[error("An enum instantiaton cannot contain more than one value. This should be a single value of type {ty}.")]
    MoreThanOneEnumInstantiator { span: Span, ty: String },
    #[error("This enum variant represents the unit type, so it should not be instantiated with any value.")]
    UnnecessaryEnumInstantiator { span: Span },
    #[error("Trait \"{name}\" does not exist in this scope.")]
    TraitNotFound { name: &'static str, span: Span },
    #[error("This expression is not valid on the left hand side of a reassignment.")]
    InvalidExpressionOnLhs { span: Span },
    #[error(
        "Function \"{method_name}\" expects {expected} arguments but you provided {received}."
    )]
    TooManyArgumentsForFunction {
        span: Span,
        method_name: &'static str,
        expected: usize,
        received: usize,
    },
    #[error(
        "Function \"{method_name}\" expects {expected} arguments but you provided {received}."
    )]
    TooFewArgumentsForFunction {
        span: Span,
        method_name: &'static str,
        expected: usize,
        received: usize,
    },
    #[error("This type is invalid in a function selector. A contract ABI function selector must be a known sized type, not generic.")]
    InvalidAbiType { span: Span },
    #[error("An ABI function must accept exactly four arguments.")]
    InvalidNumberOfAbiParams { span: Span },
    #[error("This is a {actually_is}, not an ABI. An ABI cast requires a valid ABI to cast the address to.")]
    NotAnAbi {
        span: Span,
        actually_is: &'static str,
    },
    #[error("An ABI can only be implemented for the `Contract` type, so this implementation of an ABI for type \"{ty}\" is invalid.")]
    ImplAbiForNonContract { span: Span, ty: String },
    #[error("The trait function \"{fn_name}\" in trait \"{trait_name}\" expects {num_args} arguments, but the provided implementation only takes {provided_args} arguments.")]
    IncorrectNumberOfInterfaceSurfaceFunctionParameters {
        fn_name: &'static str,
        trait_name: &'static str,
        num_args: usize,
        provided_args: usize,
        span: Span,
    },
    #[error("For now, ABI functions must take exactly four parameters, in this order: gas_to_forward: u64, coins_to_forward: u64, color_of_coins: b256, <your_function_parameter>: ?")]
    AbiFunctionRequiresSpecificSignature { span: Span },
    #[error("This parameter was declared as type {should_be}, but argument of type {provided} was provided.")]
    ArgumentParameterTypeMismatch {
        span: Span,
        should_be: String,
        provided: String,
    },
    #[error("Function {fn_name} is recursive, which is unsupported at this time.")]
    RecursiveCall { fn_name: &'static str, span: Span },
    #[error(
        "Function {fn_name} is recursive via {call_chain}, which is unsupported at this time."
    )]
    RecursiveCallChain {
        fn_name: &'static str,
        call_chain: String, // Pretty list of symbols, e.g., "a, b and c".
        span: Span,
    },
}

impl std::convert::From<TypeError> for CompileError {
    fn from(other: TypeError) -> CompileError {
        CompileError::TypeError(other)
    }
}

#[derive(Error, Debug, Clone)]
pub enum TypeError {
    #[error(
        "Mismatched types: Expected type {expected} but found type {received}. Type {received} is \
         not castable to type {expected}.\n help: {help_text}"
    )]
    MismatchedType {
        expected: String,
        received: String,
        help_text: String,
        span: Span,
    },
    #[error("This type is not known. Try annotating it with a type annotation.")]
    UnknownType { span: Span },
}

impl TypeError {
    pub(crate) fn internal_span(&self) -> &Span {
        use TypeError::*;
        match self {
            MismatchedType { span, .. } => span,
            UnknownType { span } => span,
        }
    }
}

impl CompileError {
    pub fn to_friendly_error_string(&self) -> String {
        match self {
            CompileError::ParseFailure { err, .. } => format!(
                "Error parsing input: {}",
                match &err.variant {
                    pest::error::ErrorVariant::ParsingError {
                        positives,
                        negatives,
                    } => {
                        let mut buf = String::new();
                        if !positives.is_empty() {
                            buf.push_str(&format!(
                                "expected one of [{}]",
                                positives
                                    .iter()
                                    .map(|x| format!("{:?}", x))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ));
                        }
                        if !negatives.is_empty() {
                            buf.push_str(&format!(
                                "did not expect any of [{}]",
                                negatives
                                    .iter()
                                    .map(|x| format!("{:?}", x))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ));
                        }
                        buf
                    }
                    pest::error::ErrorVariant::CustomError { message } => message.to_string(),
                }
            ),
            a => format!("{}", a),
        }
    }

    pub fn span(&self) -> (usize, usize) {
        let sp = self.internal_span();
        (sp.start(), sp.end())
    }

    pub fn path(&self) -> String {
        self.internal_span().path()
    }

    pub fn internal_span(&self) -> &Span {
        use CompileError::*;
        match self {
            UnknownVariable { span, .. } => span,
            UnknownVariablePath { span, .. } => span,
            UnknownFunction { span, .. } => span,
            NotAVariable { span, .. } => span,
            NotAFunction { span, .. } => span,
            Unimplemented(_, span) => span,
            TypeError(err) => err.internal_span(),
            ParseFailure { span, .. } => span,
            InvalidTopLevelItem(_, span) => span,
            Internal(_, span) => span,
            UnimplementedRule(_, span) => span,
            InvalidByteLiteralLength { span, .. } => span,
            ExpectedExprAfterOp { span, .. } => span,
            ExpectedOp { span, .. } => span,
            UnexpectedWhereClause(span) => span,
            UndeclaredGenericTypeInWhereClause { span, .. } => span,
            MultiplePredicates(span) => span,
            MultipleScripts(span) => span,
            MultipleContracts(span) => span,
            ConstrainedNonExistentType { span, .. } => span,
            MultiplePredicateMainFunctions(span) => span,
            NoPredicateMainFunction(span) => span,
            PredicateMainDoesNotReturnBool(span) => span,
            NoScriptMainFunction(span) => span,
            MultipleScriptMainFunctions(span) => span,
            ReassignmentToNonVariable { span, .. } => span,
            AssignmentToNonMutable(_, span) => span,
            TypeParameterNotInTypeScope { span, .. } => span,
            MultipleImmediates(span) => span,
            MismatchedTypeInTrait { span, .. } => span,
            NotATrait { span, .. } => span,
            UnknownTrait { span, .. } => span,
            FunctionNotAPartOfInterfaceSurface { span, .. } => span,
            MissingInterfaceSurfaceMethods { span, .. } => span,
            IncorrectNumberOfTypeArguments { span, .. } => span,
            StructNotFound { span, .. } => span,
            DeclaredNonStructAsStruct { span, .. } => span,
            AccessedFieldOfNonStruct { span, .. } => span,
            MethodOnNonValue { span, .. } => span,
            StructMissingField { span, .. } => span,
            StructDoesNotHaveField { span, .. } => span,
            MethodNotFound { span, .. } => span,
            NonFinalAsteriskInPath { span, .. } => span,
            ModuleNotFound { span, .. } => span,
            NotAStruct { span, .. } => span,
            FieldNotFound { span, .. } => span,
            SymbolNotFound { span, .. } => span,
            NoElseBranch { span, .. } => span,
            UnqualifiedSelfType { span, .. } => span,
            NotAType { span, .. } => span,
            MissingEnumInstantiator { span, .. } => span,
            PathDoesNotReturn { span, .. } => span,
            ExpectedImplicitReturnFromBlockWithType { span, .. } => span,
            ExpectedImplicitReturnFromBlock { span, .. } => span,
            UnknownRegister { span, .. } => span,
            MissingImmediate { span, .. } => span,
            InvalidImmediateValue { span, .. } => span,
            InvalidAssemblyMismatchedReturn { span, .. } => span,
            UnknownEnumVariant { span, .. } => span,
            UnrecognizedOp { span, .. } => span,
            TypeMustBeKnown { span, .. } => span,
            Immediate06TooLarge { span, .. } => span,
            Immediate12TooLarge { span, .. } => span,
            Immediate18TooLarge { span, .. } => span,
            Immediate24TooLarge { span, .. } => span,
            DisallowedJnei { span, .. } => span,
            DisallowedJi { span, .. } => span,
            DisallowedLw { span, .. } => span,
            IncorrectNumberOfAsmRegisters { span, .. } => span,
            UnnecessaryImmediate { span, .. } => span,
            AmbiguousPath { span, .. } => span,
            UnknownType { span, .. } => span,
            InvalidStrType { span, .. } => span,
            TooManyInstructions { span, .. } => span,
            FileNotFound { span, .. } => span,
            FileCouldNotBeRead { span, .. } => span,
            ImportMustBeLibrary { span, .. } => span,
            MoreThanOneEnumInstantiator { span, .. } => span,
            UnnecessaryEnumInstantiator { span, .. } => span,
            TraitNotFound { span, .. } => span,
            InvalidExpressionOnLhs { span, .. } => span,
            TooManyArgumentsForFunction { span, .. } => span,
            TooFewArgumentsForFunction { span, .. } => span,
            InvalidAbiType { span, .. } => span,
            InvalidNumberOfAbiParams { span, .. } => span,
            NotAnAbi { span, .. } => span,
            ImplAbiForNonContract { span, .. } => span,
            IncorrectNumberOfInterfaceSurfaceFunctionParameters { span, .. } => span,
            AbiFunctionRequiresSpecificSignature { span, .. } => span,
            ArgumentParameterTypeMismatch { span, .. } => span,
            RecursiveCall { span, .. } => span,
            RecursiveCallChain { span, .. } => span,
        }
    }

    /// Returns the line and column start and end
    pub fn line_col(&self) -> (LineCol, LineCol) {
        (
            self.internal_span().start_pos().line_col().into(),
            self.internal_span().end_pos().line_col().into(),
        )
    }
}
