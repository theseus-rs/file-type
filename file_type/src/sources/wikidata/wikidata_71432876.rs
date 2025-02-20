use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71432876: FileType = FileType {
    file_format: &FileFormat {
        id: 71_432_876,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 6",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
