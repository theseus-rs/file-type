use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27225806: FileType = FileType {
    file_format: &FileFormat {
        id: 27_225_806,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Database, version 1.1",
        extensions: &["odb"],
        media_types: &["application/vnd.oasis.opendocument.database"],
        signatures: &[],
        related_formats: &[],
    },
};
