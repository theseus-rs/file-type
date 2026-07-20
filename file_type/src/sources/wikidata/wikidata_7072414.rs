use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7072414: FileType = FileType {
    file_format: &FileFormat {
        id: 7_072_414,
        source_type: SourceType::Wikidata,
        name: "ODB++",
        extensions: &["xml", "zip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
