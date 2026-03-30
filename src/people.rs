use ixa::prelude::*;
use ixa::trace;

use crate::I0;

define_entity!(Person);
define_property!(
    enum InfectionStatus {
        S,
        I,
        R,
    },
    Person,
    default_const = InfectionStatus::S
);

pub fn init(context: &mut Context) {
    trace!("Initializing person");
    for _ in 0..I0 {
        let person: PersonId = context.add_entity(()).expect("failed to add person");
        context.set_property(person, InfectionStatus::I);
    }
}
