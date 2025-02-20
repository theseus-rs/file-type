use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110443436: FileType = FileType {
    file_format: &FileFormat {
        id: 110_443_436,
        source_type: SourceType::Wikidata,
        name: "Bentley Microstation Hidden Line File",
        extensions: &["hln"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
