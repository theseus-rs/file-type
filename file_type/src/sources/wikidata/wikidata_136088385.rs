use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136088385: FileType = FileType {
    file_format: &FileFormat {
        id: 136_088_385,
        source_type: SourceType::Wikidata,
        name: "Celtx file format",
        extensions: &["celtx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
