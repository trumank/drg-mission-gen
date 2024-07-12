pub(crate) mod discord;
pub(crate) mod plain;

use tabled::Tabled;

use crate::cleaned_deep_dive::Mission;

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
pub(crate) struct Stage {
    stage: usize,
    primary: &'static str,
    secondary: &'static str,
    warning: &'static str,
    mutator: &'static str,
}

pub(crate) fn mission_to_stage(i: usize, m: &Mission) -> Stage {
    Stage {
        stage: i + 1,
        primary: m
            .primary_objective
            .display_detailed(m.complexity, m.duration),
        secondary: m.secondary_objective.display(),
        warning: m.warning.map(|w| w.display()).unwrap_or(""),
        mutator: m.mutator.map(|mt| mt.display()).unwrap_or(""),
    }
}
