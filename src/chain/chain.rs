
//报销请求
#[derive(Debug)]
struct ReimbursementRequest {
    amount: f64,
    desc: String,
}

impl ReimbursementRequest {
    fn new(amount: f64, desc: &str) -> Self {
        Self {
            amount,
            desc: desc.to_string(),
        }
    }
}
//ReimbursementHandler【抽象处理角色（Handler）】
trait ReimbursementHandler {
    fn handle(&self, request:&ReimbursementRequest);
}

//ManagerHandler经理【具体处理者角色（ConcreteHandler）】
struct ManagerHandler{
    name: String,
    successor: Option<Box<dyn ReimbursementHandler>>,
}

impl ManagerHandler {
    fn new<H>(name: &str, handler:  H) -> Self
    where
        H: ReimbursementHandler+'static,
    {
        Self {
            name: name.to_string(),
            successor: Some(Box::new(handler)),
        }
    }
}

//经理处理报销:800以下可以处理
impl ReimbursementHandler for ManagerHandler {
    fn handle(&self, request: & ReimbursementRequest) {
        if request.amount < 800.0_f64 {
            println!(
                "经理:{}处理报销：{}元，报销情况：{}",
                self.name, request.amount, request.desc
            )
        } else {
            if self.successor.is_some() {
                self.successor.as_ref().unwrap().handle(request);
            }
        }
    }
}

//DepartmentHeadHandler部门负责人【具体处理者角色（ConcreteHandler）】
struct DepartmentHeadHandler {
    name: String,
    successor: Option<Box<dyn ReimbursementHandler>>,
}

impl DepartmentHeadHandler{
    fn new<H>(name: &str, handler: H) -> Self
    where
        H: ReimbursementHandler+ 'static,
    {
        Self {
            name: name.to_string(),
            successor: Some(Box::new(handler)),
        }
    }
}

//部门处理报销:4000以下可以处理
impl ReimbursementHandler for DepartmentHeadHandler {
    fn handle(&self, request: &ReimbursementRequest) {
        if request.amount < 4000.0_f64 {
            println!(
                "部门主管:{} 处理报销：{}元，报销情况：{}",
                self.name, request.amount, request.desc
            )
        } else {
            if self.successor.is_some() {
                self.successor.as_ref().unwrap().handle(request);
            }
        }
    }
}

//FinanceHandler财务处理人【具体处理者角色（ConcreteHandler）】
struct FinanceHandler{
    name: String,
}

impl FinanceHandler {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
//财务处理人处理报销
impl ReimbursementHandler for FinanceHandler {
    fn handle(&self, request: &ReimbursementRequest) {
        println!(
            "财务:{} 处理报销：{}元，报销情况：{}",
            self.name, request.amount, request.desc
        )
    }
}

#[test]
fn main(){
    let finance = FinanceHandler::new("小李");
    let department = DepartmentHeadHandler::new("小强", finance);
    let manager = ManagerHandler::new("小明", department);

    let request1 = ReimbursementRequest::new(799.0_f64, "王五北京出差高铁报销费用");
    let request2 = ReimbursementRequest::new(3999.0_f64, "王五上海出差8天住宿报销费用");
    let request3 = ReimbursementRequest::new(4500.0_f64, "王五杭州出差15天住宿报销费用");
   
    manager.handle(&request1);//经理能处理
    
    manager.handle(&request2);//经理不能处理，转给部门主管处理
    
    manager.handle(&request3);//经理不能处理，部分主管也不能处理，最后由财务处理

}