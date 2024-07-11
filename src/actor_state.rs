use async_trait::async_trait;

///
/// The actor state trait for standard thread and blocking thread based actors.
/// 
/// The returned boolean values from the on_begin and run method implementations indicate whether or not actor execution should proceed.
/// 
pub trait ActorState
{

    fn beginning(&mut self) -> bool
    {

        true

    }

    fn run(&mut self) -> bool;

    fn ending(&mut self)
    {
    }

}

///
/// The start trait for async oriented actors.
/// 
/// The returned boolean values from the on_begin_async and run_async method implementations indicate whether or not actor execution should proceed.
///
#[async_trait]
pub trait AsyncActorState
{

    async fn beginning_async(&mut self) -> bool
    {

        true

    }

    async fn run_async(&mut self) -> bool;

    async fn ending_async(&mut self)
    {
    }

}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum CurrentActorState
{

    #[default]
    Beginning,
    Running,
    Ending

}

impl CurrentActorState
{

    pub fn new() -> Self
    {

        Self::default()

    }

    pub fn next(&mut self) -> bool
    {

        match self
        {

            CurrentActorState::Beginning =>
            {

                *self = CurrentActorState::Running;

                true

            }
            CurrentActorState::Running =>
            {

                *self = CurrentActorState::Ending;

                true

            },
            CurrentActorState::Ending =>
            {

                false

            }

        }

    }

}


