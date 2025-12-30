use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136411400: FileType = FileType {
    file_format: &FileFormat {
        id: 136_411_400,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Graphics, version 1.4",
        extensions: &["odg"],
        media_types: &["application/vnd.oasis.opendocument.graphics"],
        signatures: &[],
        related_formats: &[],
    },
};
