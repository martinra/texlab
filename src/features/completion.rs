mod acronym_ref;
mod argument;
mod begin_snippet;
pub mod builder;
mod citation;
mod color;
mod color_model;
mod component_command;
mod component_environment;
mod entry_type;
mod field;
mod glossary_ref;
mod import;
mod include;
mod label;
mod theorem;
mod tikz_library;
mod user_command;
mod user_environment;

use lsp_types::{CompletionList, Position, Url};

use crate::{features::completion::builder::CompletionBuilder, util::cursor::CursorContext, Db};

pub const COMPLETION_LIMIT: usize = 50;

pub fn complete(db: &dyn Db, uri: &Url, position: Position) -> Option<CompletionList> {
    let context = CursorContext::new(db, uri, position, ())?;
    let mut builder = CompletionBuilder::new(&context);
    log::debug!("[Completion] Cursor: {:?}", context.cursor);
    entry_type::complete(&context, &mut builder);
    field::complete(&context, &mut builder);
    argument::complete(&context, &mut builder);
    citation::complete(&context, &mut builder);
    import::complete(&context, &mut builder);
    color::complete(&context, &mut builder);
    color_model::complete(&context, &mut builder);
    acronym_ref::complete(&context, &mut builder);
    glossary_ref::complete(&context, &mut builder);
    include::complete(&context, &mut builder);
    label::complete(&context, &mut builder);
    tikz_library::complete(&context, &mut builder);
    component_environment::complete(&context, &mut builder);
    theorem::complete(&context, &mut builder);
    user_environment::complete(&context, &mut builder);
    begin_snippet::complete(&context, &mut builder);
    component_command::complete(&context, &mut builder);
    user_command::complete(&context, &mut builder);
    Some(builder.finish())
}
