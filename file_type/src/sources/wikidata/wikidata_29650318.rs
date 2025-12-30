use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650318: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_318,
        source_type: SourceType::Wikidata,
        name: "PKPass",
        extensions: &["pkpass", "pkpasses"],
        media_types: &["application/vnd.apple.pkpass"],
        signatures: &[],
        related_formats: &[],
    },
};
