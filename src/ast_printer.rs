
// pub(crate) struct AstPrinter;
//
// impl AstPrinter {
//     pub fn print(&self, expr: &Expr) -> Result<String, LoxResult> {
//         expr.accept(self)
//     }
//
//     // let name = "*".to_string();
//     // let expr1 = Box::new(Expr::Literal(LiteralExpr { value: Some(Object::Number(4.0)) }));
//     // let expr2 = Box::new(Expr::Literal(LiteralExpr { value: Some(Object::Number(3.0)) }));
//     // let exprs = [&expr1, &expr2];
//     //
//     // let mult_result = ast_printer.parenthesize(name, &exprs);
//     // // Result: "(* 4 3)"
//     // For the unary minus:
//     // let name = "-".to_string();
//     // let expr = Box::new(Expr::Grouping(GroupingExpr { expression: Box::new(/* the multiplication expression */) }));
//     // let exprs = [&expr];
//     //
//     // let final_result = ast_printer.parenthesize(name, &exprs);
//     // // Result: "(- (* 4 3))"
//     fn parenthesize(&self, name: String, exprs: &[&Expr]) -> Result<String, LoxResult> {
//         let mut builder = format!("({} ", name);
//         for expr in exprs {
//             builder = format!("{} {}", builder, self.print(expr)?);
//         }
//         Ok(format!("{} )", builder))
//     }
// }
//
// impl ExprVisitor<String> for AstPrinter {
//     fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<String, LoxResult> {
//         todo!()
//     }
//     fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxResult> {
//         self.parenthesize(expr.operator.lexeme.clone(), &[&expr.left, &expr.right])
//     }
//     fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxResult> {
//         self.parenthesize("group".to_string(), &[&expr.expression])
//     }
//
//     fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxResult> {
//         match &expr.value {
//             Some(value) => Ok(value.to_string()),
//             None => Ok("nil".to_string()),
//         }
//     }
//
//     fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<String, LoxResult> {
//         todo!()
//     }
//
//     fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxResult> {
//         self.parenthesize(expr.operator.lexeme.clone(), &[&expr.right])
//     }
//
//     fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<String, LoxResult> {
//         todo!()
//     }
// }
