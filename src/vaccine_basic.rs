use crate::population::{Person, PersonId};
use crate::vaccine::{Vaccine, VaccineAdministration};
use ixa::prelude::*;
use std::collections::HashMap;

// Concrete data storage: from PersonId to a vector of vaccine administrations
define_data_plugin!(VaccineAdministrationHistoryPlugin, HashMap<PersonId, Vec<VaccineAdministration<(), ()>>>, HashMap::new());

// Concrete implementation of Vaccine, where I don't care for the vaccination time, product, or error
impl Vaccine<Person, (), (), ()> for Context {
    fn vaccinate(&mut self, entity_id: PersonId, time: (), product: ()) -> Result<(), ()> {
        let admin = VaccineAdministration { time, product };

        // push onto an existing vector, or insert and return a new one
        self.get_data_mut(VaccineAdministrationHistoryPlugin)
            .entry(entity_id)
            .or_default()
            .push(admin);

        Ok(())
    }

    fn get_vaccination_history(
        &mut self,
        entity_id: PersonId,
    ) -> Result<&impl IntoIterator<Item = VaccineAdministration<(), ()>>, ()> {
        // if no existing history, initialize with empty vector
        Ok(self
            .get_data_mut(VaccineAdministrationHistoryPlugin)
            .entry(entity_id)
            .or_default())
    }
}

pub trait VaccineBasic: Vaccine<Person, (), (), ()> {
    /// Vaccinated if has had exactly one dose. Multiple doses are unexpected.
    fn is_vaccinated(&mut self, entity_id: PersonId) -> Result<bool, ()> {
        match self.get_vaccination_history(entity_id).iter().len() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(()),
        }
    }
}

impl VaccineBasic for Context {}
