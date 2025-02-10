use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27473537: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_537,
        source_type: SourceType::Wikidata,
        name: "Advanced Forensic Format, version 3.0",
        extensions: &["aff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
