use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_383305: FileType = FileType {
    file_format: &FileFormat {
        id: 383_305,
        source_type: SourceType::Wikidata,
        name: "afio",
        extensions: &["af", "afio", "cpio"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
