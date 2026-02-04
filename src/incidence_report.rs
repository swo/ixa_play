use crate::infection_manager::InfectionStatusEvent;
use ixa::{Context, IxaError, trace};

fn handle_infection_status_change(_: &mut Context, event: InfectionStatusEvent) {
    trace!(
        "Recording infection status change from {:?} to {:?} for {:?}",
        event.previous, event.current, event.person_id
    );
}

pub fn init(context: &mut Context) -> Result<(), IxaError> {
    trace!("Initializing incidence report");
    context.subscribe_to_event::<InfectionStatusEvent>(handle_infection_status_change);
    Ok(())
}
