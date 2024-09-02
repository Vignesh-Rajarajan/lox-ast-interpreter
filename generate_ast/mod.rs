
use std::io;
use std::io::Write;

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &str) -> io::Result<()> {
    define_ast(output_dir, "Expr", vec![
        "Binary: Box<Expr> left, Token operator, Box<Expr> right".to_string(),
        "Grouping: Box<Expr> expression".to_string(),
        "Literal: Option<Object> value".to_string(),
        "Unary: Token operator, Box<Expr> right".to_string()
    ])?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<String>) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = std::fs::File::create(path)?;
    let mut tree_types = Vec::new();
    write!(file, "use crate::token::*;\n")?;
    write!(file, "use crate::error::*;\n")?;
    for type_def in types {
        let (base_class_name, args) = type_def.split_once(':').unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let args_split = args.trim().split(',');
        let mut fields = Vec::new();
        for arg in args_split {
            let (type_name, name) = arg.trim().split_once(' ').unwrap();
            fields.push(format!("{}: {}", name, type_name));
        }
        tree_types.push(TreeType { base_class_name: base_class_name.trim().to_string(), class_name, fields });
    }

    write!(file, "pub enum {} {{\n", base_name)?;
    for tree_type in &tree_types {
        write!(file, "    {}({}),\n", tree_type.base_class_name, tree_type.class_name)?;
    }
    write!(file, "}}\n")?;

    write!(file, "impl {} {{\n", base_name)?;
    write!(file, "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{\n", base_name)?;
    write!(file, "        match self {{\n")?;
    for tree_type in &tree_types {
        write!(file, "            {}::{}(expr) => expr.accept(visitor),\n", base_name, tree_type.base_class_name)?;
    }
    write!(file, "        }}\n")?;
    write!(file, "    }}\n")?;
    write!(file, "}}\n")?;

    for tree_type in &tree_types {
        write!(file, "pub struct {} {{\n", tree_type.class_name)?;
        for field in &tree_type.fields {
            write!(file, "    pub {},\n", field)?;
        }
        write!(file, "}}\n")?;
    }

    write!(file, "pub trait ExprVisitor<T> {{\n")?;
    for tree_type in &tree_types {
        write!(file, "    fn visit_{}_{}(&self, expr: &{}) -> Result<T,LoxError>;\n",
               tree_type.base_class_name.to_lowercase(), base_name.to_lowercase(), tree_type.class_name)?;
    }
    write!(file, "}}\n")?;

    for tree_type in &tree_types {
        write!(file, "impl {} {{\n", tree_type.class_name)?;
        write!(file, "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{\n",base_name)?;
        write!(file, "        visitor.visit_{}_{}(self)\n", tree_type.base_class_name.to_lowercase(), base_name.to_lowercase())?;
        write!(file, "    }}\n")?;
        write!(file, "}}\n")?;
    }
    file.flush()?;
    Ok(())
}
