use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_71829168: FileType = FileType {
    file_format: &FileFormat {
        id: 71_829_168,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 3",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
