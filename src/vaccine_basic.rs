use crate::population::{Person, PersonId};
use crate::vaccine::{Vaccine, VaccineAdministration};
use ixa::prelude::*;
use std::collections::HashMap;

define_data_plugin!(VaccineAdministrationHistoryPlugin, HashMap<PersonId, Vec<VaccineAdministration<(), ()>>>, HashMap::new());

impl Vaccine<Person, (), (), ()> for Context {
    fn vaccinate(&mut self, entity_id: PersonId, time: (), product: ()) -> Result<(), ()> {
        let admin = VaccineAdministration { time, product };
        let data = self.get_data_mut(VaccineAdministrationHistoryPlugin);

        // initialize if needed
        if data.contains_key(&entity_id) {
            data.get_mut(&entity_id).ok_or(())?.push(admin);
        } else {
            data.insert(entity_id, vec![admin]);
        }

        Ok(())
    }

    fn get_vaccination_history(
        &mut self,
        entity_id: PersonId,
    ) -> Result<&Vec<VaccineAdministration<(), ()>>, ()> {
        Ok(self
            .get_data_mut(VaccineAdministrationHistoryPlugin)
            .entry(entity_id)
            .or_insert(vec![]))
    }
}

pub trait VaccineBasic: Vaccine<Person, (), (), ()> {
    fn is_vaccinated(&mut self, entity_id: PersonId) -> Result<bool, ()> {
        match self.get_vaccination_history(entity_id).iter().len() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(()),
        }
    }
}

impl VaccineBasic for Context {}
