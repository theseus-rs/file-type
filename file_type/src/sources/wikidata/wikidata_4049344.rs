use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4049344: FileType = FileType {
    file_format: &FileFormat {
        id: 4_049_344,
        source_type: SourceType::Wikidata,
        name: "SFNT",
        extensions: &["aat", "cff", "otf", "sil", "ttf"],
        media_types: &["font/sfnt"],
        signatures: &[],
        related_formats: &[],
    },
};
