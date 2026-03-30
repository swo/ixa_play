use ixa::prelude::*;
// use rand_distr::Exp;

use crate::people::{InfectionStatus, Person, PersonId};
use crate::{GI, N_OFFSPRING};
pub type CreationEvent = EntityCreatedEvent<Person>;

// define_rng!(InfectionRng);
fn handle_person_creation(context: &mut Context, event: CreationEvent) {
    trace!("Handling person creation");
    schedule_infection(context, event.entity_id);
}

fn schedule_infection(context: &mut Context, person_id: PersonId) {
    trace!("Scheduling infection");
    let current_time = context.get_current_time();
    context.add_plan(current_time + GI, move |context| infect(context, person_id));
}

fn infect(context: &mut Context, infector: PersonId) {
    trace!("Infecting");
    for _ in 0..N_OFFSPRING {
        let _: PersonId = context.add_entity(()).expect("failed to add person");
    }
    context.set_property(infector, InfectionStatus::R);
}

pub fn init(context: &mut Context) {
    trace!("Initializing infection_manager");
    context.subscribe_to_event::<CreationEvent>(handle_person_creation);
}
