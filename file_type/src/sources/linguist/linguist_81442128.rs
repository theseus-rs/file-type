use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_81442128: FileFormat = FileFormat {
    id: 81_442_128,
    source_type: SourceType::Linguist,
    name: "PEG.js",
    extensions: &["peggy", "pegjs"],
    media_types: &["text/javascript"],
    signatures: &[],
    related_formats: &[],
};
