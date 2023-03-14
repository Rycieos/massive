use rustpython_vm as vm;
use vm::builtins::PyStrRef;

pub(crate) use data_module::make_module;

fn main() {
    vm::Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_module("data".to_owned(), Box::new(make_module));
    })
    .enter(run);
}

fn run(vm: &vm::VirtualMachine) {
    match exec_module(vm, "prompt") {
        Ok(prompt) => println!("prompt: '{prompt}'"),
        Err(exc) => vm.print_exception(exc),
    };
}

fn exec_module(vm: &vm::VirtualMachine, module: &str) -> vm::PyResult<PyStrRef> {
    vm.insert_sys_path(vm.new_pyobj("src")).expect("add path");
    let module = vm.import(module, None, 0)?;
    let name_func = module.get_attr("generate_prompt", vm)?;
    let data = data_module::Data { hostname: "jaguar" };
    let result = vm.invoke(&name_func, (data,))?;
    let result: PyStrRef = result.try_into_value(vm)?;

    vm::PyResult::Ok(result)
}

#[vm::pymodule]
mod data_module {
    use indexmap::IndexMap;
    use rustpython_vm::convert::ToPyObject;
    use rustpython_vm::{
        function::{FuncArgs, IntoFuncArgs},
        pyclass, PyResult, VirtualMachine,
    };

    #[pyattr]
    #[pyclass(module = "data_module", name = "Data")]
    pub struct Data {
        hostname: &str,
    }

    #[pyclass]
    impl Data {
        #[pymethod]
        fn hostname(&self) -> PyResult<&str> {
            Ok(self.hostname)
        }
    }

    impl IntoFuncArgs for Data {
        fn into_args(self, vm: &VirtualMachine) -> FuncArgs {
            let mut data = IndexMap::new();
            data.insert("hostname", self.hostname.to_pyobject(vm));
            let args = vec![data.to_pyobject()];
            FuncArgs {
                args: args,
                kwargs: IndexMap::new(),
            }
        }
    }
}
