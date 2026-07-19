use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_140370519: FileType = FileType {
    file_format: &FileFormat {
        id: 140_370_519,
        source_type: SourceType::Wikidata,
        name: "OpenOffice.org 1.0 Graphics Document",
        extensions: &["sxd"],
        media_types: &["application/vnd.sun.xml.draw"],
        signatures: &[],
        related_formats: &[],
    },
};
