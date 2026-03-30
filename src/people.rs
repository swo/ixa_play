use ixa::prelude::*;
use ixa::trace;

use crate::I0;

define_entity!(Person);
define_property!(
    enum InfectionStatus {
        I,
        R,
    },
    Person,
    default_const = InfectionStatus::I
);

/// Populates the "world" with the `POPULATION` number of people.
pub fn init(context: &mut Context) {
    trace!("Initializing person");
    for _ in 0..I0 {
        let _: PersonId = context.add_entity(()).expect("failed to add person");
    }
}
