use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_55627273: FileFormat = FileFormat {
    id: 55_627_273,
    source_type: SourceType::Linguist,
    name: "Carbon",
    extensions: &["carbon"],
    media_types: &["text/x-go"],
    internal_signatures: &[],
    related_formats: &[],
};
