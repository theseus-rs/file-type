use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_446573572: FileFormat = FileFormat {
    id: 446_573_572,
    source_type: SourceType::Linguist,
    name: "Nushell",
    extensions: &["nu"],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
