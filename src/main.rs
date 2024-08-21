mod contractos;
mod enums;
mod modelos;

use contractos::enums::EnumString;
use enums::work_flow_status::WorkFlowStatus;
use modelos::workflow::Workflow;

fn main() {
    let mut workflows: Vec<Result<Workflow, &str>> = Vec::new();

    let n = Workflow::new(String::from("Example Workflow 0"), WorkFlowStatus::Locked);
    let c = Workflow::new(String::from("Example Workflow 1"), WorkFlowStatus::Archived);
    let p = Workflow::new(String::from("Example Workflow 2"), WorkFlowStatus::Enable);
    let l = Workflow::new(String::from("Example Workflow 2"), WorkFlowStatus::Enable);
    workflows.push(n);
    workflows.push(c);
    workflows.push(p);
    workflows.push(l);
    println!(
        "{}                           {}             {}",
        "Identificador", "Nombre", "Estado"
    );
    println!("------------------------------------------------------------------");

    let mut _w = &Workflow::defaul();

    for row in &workflows {
        match row {
            Ok(workflow) => {
                println!(
                    "| {} | {} | {} | {}",
                    workflow.id.to_string(),
                    workflow.name,
                    workflow.state.to_str(),
                    _w == workflow
                );
                println!("------------------------------------------------------------------");
                _w = &workflow;
            }
            Err(e) => println!("Invalid record"),
        }
    }
}
