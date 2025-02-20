use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108182078: FileType = FileType {
    file_format: &FileFormat {
        id: 108_182_078,
        source_type: SourceType::Wikidata,
        name: "Android App Bundle",
        extensions: &["aab"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
