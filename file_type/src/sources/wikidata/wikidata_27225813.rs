use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27225813: FileType = FileType {
    file_format: &FileFormat {
        id: 27_225_813,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Database, version 1.2",
        extensions: &["odb"],
        media_types: &["application/vnd.oasis.opendocument.database"],
        signatures: &[],
        related_formats: &[],
    },
};
