use std::{fmt::Display, ops::Deref};

///
/// In-case you wanted another way to indicate whether or not an actor should proceed or exit.
/// 
#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
pub enum ActorFlow
{

    Proceed,
    Exit

}

impl ActorFlow
{

    pub fn match_core_control_flow<B, C>(cf: &core::ops::ControlFlow<B, C>) -> Self
    {

        match cf
        {

            std::ops::ControlFlow::Continue(_) => ActorFlow::Proceed,
            std::ops::ControlFlow::Break(_) => ActorFlow::Exit

        }

    }

    pub fn is_proceed(&self) -> bool
    {

        matches!(self, ActorFlow::Proceed)

    }

    pub fn is_exit(&self) -> bool
    {

        matches!(self, ActorFlow::Exit)

    }
    
}

impl From<bool> for ActorFlow
{

    ///
    /// true == Proceed, false == Exit
    /// 
    fn from(value: bool) -> Self
    {

        if value
        {

            Self::Proceed

        }
        else
        {

            Self::Exit
            
        }
        
    }

}

impl From<std::ops::ControlFlow<()>> for ActorFlow
{

    fn from(value: std::ops::ControlFlow<()>) -> Self
    {
        
        match value
        {

            std::ops::ControlFlow::Continue(_) => ActorFlow::Proceed,
            std::ops::ControlFlow::Break(_) => ActorFlow::Exit,

        }

    }

}

impl Display for ActorFlow
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        
        match self
        {

            ActorFlow::Proceed =>
            {

                write!(f, "ActorFlow::Proceed")

            }
            ActorFlow::Exit =>
            {

                write!(f, "ActorFlow::Exit")

            }

        }
        
    }

}

impl Deref for ActorFlow
{

    type Target = bool;

    fn deref(&self) -> &Self::Target
    {

        match self
        {

            ActorFlow::Proceed => &true,
            ActorFlow::Exit => &false

        }

    }

}