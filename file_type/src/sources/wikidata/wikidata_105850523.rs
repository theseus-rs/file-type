use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105850523: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_523,
        source_type: SourceType::Wikidata,
        name: "Camtasia Studio Project",
        extensions: &["camproj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
