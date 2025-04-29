
///
/// A boilerplate reducer intended to be used in actor run methods.
/// 
/// Expects “$this” to have get_input(&/&mut self) -> Result<\<?\>, \<Error?\>> and input_ok(&/&mut self, res: <Message?>) -> Result<(), <Error?>> methods.
/// 
/// Use the get_input method for calling and returning the result of a recv method of a channel receiver object.
/// 
/// Use the input_ok method for handling the Ok result of the get_input method. It also is expected to return a Result object (likely from channel usage).
/// 
/// Results in a bool value.
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
/// Expects “$this” to have get_input(&/&mut self) -> Result<\<?\>, \<Error?\>>, input_ok(&/&mut self, res: <Message?>) -> Result<(), <Error?>> and input_err(&/&mut self, err: <Error?>) methods.
/// 
/// Use the get_input method for calling and returning the result of a recv method of a channel receiver object.
/// 
/// Use the input_ok method for handling the Ok result of the get_input method. It also is expected to return a Result object (likely from channel usage).
/// 
/// input_err is for handling the error object from the get_input method call.
/// 
/// Results in a bool value.
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
/// Expects “$this” to have get_input(&/&mut self) -> Result<\<?\>, \<Error?\>>, input_ok(&/&mut self, res: <Message?>) -> Result<(), <Error?>>, input_err(&/&mut self, err: <Error?>) and input_ok_err(&/&mut self, err: <Error?>) methods.
/// 
/// Use the get_input method for calling and returning the result of a recv method of a channel receiver object.
/// 
/// Use the input_ok method for handling the Ok result of the get_input method. It also is expected to return a Result object (likely from channel usage).
/// 
/// input_err is for handling the error object from the get_input method call.
/// 
/// input_ok_err is for handling the error object from the input_ok method call.
/// 
/// Results in a bool value.
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
/// Expects “$this” to have get_input_async(&/&mut self) -> Result<\<?\>, \<Error?\>> and input_ok_async(res: <Message?>) -> Result<(), <Error?>> methods.
/// 
/// Use the get_input_async method for calling and returning the result of a recv method of a channel receiver object.
/// 
/// Use the input_ok_async method for handling the Ok result of the get_input_async method. It also is expected to return a Result object (likely from channel usage).
/// 
/// Results in a bool value.
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
/// Expects “$this” to have get_input_async(&/&mut self) -> Result<\<?\>, \<Error?\>>, input_ok_async(res: <Message?>) -> Result<(), <Error?>> and input_err_async(err: <Error?>) methods.
/// 
/// Use the get_input_async method for calling and returning the result of a recv method of a channel receiver object.
/// 
/// Use the input_ok_async method for handling the Ok result of the get_input_async method. It also is expected to return a Result object (likely from channel usage).
/// 
/// input_err_async is for handling the error object from the get_input_async method call.
/// 
/// Results in a bool value.
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
/// Expects “$this” to have get_input_async(&/&mut self) -> Result<\<?\>, \<Error?\>>, input_ok_async(res: <Message?>) -> Result<(), <Error?>>, input_err_async(err: <Error?>) and input_ok_err_async(err: <Error?>) methods.
/// 
/// Use the get_input_async method for calling and returning the result of a recv method of a channel receiver object.
/// 
/// Use the input_ok_async method for handling the Ok result of the get_input_async method. It also is expected to return a Result object (likely from channel usage).
/// 
/// input_err_async is for handling the error object from the get_input_async method call.
/// 
/// input_ok_err_async is for handling the error object from the input_ok_async method call.
/// 
/// Results in a bool value.
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