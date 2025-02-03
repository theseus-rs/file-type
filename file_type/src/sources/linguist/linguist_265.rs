use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_265: FileFormat = FileFormat {
    id: 265,
    source_type: SourceType::Linguist,
    name: "OpenRC runscript",
    extensions: &[],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
