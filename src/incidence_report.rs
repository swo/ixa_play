use std::path::PathBuf;

use ixa::prelude::*;
use ixa::trace;
use serde::Serialize;

use crate::infection::{InfectionStatus, InfectionStatusEvent};
use crate::population::Person;

#[derive(Serialize, Clone)]
struct PrevalenceReportItem {
    time: f64,
    n: usize,
}

define_report!(PrevalenceReportItem);

fn handle_infection_status_event(context: &mut Context, event: InfectionStatusEvent) {
    trace!(
        "Recording creation event for ID {} from {:?} to {:?}",
        event.entity_id, event.previous, event.current
    );
    context.send_report(PrevalenceReportItem {
        time: context.get_current_time(),
        n: context.query_entity_count(with!(Person, InfectionStatus::I)),
    });
}

pub fn init(context: &mut Context, output_dir: PathBuf) -> Result<(), IxaError> {
    trace!("Initializing incidence_report");

    context
        .report_options()
        .directory(output_dir)
        .overwrite(true);
    context.add_report::<PrevalenceReportItem>("prevalence")?;
    context.subscribe_to_event::<InfectionStatusEvent>(handle_infection_status_event);
    Ok(())
}
