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
        vec!["error", "token", "object", "rc"],
        vec![
            "Assign: Token name, Rc<Expr> value".to_string(),
            "Binary: Rc<Expr> left, Token operator, Rc<Expr> right".to_string(),
            "Call: Rc<Expr> callee, Token paren, Vec<Rc<Expr>> arguments".to_string(),
            "Grouping: Rc<Expr> expression".to_string(),
            "Literal: Option<Object> value".to_string(),
            "Logical: Rc<Expr> left, Token operator, Rc<Expr> right".to_string(),
            "Unary: Token operator, Rc<Expr> right".to_string(),
            "Variable : Token name".to_string(),
        ],
    )?;

    define_ast(
        output_dir,
        "Stmt",
        vec!["error", "expr", "token", "rc"],
        vec![
            "Block : Rc<Vec<Rc<Stmt>>> statements".to_string(),
            "If : Rc<Expr> condition, Rc<Stmt> then_branch, Option<Rc<Stmt>> else_branch".to_string(),
            "Expression : Rc<Expr> expression".to_string(),
            "Function : Token name, Rc<Vec<Token>> params, Rc<Vec<Rc<Stmt>>> body".to_string(),
            "Break: Token token".to_string(),
            "Print : Rc<Expr> expression".to_string(),
            "Return : Token token, Option<Rc<Expr>> value".to_string(),
            "Var : Token name, Option<Rc<Expr>> initializer".to_string(),
            "While : Rc<Expr> condition, Rc<Stmt> body".to_string(),
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
        if import == "rc" {
            writeln!(file, "use std::rc::Rc;")?;
        } else {
            writeln!(file, "use crate::{}::*;", import)?;
        }
    }
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
            "    {}(Rc<{}>),",
            tree_type.base_class_name, tree_type.class_name
        )?;
    }
    writeln!(file, "}}")?;

    writeln!(file, "impl PartialEq for {} {{", base_name)?;
    writeln!(file, "    fn eq(&self, other: &Self) -> bool {{")?;
    writeln!(file, "        match (self, other) {{")?;
    for tree_type in &tree_types {
        writeln!(
            file,
            "            ({0}::{1}(expr1), {0}::{1}(expr2)) => Rc::ptr_eq(expr1, expr2),",
            base_name, tree_type.base_class_name
        )?;
    }
    writeln!(file, "          _=> false,")?;
    writeln!(file, "      }}")?;
    writeln!(file, "  }}")?;

    writeln!(file, "}}\n\nimpl Eq for {}{{}}\n", base_name)?;
    writeln!(file,"use std::hash::*;")?;
    writeln!(file, "impl Hash for {} {{",base_name)?;
    writeln!(file,"    fn hash<H: Hasher>(&self, hasher: &mut H)")?;
    writeln!(file,"    where H: Hasher, ")?;
    writeln!(file,"       {{ match self {{")?;
    for tree_type in &tree_types {
        writeln!(
            file,
            "            {0}::{1}(expr) => {{hasher.write_usize(Rc::as_ptr(expr) as usize);}}",
            base_name, tree_type.base_class_name
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file,"    }}")?;
    writeln!(file,"}}")?;
    writeln!(file, "impl {} {{", base_name)?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, wrapper: Rc<{}>, {}_visitor: &dyn {}Visitor<T>) -> Result<T, LoxResult> {{",
        base_name,
        base_name.to_lowercase(),
        base_name
    )?;
    writeln!(file, "        match self {{")?;
    for tree_type in &tree_types {
        writeln!(
            file,
            "            {0}::{1}(expr) => {3}_visitor.visit_{2}_{3}(wrapper,&expr),",
            base_name,
            tree_type.base_class_name, tree_type.base_class_name.to_lowercase(), base_name.to_lowercase()
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
            "    fn visit_{0}_{1}(&self, wrapper: Rc<{3}>, {1}: &{2}) -> Result<T,LoxResult>;",
            tree_type.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            tree_type.class_name,
            base_name
        )?;
    }
    writeln!(file, "}}")?;
    file.flush()?;
    Ok(())
}
