use ixa::prelude::*;

/// The act of administering a vaccine
pub struct VaccineAdministration<U, P> {
    /// The "time" of vaccination could be a literal clock time, or it could be something more abstract
    pub time: U,
    /// Could be general like "flu", or it could be a specific manufacturer, dosage, lot, etc.
    pub product: P,
}

/// Very general interface for vaccines
pub trait Vaccine<T: Entity, U, P, E>: PluginContext {
    fn vaccinate(&mut self, entity_id: EntityId<T>, time: U, product: P) -> Result<(), E>;

    /// Vaccination history is a collection of vaccination events
    fn get_vaccination_history(
        &mut self,
        entity_id: EntityId<T>,
    ) -> Result<&impl IntoIterator<Item = VaccineAdministration<U, P>>, E>;
}
