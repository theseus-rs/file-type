use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_274: FileFormat = FileFormat {
    id: 274,
    source_type: SourceType::Linguist,
    name: "PLpgSQL",
    extensions: &["pgsql", "sql"],
    media_types: &["text/x-sql"],
    internal_signatures: &[],
    related_formats: &[],
};
