use ixa::prelude::*;
// use rand_distr::Exp;
use crate::GI;
use crate::people::{InfectionStatus, Person, PersonId};
pub type InfectionStatusEvent = PropertyChangeEvent<Person, InfectionStatus>;

fn handle_infection_status_change(context: &mut Context, event: InfectionStatusEvent) {
    trace!("Handling infection status event");
    if event.current == InfectionStatus::I {
        let infector = event.entity_id;
        for infectee in get_contacts(context, infector) {
            schedule_infection_attempt(context, infector, infectee);
        }
        schedule_recovery(context, infector);
    }
}

fn schedule_infection_attempt(context: &mut Context, infector: PersonId, infectee: PersonId) {
    trace!("Scheduling infection");
    let current_time = context.get_current_time();
    context.add_plan(current_time + GI, move |context| {
        attempt_infection(context, infector, infectee)
    });
}

fn schedule_recovery(context: &mut Context, person: PersonId) {
    trace!("Schedule recovery");
    context.add_plan(context.get_current_time() + GI, move |context| {
        recover(context, person)
    });
}

fn recover(context: &mut Context, person: PersonId) {
    trace!("Recovery {}", person);
    context.set_property(person, InfectionStatus::R);
}

fn attempt_infection(context: &mut Context, infector: PersonId, infectee: PersonId) {
    trace!("Infecting");
    if context.get_property::<Person, InfectionStatus>(infector) == InfectionStatus::I
        && context.get_property::<Person, InfectionStatus>(infectee) == InfectionStatus::S
    {
        context.set_property(infectee, InfectionStatus::I);
    }
}

pub fn init(context: &mut Context) {
    trace!("Initializing infection_manager");
    context.subscribe_to_event::<InfectionStatusEvent>(handle_infection_status_change);
}
