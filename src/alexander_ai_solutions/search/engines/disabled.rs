use crate::alexander_ai_solutions::config::Config;
use crate::alexander_ai_solutions::search::registry::SearchToolParams;
use crate::alexander_ai_solutions::tools::Tool;

pub(crate) fn build(_: &Config, _: SearchToolParams) -> Vec<Box<dyn Tool>> {
    tracing::debug!("[search] disabled — no search tools registered");
    Vec::new()
}
