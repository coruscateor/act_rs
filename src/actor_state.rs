use async_trait::async_trait;

///
/// The actor state trait for standard thread and blocking thread based actors.
/// 
/// The returned boolean values from the on_begin and run method implementations indicate whether or not actor execution should proceed.
/// 
pub trait ActorState
{

    fn start(&mut self) -> bool
    {

        true

    }

    fn run(&mut self) -> bool;

    fn end(&mut self)
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

    async fn start_async(&mut self) -> bool
    {

        true

    }

    async fn run_async(&mut self) -> bool;

    async fn end_async(&mut self)
    {
    }

}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum CurrentActorPhase
{

    #[default]
    Start,
    Run,
    End

}

impl CurrentActorPhase
{

    pub fn new() -> Self
    {

        Self::default()

    }

    pub fn next(&mut self) -> bool
    {

        match self
        {

            CurrentActorPhase::Start =>
            {

                *self = CurrentActorPhase::Run;

                true

            }
            CurrentActorPhase::Run =>
            {

                *self = CurrentActorPhase::End;

                true

            },
            CurrentActorPhase::End =>
            {

                false

            }

        }

    }

    pub fn is_start(&self) -> bool
    {

        *self == CurrentActorPhase::Start

    }

    pub fn is_run(&self) -> bool
    {

        *self == CurrentActorPhase::Run

    }

    pub fn is_end(&self) -> bool
    {

        *self == CurrentActorPhase::End

    }

}


