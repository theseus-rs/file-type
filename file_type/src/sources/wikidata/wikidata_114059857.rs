use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114059857: FileType = FileType {
    file_format: &FileFormat {
        id: 114_059_857,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Presentation, version 1.3",
        extensions: &["odp"],
        media_types: &["application/vnd.oasis.opendocument.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
