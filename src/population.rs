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

    fn generate_contacts(&mut self, contactor: PersonId) {
        trace!("Generating contacts for {:?}", contactor);

        let n_offspring = self
            .get_global_property_value(Parameters)
            .unwrap()
            .n_offspring;

        let contactees: Vec<PersonId> = (0..n_offspring)
            .map(|_| self.add_entity(()).expect("failed to add person"))
            .collect();

        trace!("Contactees are: {:?}", contactees);

        self.get_data_mut(ContactsPlugin)
            .insert(contactor, Some(contactees));
    }

    fn get_contacts(&mut self, person_id: PersonId) -> Result<&Vec<PersonId>, &str> {
        trace!("Getting contacts for {:?}", person_id);

        if self
            .get_data(ContactsPlugin)
            .get(&person_id)
            .expect("every person should be initialized")
            .is_none()
        {
            self.generate_contacts(person_id);
        }

        match self.get_data(ContactsPlugin).get(&person_id) {
            None => Err("contact map not initialized on person creation"),
            Some(None) => Err("contacts not generated"),
            Some(Some(contactees)) => Ok(contactees),
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
        let person_id: PersonId = context.get_entity_iterator().next().unwrap();

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
                    p_vax: 0.0,
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
        let person_id: PersonId = context.get_entity_iterator().next().unwrap();

        // check this person has 3 contacts
        let contacts = context.get_contacts(person_id).unwrap();
        assert_eq!(contacts.clone().len(), n_offspring)
    }
}
