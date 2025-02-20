use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29944082: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_082,
        source_type: SourceType::Wikidata,
        name: "OpenOffice Draw, version 1.0",
        extensions: &["sxd"],
        media_types: &["application/vnd.sun.xml.draw"],
        signatures: &[],
        related_formats: &[],
    },
};
