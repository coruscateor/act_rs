use std::sync::Arc;

///
/// Used to determine whether or not the "front-end" or the "back-end" components of an actor have been dropped.
/// 
pub struct DroppedDetector
{

    dropped_indicator: Arc<()>

}

impl DroppedDetector
{

    ///
    /// Constructs a new instance of DroppedDetector.
    /// 
    pub fn new(dropped_indicator: Arc<()>) -> Self
    {

        Self
        {

            dropped_indicator

        }

    }

    ///
    /// Creates a new pair of DroppedDetectors, one for the inter-actor on the "client" end and another for the actor state on the "server" end.
    /// 
    pub fn new_pair() -> (DroppedDetector, DroppedDetector)
    {

        let the_arc = Arc::new(());

        let di_one = DroppedDetector::new(the_arc.clone());

        let di_two = DroppedDetector::new(the_arc);

        (di_one, di_two)

    }

    ///
    /// Like new_pair, but one DroppedDetector is Arc'd.
    /// 
    pub fn one_arcd() -> (Arc<DroppedDetector>, DroppedDetector)
    {

        let (di_one, di_two) = DroppedDetector::new_pair();

        (Arc::new(di_one), di_two)

    }

    ///
    /// For checking if the "front-end" or the "back-end" of the actor has dropped (depending on where its located). 
    /// 
    pub fn has_dropped(&self) -> bool
    {

        Arc::strong_count(&self.dropped_indicator) < 2

    }

    ///
    /// For checking if the "front-end" or the "back-end" of the actor has not dropped (depending on where its located). 
    /// 
    pub fn not_dropped(&self) -> bool
    {

        Arc::strong_count(&self.dropped_indicator) == 2

    }

}