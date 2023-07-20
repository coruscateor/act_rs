use std::sync::Arc;

pub struct DroppedIndicator<'a>
{

    dropped_indicator: &'a Arc<()>

}

impl<'a> DroppedIndicator<'a>
{

    pub fn new(dropped_indicator: &'a Arc<()>) -> Self
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

    pub fn has_not_dropped(&self) -> bool
    {

        Arc::strong_count(&self.dropped_indicator) == 2

    }
    

}