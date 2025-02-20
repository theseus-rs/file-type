use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61727559: FileType = FileType {
    file_format: &FileFormat {
        id: 61_727_559,
        source_type: SourceType::Wikidata,
        name: "PrimeOCR file format, version 4",
        extensions: &["pro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
