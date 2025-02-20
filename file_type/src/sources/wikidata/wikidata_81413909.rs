use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_81413909: FileType = FileType {
    file_format: &FileFormat {
        id: 81_413_909,
        source_type: SourceType::Wikidata,
        name: "Macromedia Director Shockwave Cast",
        extensions: &["cct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
