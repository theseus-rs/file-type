use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138747298: FileType = FileType {
    file_format: &FileFormat {
        id: 138_747_298,
        source_type: SourceType::Wikidata,
        name: "Fusion 360 Archival Library file",
        extensions: &["flbr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
