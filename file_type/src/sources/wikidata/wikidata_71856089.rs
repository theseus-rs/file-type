use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71856089: FileType = FileType {
    file_format: &FileFormat {
        id: 71_856_089,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 9",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
