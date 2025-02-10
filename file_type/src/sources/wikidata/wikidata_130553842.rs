use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130553842: FileType = FileType {
    file_format: &FileFormat {
        id: 130_553_842,
        source_type: SourceType::Wikidata,
        name: "QVT Operational Mapping language file format",
        extensions: &["qvto"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
