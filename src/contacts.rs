use std::collections::HashMap;

use crate::N_OFFSPRING;
use crate::people::{Person, PersonId};
use ixa::prelude::*;

pub type PersonCreatedEvent = EntityCreatedEvent<Person>;
pub type ContactsMap = HashMap<PersonId, Option<Vec<PersonId>>>;

define_data_plugin!(ContactsPlugin, ContactsMap, HashMap::new());

pub trait ContactsExt: PluginContext {
    fn init_person_contacts(&mut self, person_id: PersonId) {
        trace!("Setting null contact for {:?}", person_id);
        self.get_data_mut(ContactsPlugin).insert(person_id, None);
    }

    fn generate_contacts(&mut self, contactor: PersonId) -> Vec<PersonId> {
        trace!("Generating contacts for {:?}", contactor);
        let contactees: Vec<PersonId> = (0..N_OFFSPRING)
            .map(|_| self.add_entity(()).expect("failed to add person"))
            .collect();

        self.get_data_mut(ContactsPlugin)
            .insert(contactor, Some(contactees.clone()));

        trace!("Contactees are {:?}", contactees);

        contactees
    }

    fn get_contacts(&mut self, person_id: PersonId) -> Result<Vec<PersonId>, IxaError> {
        trace!("Getting contacts for {:?}", person_id);
        if let Some(Some(contacts)) = self.get_data(ContactsPlugin).get(&person_id).cloned() {
            // if contacts already exist, use them
            Ok(contacts)
        } else {
            // otherwise, make new ones
            Ok(self.generate_contacts(person_id))
        }
    }
}

impl ContactsExt for Context {}

fn handle_person_created(context: &mut Context, event: PersonCreatedEvent) {
    context.init_person_contacts(event.entity_id);
}

pub fn init(context: &mut Context) {
    trace!("Initializing contacts");
    context.subscribe_to_event::<PersonCreatedEvent>(handle_person_created);
}
