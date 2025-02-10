use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_115055414: FileType = FileType {
    file_format: &FileFormat {
        id: 115_055_414,
        source_type: SourceType::Wikidata,
        name: "The Spectral Geologist Dataset",
        extensions: &["tsg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
