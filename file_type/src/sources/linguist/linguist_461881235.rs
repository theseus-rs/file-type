use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_461881235: FileFormat = FileFormat {
    id: 461_881_235,
    source_type: SourceType::Linguist,
    name: "Git Revision List",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
