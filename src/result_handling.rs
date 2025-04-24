
///
/// A boilerplate reducer intended to be used in actor run methods.
/// 
/// Expects “$this” to have get_input and input_ok(res: <?>) methods.
/// 
#[macro_export]
macro_rules! get_input
{

    ($this:ident) =>
    {

        match $this.get_input()
        {

            Ok(res) =>
            {

                match $this.input_ok(res)
                {

                    Ok(_) =>
                    {

                        true

                    }
                    Err(_) =>
                    {

                        false

                    }

                }                

            }
            Err(_) =>
            {

                false

            }

        }

    }

}

///
/// A boilerplate reducer intended to be used in actor run methods.
/// 
/// Expects “$this” to have get_input, input_ok(res: <?>) and input_err(err: <?>) methods.
/// 
#[macro_export]
macro_rules! get_input_with_err
{

    ($this:ident) =>
    {

        match $this.get_input()
        {

            Ok(res) =>
            {

                match $this.input_ok(res)
                {

                    Ok(_) =>
                    {

                        true

                    }
                    Err(_err) =>
                    {

                        false

                    }

                }

            }
            Err(err) =>
            {

                $this.input_err(err);

                false

            }

        }

    }

}

///
/// A boilerplate reducer intended to be used in actor run methods.
/// 
/// Expects “$this” to have get_input, input_ok(res: <?>), input_err(err: <?>) and input_ok_err(err: <?>) methods.
/// 
#[macro_export]
macro_rules! get_input_with_errs
{

    ($this:ident) =>
    {

        match $this.get_input()
        {

            Ok(res) =>
            {

                match $this.input_ok(res)
                {

                    Ok(_) =>
                    {

                        true

                    }
                    Err(err) =>
                    {

                        $this.input_ok_err(err);

                        false

                    }

                }

            }
            Err(err) =>
            {

                $this.input_err(err);

                false

            }

        }

    }

}

//Async

///
/// A boilerplate reducer intended to be used in actor run_async methods.
/// 
/// Expects “$this” to have get_input_async and input_ok_async(res: <?>) methods.
/// 
#[macro_export]
macro_rules! get_input_async
{

    ($this:ident) =>
    {

        match $this.get_input_async().await
        {

            Ok(res) =>
            {

                match $this.input_ok_async(res).await
                {

                    Ok(_) =>
                    {

                        true

                    }
                    Err(_) =>
                    {

                        false

                    }

                }                

            }
            Err(_) =>
            {

                false

            }

        }

    }

}

///
/// A boilerplate reducer intended to be used in actor run_async methods.
/// 
/// Expects “$this” to have get_input_async, input_ok_async(res: <?>) and input_err_async(err: <?>) methods.
/// 
#[macro_export]
macro_rules! get_input_with_err_async
{

    ($this:ident) =>
    {

        match $this.get_input_async().await
        {

            Ok(res) =>
            {

                match $this.input_ok_async(res).await
                {

                    Ok(_) =>
                    {

                        true

                    }
                    Err(_) =>
                    {

                        false

                    }

                }

            }
            Err(err) =>
            {

                self.input_err_async(err);

                false

            }

        }

    }

}

///
/// A boilerplate reducer intended to be used in actor run_async methods.
/// 
/// Expects “$this” to have get_input_async, input_ok_async(res: <?>), input_err_async(err: <?>) and input_ok_err_async(err: <?>) methods.
/// 
#[macro_export]
macro_rules! get_input_with_errs_async
{

    ($this:ident) =>
    {

        match $this.get_input_async().await
        {

            Ok(res) =>
            {

                match $this.input_ok_async(res).await
                {

                    Ok(_) =>
                    {

                        true

                    }
                    Err(err) =>
                    {

                        $this.input_ok_err_async(err);

                        false

                    }

                }

            }
            Err(err) =>
            {

                $this.input_err_async(err);

                false

            }

        }

    }

}

/*
#[macro_export]
macro_rules! run_ok
{

    (result:expr) =>
    {

        match expr
        {

            Ok(res)
            {

                match self.run_ok(res)
                {

                    Ok(_)
                    {

                        true

                    }
                    Err(_)
                    {

                        false

                    }

                }

            }
            Err(_)
            {

                false

            }

        }

    }

}

#[macro_export]
macro_rules! run_ok_with_err
{

    (result:expr) =>
    {

        match expr
        {

            Ok(res)
            {

                match self.run_ok(res)
                {

                    Ok(_)
                    {

                        true

                    }
                    Err(_)
                    {

                        false

                    }

                }

            }
            Err(err)
            {

                self.run_err(err);

                false

            }

        }

    }

}

#[macro_export]
macro_rules! run_ok_with_err_both
{

    (result:expr) =>
    {

        match expr
        {

            Ok(res)
            {

                match self.run_ok(res)
                {

                    Ok(_)
                    {

                        true

                    }
                    Err(err)
                    {

                        self.run_err(err);

                        false

                    }

                }

            }
            Err(err)
            {

                self.run_err(err);

                false

            }

        }

    }

}
*/