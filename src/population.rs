use crate::Parameters;
use crate::infection::InfectionStatus;
use ixa::prelude::*;

pub type PersonCreatedEvent = EntityCreatedEvent<Person>;

define_entity!(Person);
define_property!(struct Generation(usize), Person);
define_property!(struct Connector(Option<PersonId>), Person);

fn generate_connections(
    context: &mut Context,
    person_id: PersonId,
    generation: usize,
) -> Vec<PersonId> {
    let n_offspring = context
        .get_global_property_value(Parameters)
        .unwrap()
        .n_offspring;

    (0..n_offspring)
        .map(|_| {
            context
                .add_entity(with!(
                    Person,
                    Generation(generation),
                    Connector(Some(person_id))
                ))
                .unwrap()
        })
        .collect()
}

pub fn get_contacts(context: &mut Context, connector: PersonId) -> Vec<PersonId> {
    context
        .query(with!(Person, Connector(Some(connector))))
        .into_iter()
        .collect()
}

pub fn init(context: &mut Context, i0: usize, n_generations: usize) {
    trace!("Initializing contacts");

    // create the index cases
    let index_cases: Vec<PersonId> = (0..i0)
        .map(|_| {
            context
                .add_entity(with!(Person, Generation(0), Connector(None)))
                .unwrap()
        })
        .collect();

    // create the network
    for g in 0..(n_generations - 1) {
        let people: Vec<PersonId> = context
            .query(with!(Person, Generation(g)))
            .into_iter()
            .collect();
        for person in people {
            generate_connections(context, person, g + 1);
        }
    }

    // infect the index cases
    for person in index_cases {
        context.set_property(person, InfectionStatus::I);
    }
}
