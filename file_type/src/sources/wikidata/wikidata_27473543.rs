use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27473543: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_543,
        source_type: SourceType::Wikidata,
        name: "Advanced Forensic Format, version 4",
        extensions: &["aff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
