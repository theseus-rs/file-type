use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_140370703: FileType = FileType {
    file_format: &FileFormat {
        id: 140_370_703,
        source_type: SourceType::Wikidata,
        name: "OpenOffice.org 1.0 Formula Document",
        extensions: &["sxm"],
        media_types: &["application/vnd.sun.xml.math"],
        signatures: &[],
        related_formats: &[],
    },
};
