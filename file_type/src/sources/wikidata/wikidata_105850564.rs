use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_105850564: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_564,
        source_type: SourceType::Wikidata,
        name: "Camtasia Studio Project (UTF)",
        extensions: &["camproj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
