use crate::{create_raw_string, free_raw_string, python::{add_new_name_idx_fn, exec_py, make_private, pocketpy, pocketpy_bridge, var_to_pocketpyref}, shared::{PtrMagic, module::Module, var::Var}};

pub(super) fn create_module(module: &Module, parent: Option<&str>) {
    // Get module name
    let module_name = match parent {
        Some(s) => format!("{s}.{}", module.name),
        None => module.name.clone(),
    };

    // Create module
    let c_module_name = create_raw_string!(module_name.clone());
    let pymodule = unsafe { pocketpy::py_newmodule(c_module_name) };

    // Add variables to module
    for var in module.variables.iter() {
        let var_name = var.name.clone();
        let c_var_name = create_raw_string!(var_name);
        let r0 = unsafe { pocketpy::py_getreg(0) };
        var_to_pocketpyref(r0, unsafe{Var::from_borrow(var.var)});
        
        // Set
        unsafe {
            let py_name = pocketpy::py_name(c_var_name);
            pocketpy::py_setattr(pymodule, py_name, r0);
            free_raw_string!(c_var_name);
        }
    }
    
    // Add callbacks to module... This also needs to go through the pybridge
    for method in module.callbacks.iter() {
        let full_name = method.full_name.clone();
        // Save function
        add_new_name_idx_fn(full_name.clone(), method.idx);

        // Private name
        let private_name = make_private(&full_name);

        let c_name = create_raw_string!(private_name.clone());
        let bridge_code = format!(r#"
def {}(*args):
    return {private_name}('{}', *args)
"#, method.name, full_name);

        // Register pocketpy_bridge
        unsafe {
            pocketpy::py_bindfunc(pymodule, c_name, Some(pocketpy_bridge));
        }

        // Run bridge_code in current module
        exec_py(&bridge_code, format!("<{}>", module_name).as_str(), &module_name);
        
        // Free c
        unsafe {
            free_raw_string!(c_name);
        }
    }

    // Do the same for internal modules
    for im in module.modules.iter() {
        create_module(im, Some(&module_name));
    }

    unsafe {
        free_raw_string!(c_module_name);
    }
}

// use rustpython_vm::{PyObjectRef, VirtualMachine};

// use crate::{_python::{create_function, var_to_pyobject}, shared::{PtrMagic, module::Module, var::Var}};

// fn create_internal_module(vm: &VirtualMachine, module: &Module, parent_path: Option<&str>) -> PyObjectRef {
//     let m_dict = vm.ctx.new_dict();
//     let module_name = match parent_path{
//         Some(path) => format!("{}.{}", path, module.name),
//         None => module.name.clone(),
//     };

//     m_dict.set_item("__name__", vm.ctx.new_str(module_name.clone()).into(), vm).unwrap();
//     m_dict.set_item("__package__", vm.ctx.none(), vm).unwrap();
//     m_dict.set_item("__loader__", vm.ctx.none(), vm).unwrap();
//     m_dict.set_item("__spec__", vm.ctx.none(), vm).unwrap();

//     // Variables
//     for variable in module.variables.iter() {
//         let var = unsafe { Var::from_borrow(variable.var) };
//         let pyobj = var_to_pyobject(vm, var);
//         m_dict.set_item(&variable.name, pyobj, vm).expect("Could not set a variable in Python module.");
//     }

//     // Callbacks
//     for callback in module.callbacks.iter() {
//         let func = create_function(vm, &callback.name, callback.idx);
//         m_dict.set_item(&callback.name, func, vm).expect("Can not define method in Python module.");
//     }

//     // Inner modules
//     let modules = vm.sys_module.get_attr("modules", vm).expect("Could not get inner sys Modules Python");
//     for inner_m in module.modules.iter() {
//         let m = create_internal_module(vm, inner_m, Some(&module_name));
//         let m_name = format!("{}.{}", module_name, inner_m.name.clone());
//         // Set in sys modules
//         modules.set_item(m_name.as_str(), m.clone(), vm).expect("Could not add internal Python module");
//         m_dict.set_item(&inner_m.name, m, vm).expect("Can not define inner module in Python module.");
//     }

//     vm.new_module(&module.name, m_dict.into(), vm.ctx.new_str("").into()).into()
// }

// /// Create a Python module.
// pub(super) fn create_module(vm: &VirtualMachine, module: Arc<Module>) {
//     let m = create_internal_module(vm, module.as_ref(), None);
//     let sys_modules = vm.sys_module.get_attr("modules", vm).expect("Could not get Sys Modules Python.");
//     sys_modules.set_item(&module.name, m, vm).expect("Could not add Python module.");
// }