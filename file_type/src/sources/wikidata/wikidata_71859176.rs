use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71859176: FileType = FileType {
    file_format: &FileFormat {
        id: 71_859_176,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 11",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
