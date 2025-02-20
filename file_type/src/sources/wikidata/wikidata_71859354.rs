use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71859354: FileType = FileType {
    file_format: &FileFormat {
        id: 71_859_354,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 12",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
