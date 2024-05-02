use std::sync::Mutex;
use std::{clone, collections::HashMap, process::Output,sync::Arc};

use taurpc::Router;
use scc::HashIndex;
use scc::ebr::Guard;

mod pyo3test{
    use pyo3::types::PyString;
    use pyo3::{prelude::*,types::PyModule};

    pub fn test_python() -> PyResult<String>{
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py|{
            let py_code = PyModule::from_code(
                py, r#"
            def py_funtion():
                return "Hello from python"
            "#, 
                "test.py", "py_code")?;
                
            let py_code_func = py_code.getattr("py_function");
            let py_code_func_res:&PyString = py_code_func?.call0()?.extract()?;
            Ok(py_code_func_res.to_string())
        })
    }
}


#[derive(Debug,Default)]
#[taurpc::ipc_type]
pub struct DataSource{
    pub name: String,
    pub db_url: String,
    pub db_password: String,
}
#[derive(Debug,Default)]
#[taurpc::ipc_type]
pub struct Controller{
    pub name: String,
    pub controller_path:String,
}

#[derive(Debug,Default)]
#[taurpc::ipc_type]
pub struct Visualization{
    pub name: String,
    pub viz_path: String,
}

#[derive(Debug,Default)]
#[taurpc::ipc_type]
pub struct Page{
    pub name: String,

}

#[derive(Debug)]
#[taurpc::ipc_type]
pub struct Dashboard{
    pub name:String,
    pub description:String,
    pub id: String,
    pub file_path:String,
    pub data_sources: Vec<DataSource>,
    pub controllers: Vec<Controller>,
    pub visualizations: Vec<Visualization>,
    pub pages:Vec<Page>,
}
impl Default for Dashboard{
    fn default() -> Self {
        Self { name: Default::default(), description: Default::default(), id: Default::default(), data_sources: Default::default(), controllers: Default::default(), visualizations: Default::default(),pages:Default::default(), file_path: Default::default() }
    }
}

#[taurpc::procedures(path="api",export_to="../src/lib/types.ts")]
trait AppApi {
    async fn hello_worldorini() -> String;
    async fn get_dashboards()->Vec<Dashboard>;
    async fn create_dashboard(dashboard:Dashboard,dashboard_name:String);
    async fn call_python_code()->String;
}

#[derive(Clone)]
struct AppApiImpl{
    dashboard_state:DashboardState,
}



#[taurpc::resolvers]
impl AppApi for AppApiImpl {
    async fn hello_worldorini(self) -> String{
        println!("HELLO RPC METHOD");
        format!("Testing impl")
    }
    async fn get_dashboards(self)->Vec<Dashboard>{

        let guard:Guard = Guard::new();
        let mut dashboards:Vec<Dashboard> = Vec::new();
        for board in self.dashboard_state.iter(&guard){
            dashboards.push(board.1.clone());
        }
        return  dashboards;
        
    }
    async fn create_dashboard(self, dashboard:Dashboard,dashboard_name:String){
        self.dashboard_state.insert(dashboard_name , dashboard);
    }
    async fn call_python_code(self)->String{
        pyo3test::test_python().unwrap()
    }
}

#[taurpc::procedures(path = "events",export_to="../src/lib/types.ts")]
trait Events {
    #[taurpc(event)]
    async fn test_ev();

    #[taurpc(event)]
    async fn state_changed(new_state: String);

    #[taurpc(event)]
    async fn vec_test(args: Vec<String>);

    #[taurpc(event)]
    async fn multiple_args(arg1: u16, arg2: Vec<String>);
}

#[derive(Clone)]
struct EventsImpl;

#[taurpc::resolvers]
impl Events for EventsImpl {}

type GlobalState = Arc<Mutex<String>>;
type DashboardState = Arc<HashIndex<String,Dashboard>>;

pub fn gen_router() -> Router{
    let hashidx:Arc<HashIndex<String,Dashboard>> = Arc::new(HashIndex::default());
    let dash:Dashboard = Dashboard { name: "dashboard1".to_owned(), description: "description".to_owned(), id: "1234".to_owned(), ..Default::default()};
    hashidx.insert("Chuck".to_owned(), dash);
    println!("Size of hashmap: {:?}",hashidx.capacity());
    let guard:Guard = Guard::new();
    for vals in hashidx.iter(&guard){
        println!("values: {:?}",vals.0);
    }
        
    

    let router = Router::new()
    .merge(AppApiImpl{
        dashboard_state: Arc::new(HashIndex::default())
    }.into_handler(),)
        .merge(EventsImpl.into_handler());

    return router;
}
