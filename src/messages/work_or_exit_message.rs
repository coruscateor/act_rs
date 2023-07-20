///
/// For fundamentally controlling what the actor does.
/// Do work or exit the Task or Thread?
/// 
pub enum WorkOrExitMessage<TWork>
{

    do_work(TWork),
    exit

}

pub enum WorkedErrorOrExitedMessage<TWorkDone, TError>
{

    work_done(TWorkDone),
    error(TError),
    exited

}


