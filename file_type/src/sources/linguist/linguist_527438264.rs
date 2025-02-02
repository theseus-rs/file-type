use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_527438264: FileFormat = FileFormat {
    id: 527_438_264,
    source_type: SourceType::Linguist,
    name: "Debian Package Control File",
    extensions: &["dsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
