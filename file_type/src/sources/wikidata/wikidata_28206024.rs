use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206024: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_024,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Device-dependent Data (Compressed 16-bit)",
        extensions: &["c16"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
