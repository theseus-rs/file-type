use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111722157: FileType = FileType {
    file_format: &FileFormat {
        id: 111_722_157,
        source_type: SourceType::Wikidata,
        name: "WiDE Project File",
        extensions: &["wpj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
