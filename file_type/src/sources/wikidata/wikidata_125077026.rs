use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125077026: FileType = FileType {
    file_format: &FileFormat {
        id: 125_077_026,
        source_type: SourceType::Wikidata,
        name: "Gregorian chant score file",
        extensions: &["gabc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
