use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_942714150: FileFormat = FileFormat {
    id: 942_714_150,
    source_type: SourceType::Linguist,
    name: "Cue Sheet",
    extensions: &["cue"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
