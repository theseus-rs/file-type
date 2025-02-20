use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29944334: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_334,
        source_type: SourceType::Wikidata,
        name: "OpenOffice Impress template, version 1.0",
        extensions: &["sti"],
        media_types: &["application/vnd.sun.xml.impress.template"],
        signatures: &[],
        related_formats: &[],
    },
};
