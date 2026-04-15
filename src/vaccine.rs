use crate::population::{PersonCreatedEvent, PersonId};
use ixa::prelude::*;

define_entity!(Vaccination);
define_property!(struct Vaccinee(PersonId), Vaccination);
define_rng!(VaccineRng);

pub trait Vaccine: PluginContext {
    fn vaccinate(&mut self, person_id: PersonId) -> Result<(), ()> {
        trace!("Vaccinating {person_id:?}");
        self.add_entity(with!(Vaccination, Vaccinee(person_id)))
            .unwrap();
        Ok(())
    }

    fn get_vaccination_history(
        &mut self,
        person_id: PersonId,
    ) -> Result<impl Iterator<Item = EntityId<Vaccination>>, ()> {
        let q = self.query_result_iterator(with!(Vaccination, Vaccinee(person_id)));
        Ok(q)
    }

    fn is_vaccinated(&mut self, person_id: PersonId) -> Result<bool, ()> {
        match self.get_vaccination_history(person_id)?.count() {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(()),
        }
    }
}

impl Vaccine for Context {}

pub fn init(context: &mut Context, p_vax: f64) {
    context.subscribe_to_event::<PersonCreatedEvent>(move |context, event| {
        if context.sample_distr(VaccineRng, rand_distr::Bernoulli::new(p_vax).unwrap()) {
            context.vaccinate(event.entity_id).unwrap();
        }
    })
}
