use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_336943375: FileType = FileType {
    file_format: &FileFormat {
        id: 336_943_375,
        source_type: SourceType::Linguist,
        name: "F*",
        extensions: &["fst", "fsti"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
