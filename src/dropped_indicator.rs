use std::sync::Arc;

///
/// Used in actor implementations to determine whether or not the "front-end" part of the actor has been dropped.
/// 
pub struct DroppedIndicator
{

    dropped_indicator: Arc<()>

}

impl DroppedIndicator
{

    ///
    /// Constructs a new instance of DroppedIndicator.
    /// 
    pub fn new(dropped_indicator: Arc<()>) -> Self
    {

        Self
        {

            dropped_indicator

        }

    }

    ///
    /// For checking if the "front-end" of the actor has dropped. 
    /// 
    pub fn has_dropped(&self) -> bool
    {

        Arc::strong_count(&self.dropped_indicator) < 2

    }

    ///
    /// For checking if the "front-end" of the actor has not dropped. 
    /// 
    pub fn not_dropped(&self) -> bool
    {

        Arc::strong_count(&self.dropped_indicator) == 2

    }
    

}