
///
/// An Enum desinged to make it convenient to package a portion of dara inteneded for asyncronous work.
///
pub enum WorkMessage<TData, TError>
{

    data(TData),
    error(TError),
    cancel,
    complete

}

impl<TData, TError> From<WorkMessageResult<TData, TError>> for WorkMessage<TData, TError>
{
    
    fn from(value: WorkMessageResult<TData, TError>) -> Self
    {
        
        match value
        {

            WorkMessageResult::data(val) => WorkMessage::data(val),
            WorkMessageResult::error(err) => WorkMessage::error(err),
            WorkMessageResult::canceled => WorkMessage::cancel,
            WorkMessageResult::completed => WorkMessage::complete,

        }

    }

}

///
/// Intended to contain the result, or partial result, of asynronous of work.
/// Can be considered the oposite of WorkMessage
/// 
pub enum WorkMessageResult<TData, TError>
{

    data(TData),
    error(TError),
    canceled,
    completed

}

impl<TData, TError> From<WorkMessage<TData, TError>> for WorkMessageResult<TData, TError>
{

    fn from(value: WorkMessage<TData, TError>) -> Self
    {
        
        match value
        {

            WorkMessage::data(val) => WorkMessageResult::data(val),
            WorkMessage::error(err) => WorkMessageResult::error(err),
            WorkMessage::cancel => WorkMessageResult::canceled,
            WorkMessage::complete => WorkMessageResult::completed,

        }

    }
    
}
