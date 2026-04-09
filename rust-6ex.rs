#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expression {
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Value(i64),
}

fn eval(e: Expression) -> i64 {
    match e {
        Expression::Value(value) => value,
        Expression::Op { op, left, right } => {
            let l = eval(*left);
            let r = eval(*right);

            match op {
                Operation::Add => l + r,
                Operation::Sub => l - r,
                Operation::Mul => l * r,
                Operation::Div => l / r,
            }
        }
    }
}

fn main() {
    let expr = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(5)),
            right: Box::new(Expression::Value(3)),
        }),
    };

    println!("Результат: {}", eval(expr));
}