use std::path::PathBuf;

use ixa::prelude::*;
use ixa::trace;
use serde::Serialize;

use crate::infection_manager::CreationEvent;
use crate::people::PersonId;

#[derive(Serialize, Clone)]
struct CreationReportItem {
    time: f64,
    person_id: PersonId,
}

define_report!(CreationReportItem);

fn handle_creation(context: &mut Context, event: CreationEvent) {
    trace!("Recording creation event for ID {}", event.entity_id);
    context.send_report(CreationReportItem {
        time: context.get_current_time(),
        person_id: event.entity_id,
    });
}

pub fn init(context: &mut Context, output_dir: PathBuf) -> Result<(), IxaError> {
    trace!("Initializing incidence_report");

    context
        .report_options()
        .directory(output_dir)
        .overwrite(true);
    context.add_report::<CreationReportItem>("creation")?;
    context.subscribe_to_event::<CreationEvent>(handle_creation);
    Ok(())
}
