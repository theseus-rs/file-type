use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1970847: FileType = FileType {
    file_format: &FileFormat {
        id: 1_970_847,
        source_type: SourceType::Wikidata,
        name: "two-line element set",
        extensions: &["tle"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
