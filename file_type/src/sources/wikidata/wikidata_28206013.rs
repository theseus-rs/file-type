use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206013: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_013,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive I Color Channel (Compressed 8-bit)",
        extensions: &["cmi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
