use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47165600: FileType = FileType {
    file_format: &FileFormat {
        id: 47_165_600,
        source_type: SourceType::Wikidata,
        name: "RealLegal E-Transcript file format",
        extensions: &["ptx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
