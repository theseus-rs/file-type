use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66142123: FileType = FileType {
    file_format: &FileFormat {
        id: 66_142_123,
        source_type: SourceType::Wikidata,
        name: "ACCDE file format",
        extensions: &["accde"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
