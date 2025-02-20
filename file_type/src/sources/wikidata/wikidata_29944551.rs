use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29944551: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_551,
        source_type: SourceType::Wikidata,
        name: "OpenOffice Calc, version 1.0",
        extensions: &["sxc"],
        media_types: &["application/vnd.sun.xml.calc"],
        signatures: &[],
        related_formats: &[],
    },
};
