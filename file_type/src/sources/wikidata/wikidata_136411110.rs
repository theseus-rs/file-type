use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136411110: FileType = FileType {
    file_format: &FileFormat {
        id: 136_411_110,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Database, version 1.4",
        extensions: &["odb"],
        media_types: &["application/vnd.oasis.opendocument.base"],
        signatures: &[],
        related_formats: &[],
    },
};
