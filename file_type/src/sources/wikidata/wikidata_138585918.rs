use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138585918: FileType = FileType {
    file_format: &FileFormat {
        id: 138_585_918,
        source_type: SourceType::Wikidata,
        name: "Sega Saturn Cinepak",
        extensions: &["cpk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
