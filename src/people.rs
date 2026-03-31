use ixa::prelude::*;
use ixa::trace;

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

pub fn init(context: &mut Context, i0: usize) {
    trace!("Initializing person");
    for _ in 0..i0 {
        let person: PersonId = context.add_entity(()).expect("failed to add person");
        context.set_property(person, InfectionStatus::I);
    }
}
