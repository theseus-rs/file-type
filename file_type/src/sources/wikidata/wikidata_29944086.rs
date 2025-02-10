use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29944086: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_086,
        source_type: SourceType::Wikidata,
        name: "OpenOffice Impress, version 1.0",
        extensions: &["sxi"],
        media_types: &["application/vnd.sun.xml.impress"],
        signatures: &[],
        related_formats: &[],
    },
};
