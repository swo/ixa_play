use ixa::prelude::*;
use ixa::trace;
use rand_distr::Bernoulli;

use crate::{FRAC_HIGH_HANDWASHING, POPULATION};

define_entity!(Person);
define_property!(
    // The type of the property
    enum InfectionStatus {
        S,
        I,
        R,
    },
    // The entity the property is associated with
    Person,
    // The property's default value for newly created `Person` entities
    default_const = InfectionStatus::S
);

define_property!(
    enum HandwashingAdherence {
        Typical,
        High,
    },
    Person,
    default_const = HandwashingAdherence::Typical
);

define_rng!(PeopleRng);

/// Populates the "world" with the `POPULATION` number of people.
pub fn init(context: &mut Context) {
    trace!("Initializing people");
    for _ in 0..POPULATION {
        let person_id: PersonId = context.add_entity(()).expect("failed to add person");
        if context.sample_distr(PeopleRng, Bernoulli::new(FRAC_HIGH_HANDWASHING).unwrap()) {
            context.set_property(person_id, HandwashingAdherence::High);
        }
    }
}
