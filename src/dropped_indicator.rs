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

    pub fn new(dropped_indicator: Arc<()>) -> Self
    {

        Self
        {

            dropped_indicator

        }

    }

    pub fn has_dropped(&self) -> bool
    {

        Arc::strong_count(&self.dropped_indicator) < 2

    }

    pub fn not_dropped(&self) -> bool
    {

        Arc::strong_count(&self.dropped_indicator) == 2

    }
    

}