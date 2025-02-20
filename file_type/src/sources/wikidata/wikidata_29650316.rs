use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650316: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_316,
        source_type: SourceType::Wikidata,
        name: "Packed Font File Format",
        extensions: &["pk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
