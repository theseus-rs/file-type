use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_115055515: FileType = FileType {
    file_format: &FileFormat {
        id: 115_055_515,
        source_type: SourceType::Wikidata,
        name: "The Spectral Geologist Dataset 7",
        extensions: &["tsg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
