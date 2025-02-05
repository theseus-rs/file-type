use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_422: FileFormat = FileFormat {
    id: 422,
    source_type: SourceType::Linguist,
    name: "TI Program",
    extensions: &["8xp", "8xp.txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
