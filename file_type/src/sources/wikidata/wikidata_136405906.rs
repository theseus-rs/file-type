use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136405906: FileType = FileType {
    file_format: &FileFormat {
        id: 136_405_906,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Presentation, version 1.4",
        extensions: &["odp"],
        media_types: &["application/vnd.oasis.opendocument.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
