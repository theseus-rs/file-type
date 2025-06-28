use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134691828: FileType = FileType {
    file_format: &FileFormat {
        id: 134_691_828,
        source_type: SourceType::Wikidata,
        name: "NooJ corpus file",
        extensions: &["noc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
