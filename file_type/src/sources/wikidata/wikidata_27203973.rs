use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27203973: FileType = FileType {
    file_format: &FileFormat {
        id: 27_203_973,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Presentation, version 1.1",
        extensions: &["odp"],
        media_types: &["application/vnd.oasis.opendocument.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
