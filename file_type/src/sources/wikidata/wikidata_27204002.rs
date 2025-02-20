use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27204002: FileType = FileType {
    file_format: &FileFormat {
        id: 27_204_002,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Presentation, version 1.2",
        extensions: &["odp"],
        media_types: &["application/vnd.oasis.opendocument.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
