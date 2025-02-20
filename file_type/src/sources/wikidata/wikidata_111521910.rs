use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111521910: FileType = FileType {
    file_format: &FileFormat {
        id: 111_521_910,
        source_type: SourceType::Wikidata,
        name: "Packed-Ice True Colour Picture",
        extensions: &["trp", "tru"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
