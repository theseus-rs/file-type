use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27225797: FileType = FileType {
    file_format: &FileFormat {
        id: 27_225_797,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Graphics, version 1.1",
        extensions: &["odg"],
        media_types: &["application/vnd.oasis.opendocument.graphics"],
        signatures: &[],
        related_formats: &[],
    },
};
