use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206006: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_006,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Device-dependent Uncompressed 16-bit Data",
        extensions: &["i16"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
