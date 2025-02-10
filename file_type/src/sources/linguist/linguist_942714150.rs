use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_942714150: FileType = FileType {
    file_format: &FileFormat {
        id: 942_714_150,
        source_type: SourceType::Linguist,
        name: "Cue Sheet",
        extensions: &["cue"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
