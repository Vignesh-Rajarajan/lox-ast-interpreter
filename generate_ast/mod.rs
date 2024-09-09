use std::io;
use std::io::Write;

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &str) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        vec!["error", "token", "object"],
        vec![
            "Assign: Token name, Box<Expr> value".to_string(),
            "Binary: Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping: Box<Expr> expression".to_string(),
            "Literal: Option<Object> value".to_string(),
            "Unary: Token operator, Box<Expr> right".to_string(),
            "Variable : Token name".to_string(),
        ],
    )?;

    define_ast(
        output_dir,
        "Stmt",
        vec!["error", "expr", "token"],
        vec![
            "Block : Vec<Stmt> statements".to_string(),
            "Expression : Expr expression".to_string(),
            "Print : Expr expression".to_string(),
            "Var : Token name, Option<Expr> initializer".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(
    output_dir: &str,
    base_name: &str,
    imports: Vec<&str>,
    types: Vec<String>,
) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = std::fs::File::create(path)?;
    let mut tree_types = Vec::new();
    for import in imports {
        writeln!(file, "use crate::{}::*;", import)?;
    }
    // writeln!(file, "use crate::token::*;")?;
    // writeln!(file, "use crate::error::*;")?;
    // writeln!(file, "use crate::object::*;")?;
    // writeln!(file, "use crate::expr::Expr;")?;
    for type_def in types {
        let (base_class_name, args) = type_def.split_once(':').unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let args_split = args.trim().split(',');
        let mut fields = Vec::new();
        for arg in args_split {
            let (type_name, name) = arg.trim().split_once(' ').unwrap();
            fields.push(format!("{}: {}", name, type_name));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    writeln!(file, "pub enum {} {{", base_name)?;
    for tree_type in &tree_types {
        writeln!(
            file,
            "    {}({}),",
            tree_type.base_class_name, tree_type.class_name
        )?;
    }
    writeln!(file, "}}")?;

    writeln!(file, "impl {} {{", base_name)?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{",
        base_name
    )?;
    writeln!(file, "        match self {{")?;
    for tree_type in &tree_types {
        writeln!(
            file,
            "            {}::{}(expr) => expr.accept(visitor),",
            base_name, tree_type.base_class_name
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    for tree_type in &tree_types {
        writeln!(file, "pub struct {} {{", tree_type.class_name)?;
        for field in &tree_type.fields {
            writeln!(file, "    pub {},", field)?;
        }
        writeln!(file, "}}")?;
    }

    writeln!(file, "pub trait {}Visitor<T> {{", base_name)?;
    for tree_type in &tree_types {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, expr: &{}) -> Result<T,LoxError>;",
            tree_type.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            tree_type.class_name
        )?;
    }
    writeln!(file, "}}")?;

    for tree_type in &tree_types {
        writeln!(file, "impl {} {{", tree_type.class_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{",
            base_name
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            tree_type.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
    }
    file.flush()?;
    Ok(())
}
