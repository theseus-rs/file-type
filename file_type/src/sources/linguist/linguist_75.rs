use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_75: FileFormat = FileFormat {
    id: 75,
    source_type: SourceType::Linguist,
    name: "Csound Score",
    extensions: &["sco"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
