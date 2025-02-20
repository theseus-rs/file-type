use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105851526: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_526,
        source_type: SourceType::Wikidata,
        name: "Camtasia Project",
        extensions: &["tscproj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
