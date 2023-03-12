use rustpython_vm as vm;
use vm::{builtins::PyStrRef, pymodule, PyResult};

pub(crate) use data_module::make_module;


fn main() {
    vm::Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_module("data".to_owned(), Box::new(make_module));
    }).enter(run);
}

fn run(vm: &vm::VirtualMachine) {
    match exec_module(vm, "prompt") {
        Ok(prompt) => println!("prompt: '{prompt}'"),
        Err(exc) => vm.print_exception(exc),
    };
}

fn exec_module(vm: &vm::VirtualMachine, module: &str) -> PyResult<PyStrRef> {
    vm.insert_sys_path(vm.new_pyobj("src"))
        .expect("add path");
    let module = vm.import(module, None, 0)?;
    let name_func = module.get_attr("generate_prompt", vm)?;
    let result = vm.invoke(&name_func, ())?;
    let result: PyStrRef = result.try_into_value(vm)?;

    vm::PyResult::Ok(result)
}

#[pymodule]
mod data_module {
    use rustpython_vm::PyResult;

    #[pyfunction]
    fn hostname() -> PyResult<String> {
        Ok("jaguar".to_owned())
    }
}
