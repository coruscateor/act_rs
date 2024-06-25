use tokio::runtime::{Handle, Runtime};

//Runtime

pub fn runtime_enter<F, R>(runtime: &Runtime, func: F) -> R
    where F: FnOnce() -> R
{

    let _entered = runtime.enter();

    func()

}

pub fn runtime_enter_param<F, P, R>(runtime: &Runtime, param: &P, func: F) -> R
    where F: FnOnce(&P) -> R
{

    let _entered = runtime.enter();

    func(param)

}

pub fn runtime_enter_mut_param<F, P, R>(runtime: &Runtime, param: &mut P, func: F) -> R
    where F: FnOnce(&mut P) -> R
{

    let _entered = runtime.enter();

    func(param)

}

//Handle

pub fn handle_enter<F, R>(handle: &Handle, func: F) -> R
    where F: FnOnce() -> R
{

    let _entered = handle.enter();

    func()

}

pub fn handle_enter_param<F, P, R>(handle: &Handle, param: &P, func: F) -> R
    where F: FnOnce(&P) -> R
{

    let _entered = handle.enter();

    func(param)

}

pub fn handle_enter_mut_param<F, P, R>(handle: &Handle, param: &mut P, func: F) -> R
    where F: FnOnce(&mut P) -> R
{

    let _entered = handle.enter();

    func(param)

}
