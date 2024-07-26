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

//Macros

///
/// Calls "enter()" on the provided "$to_enter" parameter in a block, storing the result in a local constant. Then the provided "$func" parameter is called.
/// 
/// When the "$param" parameter is included, it is passed by reference to the provided "$func" parameter.
/// 
/// For use with tokio::runtime::Runtime and Handle objects.
/// 
/*
#[macro_export]
macro_rules! enter
{

    ($to_enter:ident, $func:expr) =>
    {
        
        {

            let _entered = $to_enter.enter();

            $func()

        }

    };
    ($to_enter:ident, $func:expr, $param:ident) =>
    {
        
        {

            let _entered = $to_enter.enter();

            $func(&$param)

        }

    };

}
*/

struct not_enter();

///
/// Calls "enter()" on the provided "$to_enter" parameter in a block, storing the result in a local constant. Then the provided "$expr" expression parameter is executed in this block.
/// 
#[macro_export]
macro_rules! enter
{

    ($to_enter:ident, $expr:expr) =>
    {
        
        {

            let _entered = $to_enter.enter();

            $expr

        }

    }

}

///
/// Like the "enter" macro but for when you want to pass the provided "$param" to the "$func" by mutable reference.
///
/*
#[macro_export]
macro_rules! enter_mut_param
{

    ($to_enter:ident, $func:expr, $param:ident) =>
    {
        
        {

            let _entered = $to_enter.enter();

            $func(&mut $param)

        }

    };

}
*/

struct not_enter_mut_param();
