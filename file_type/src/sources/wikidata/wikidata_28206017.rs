use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206017: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_017,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Q Color Channel (Compressed 8-bit)",
        extensions: &["cmq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
