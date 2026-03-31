use ixa::prelude::*;
// use rand_distr::Exp;
use crate::contacts::ContactsExt;
use crate::people::{InfectionStatus, Person, PersonId};
pub type InfectionStatusEvent = PropertyChangeEvent<Person, InfectionStatus>;

fn handle_infection_status_change(context: &mut Context, event: InfectionStatusEvent, gi: f64) {
    trace!("Handling infection status event");

    if event.current == InfectionStatus::I {
        let infector = event.entity_id;

        // schedule infections and recovery for one generation interval in the future
        let t = context.get_current_time() + gi;

        for infectee in context.get_contacts(infector).unwrap() {
            schedule_infection_attempt(context, infector, infectee, t);
        }

        schedule_recovery(context, infector, t);
    }
}

fn schedule_infection_attempt(
    context: &mut Context,
    infector: PersonId,
    infectee: PersonId,
    time: f64,
) {
    trace!("Scheduling infection");
    context.add_plan(time, move |context| {
        attempt_infection(context, infector, infectee)
    });
}

fn schedule_recovery(context: &mut Context, person: PersonId, time: f64) {
    trace!("Scheduling recovery");
    context.add_plan(time, move |context| recover(context, person));
}

fn recover(context: &mut Context, person: PersonId) {
    trace!("Recovery {}", person);
    context.set_property(person, InfectionStatus::R);
}

fn attempt_infection(context: &mut Context, infector: PersonId, infectee: PersonId) {
    trace!("Attempting infection");
    // only do the infection if the planned infector is infectious and the planned infectee is susceptible
    if context.get_property::<Person, InfectionStatus>(infector) == InfectionStatus::I
        && context.get_property::<Person, InfectionStatus>(infectee) == InfectionStatus::S
    {
        trace!("{:?} infected {:?}", infector, infectee);
        context.set_property(infectee, InfectionStatus::I);
    } else {
        trace!("No infection")
    }
}

pub fn init(context: &mut Context, gi: f64) {
    trace!("Initializing infection_manager");
    context.subscribe_to_event::<InfectionStatusEvent>(move |context, event| {
        handle_infection_status_change(context, event, gi)
    });
}
