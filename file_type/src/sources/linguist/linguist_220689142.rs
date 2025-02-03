use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_220689142: FileFormat = FileFormat {
    id: 220_689_142,
    source_type: SourceType::Linguist,
    name: "Julia REPL",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
