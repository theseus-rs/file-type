use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29944206: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_206,
        source_type: SourceType::Wikidata,
        name: "OpenOffice Draw template, version 1.0",
        extensions: &["std"],
        media_types: &["application/vnd.sun.xml.draw.template"],
        signatures: &[],
        related_formats: &[],
    },
};
