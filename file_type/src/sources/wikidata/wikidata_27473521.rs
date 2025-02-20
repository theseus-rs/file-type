use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27473521: FileType = FileType {
    file_format: &FileFormat {
        id: 27_473_521,
        source_type: SourceType::Wikidata,
        name: "Advanced Forensic Format, version 2.0",
        extensions: &["aff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
