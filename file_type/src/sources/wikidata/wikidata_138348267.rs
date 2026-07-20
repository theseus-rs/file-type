use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138348267: FileType = FileType {
    file_format: &FileFormat {
        id: 138_348_267,
        source_type: SourceType::Wikidata,
        name: "Draco 2 file format",
        extensions: &["bin", "drc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
