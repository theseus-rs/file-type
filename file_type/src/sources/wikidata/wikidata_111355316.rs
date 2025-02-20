use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111355316: FileType = FileType {
    file_format: &FileFormat {
        id: 111_355_316,
        source_type: SourceType::Wikidata,
        name: "UltraTracker wave file",
        extensions: &["uwf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
