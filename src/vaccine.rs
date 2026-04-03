use ixa::prelude::*;

pub struct VaccineAdministration<U, P> {
    pub time: U,
    pub product: P,
}

pub trait Vaccine<T: Entity, U, P, E>: PluginContext {
    fn vaccinate(&mut self, entity_id: EntityId<T>, time: U, product: P) -> Result<(), E>;

    fn get_vaccination_history(
        &mut self,
        entity_id: EntityId<T>,
    ) -> Result<&Vec<VaccineAdministration<U, P>>, E>;
}
