#![allow(dead_code)]
fn main() {
    let operation: Box<dyn Operation> = OperationFactory::build_operation(Operator::Add, 3, 3);
    let res = (*operation).get_result();
    println!("Add res: {}", res);
    let operation: Box<dyn Operation> = OperationFactory::build_operation(Operator::Sub, 3, 3);
    let res = (*operation).get_result();
    println!("Sub res: {}", res);
    let operation: Box<dyn Operation> = OperationFactory::build_operation(Operator::Mul, 3, 3);
    let res = (*operation).get_result();
    println!("Mul res: {}", res);
    let operation: Box<dyn Operation> = OperationFactory::build_operation(Operator::Div, 3, 3);
    let res = (*operation).get_result();
    println!("Div res: {}", res);
    let operation: Box<dyn Operation> = OperationFactory::build_operation(Operator::Div, 3, 0);
    let res = (*operation).get_result();
    println!("Div by zero res: {}", res);
}
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

struct OperationFactory;
impl OperationFactory {
    fn build_operation(op: Operator, x: isize, y: isize) -> Box<dyn Operation> {
        match op {
            Operator::Add => Box::new(AddOperation { x, y }),
            Operator::Sub => Box::new(SubOperation { x, y }),
            Operator::Mul => Box::new(MulOperation { x, y }),
            Operator::Div => Box::new(DivOperation { x, y }),
        }
    }
}

trait Operation {
    fn get_result(&self) -> isize;
}

struct AddOperation {
    x: isize,
    y: isize,
}

impl Operation for AddOperation {
    fn get_result(&self) -> isize {
        self.add()
    }
}

impl AddOperation {
    fn add(&self) -> isize {
        self.x + self.y
    }
}

struct SubOperation {
    x: isize,
    y: isize,
}

impl Operation for SubOperation {
    fn get_result(&self) -> isize {
        self.sub()
    }
}

impl SubOperation {
    fn sub(&self) -> isize {
        self.x - self.y
    }
}

struct MulOperation {
    x: isize,
    y: isize,
}

impl Operation for MulOperation {
    fn get_result(&self) -> isize {
        self.mul()
    }
}

impl MulOperation {
    fn mul(&self) -> isize {
        self.x * self.y
    }
}

struct DivOperation {
    x: isize,
    y: isize,
}

impl Operation for DivOperation {
    fn get_result(&self) -> isize {
        self.div()
    }
}

impl DivOperation {
    fn div(&self) -> isize {
        if self.y != 0 {
            self.x / self.y
        } else {
            0
        }
    }
}
