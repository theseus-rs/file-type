use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_140370325: FileType = FileType {
    file_format: &FileFormat {
        id: 140_370_325,
        source_type: SourceType::Wikidata,
        name: "OpenOffice.org 1.0 Presentation Document",
        extensions: &["sxi"],
        media_types: &["application/vnd.sun.xml.impress"],
        signatures: &[],
        related_formats: &[],
    },
};
