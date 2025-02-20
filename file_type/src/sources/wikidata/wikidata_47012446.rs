use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47012446: FileType = FileType {
    file_format: &FileFormat {
        id: 47_012_446,
        source_type: SourceType::Wikidata,
        name: "Microstation CAD Drawing file format",
        extensions: &["dgn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
