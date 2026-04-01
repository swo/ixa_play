use crate::Parameters;
use crate::infection::InfectionStatus;
use ixa::prelude::*;
use std::collections::HashMap;

pub type PersonCreatedEvent = EntityCreatedEvent<Person>;
pub type ContactsMap = HashMap<PersonId, Option<Vec<PersonId>>>;

define_entity!(Person);
define_data_plugin!(ContactsPlugin, ContactsMap, HashMap::new());

pub trait ContactsExt: PluginContext {
    fn init_person_contacts(&mut self, person_id: PersonId) {
        trace!("Setting null contact for {:?}", person_id);
        self.get_data_mut(ContactsPlugin).insert(person_id, None);
    }

    fn generate_contacts(&mut self, contactor: PersonId) -> Vec<PersonId> {
        trace!("Generating contacts for {:?}", contactor);

        let n_offspring = self
            .get_global_property_value(Parameters)
            .unwrap()
            .n_offspring;

        let contactees: Vec<PersonId> = (0..n_offspring)
            .map(|_| self.add_entity(()).expect("failed to add person"))
            .collect();

        self.get_data_mut(ContactsPlugin)
            .insert(contactor, Some(contactees.clone()));

        trace!("Contactees are {:?}", contactees);

        contactees
    }

    fn get_contacts(&mut self, person_id: PersonId) -> Result<Vec<PersonId>, IxaError> {
        trace!("Getting contacts for {:?}", person_id);
        let value = self.get_data(ContactsPlugin).get(&person_id);

        match value {
            None => Err(IxaError::IxaError("slot not initialized".to_string())),
            // if contacts already exist, use them
            Some(Some(contacts)) => Ok(contacts.clone()),
            // otherwise, make new ones
            Some(None) => Ok(self.generate_contacts(person_id)),
        }
    }
}

impl ContactsExt for Context {}

fn handle_person_created(context: &mut Context, event: PersonCreatedEvent) {
    context.init_person_contacts(event.entity_id);
}

pub fn init(context: &mut Context, i0: usize) {
    trace!("Initializing contacts");
    context.subscribe_to_event::<PersonCreatedEvent>(handle_person_created);

    for _ in 0..i0 {
        let person: PersonId = context.add_entity(()).expect("failed to add person");
        context.set_property(person, InfectionStatus::I);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_person_has_none_contacts() {
        let mut context = Context::new();
        init(&mut context, 0);
        context.add_plan(0.0, |context| {
            context.add_entity::<Person, _>(()).unwrap();
        });
        context.execute();

        // there should be only one person in the simulation
        assert_eq!(context.get_entity_count::<Person>(), 1);
        // pull out their ID
        let person_id = context.get_entity_iterator::<Person>().next().unwrap();

        let value = context.get_data(ContactsPlugin).get(&person_id);
        assert!(value.is_some(), "slot should be initialized");
        assert!(value.unwrap().is_none(), "value in the slot should be None");
    }

    #[test]
    fn query_contacts_creates_them() {
        let n_offspring = 3;

        let mut context = Context::new();

        context
            .set_global_property_value(
                crate::Parameters,
                crate::ParametersValues {
                    i0: 0,
                    gi: 0.0,
                    max_time: 0.0,
                    n_offspring,
                },
            )
            .unwrap();

        init(&mut context, 0);
        context.add_plan(0.0, |context| {
            context.add_entity::<Person, _>(()).unwrap();
        });
        context.execute();
        // there should be only one person in the simulation
        assert_eq!(context.get_entity_count::<Person>(), 1);
        // pull out their ID
        let person_id = context.get_entity_iterator::<Person>().next().unwrap();

        // check this person has 3 contacts
        let contacts = context.get_contacts(person_id).unwrap();
        assert_eq!(contacts.len(), n_offspring)
    }
}
