use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_69: FileType = FileType {
    file_format: &FileFormat {
        id: 69,
        source_type: SourceType::Linguist,
        name: "Rocq Prover",
        extensions: &["coq", "v"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
