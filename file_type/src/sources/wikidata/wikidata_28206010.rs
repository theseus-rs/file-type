use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206010: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_010,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Y Luminance Channel (Compressed 8-bit)",
        extensions: &["cmy"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
