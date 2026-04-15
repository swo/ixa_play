use crate::population::{Person, PersonCreatedEvent, PersonId};
use ixa::prelude::*;

define_property!(
    struct VaccinationStatus(bool),
    Person,
    default_const = VaccinationStatus(false)
);

define_rng!(VaccineRng);

pub trait Vaccine: PluginContext {
    fn vaccinate(&mut self, person_id: PersonId) -> Result<(), ()> {
        if self.is_vaccinated(person_id).unwrap() {
            return Err(());
        }

        trace!("Vaccinating {person_id:?}");
        self.set_property(person_id, VaccinationStatus(true));
        Ok(())
    }

    fn is_vaccinated(&mut self, person_id: PersonId) -> Result<bool, ()> {
        match self.get_property(person_id) {
            VaccinationStatus(x) => Ok(x),
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
