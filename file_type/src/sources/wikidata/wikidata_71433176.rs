use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71433176: FileType = FileType {
    file_format: &FileFormat {
        id: 71_433_176,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 5",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
