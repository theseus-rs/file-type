use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71828821: FileType = FileType {
    file_format: &FileFormat {
        id: 71_828_821,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 4",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
