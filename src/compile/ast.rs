    use std::fmt::Debug;

    #[derive(Clone, Debug)]
    pub enum Program {
        Program(Vec<Block>),
    }

    #[derive(Clone, Debug)]
    pub enum Block {
        FunctionDefinition(
            String,
            Vec<TypedArgument>,
            String,
            Vec<Statement>,
        ),

        TypeDefinition(
            String, // type name
            Vec<TypedArgument>, // list of param names to their types
            Vec<Box<Expr>> // list of refinements
        )
    }

    #[derive(Clone, Debug)]
    pub enum TypedArgument {
        TypedArgument(String, String),
    }

    #[derive(Clone, Debug)]
    pub enum Statement {
        Assignment(String, Box<Expr>),
        TypeAssignment(String, String),
        Expr(Expr),
    }

    #[derive(Clone, Debug)]
    pub enum Expr {
        Number(i32),
        Op(Box<Expr>, Opcode, Box<Expr>),
        FunctionCall(String, Vec<Box<Expr>>),
        Conditional(Box<Expr>, Box<Expr>, Box<Expr>),
        Identifier(String),
    }

    #[derive(Clone, Debug)]
    pub enum Opcode {
        Mul,
        Div,
        Add,
        Sub,
        Gt,
        Lt,
        Gteq,
        Lteq
    }