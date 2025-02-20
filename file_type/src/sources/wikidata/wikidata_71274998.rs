use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71274998: FileType = FileType {
    file_format: &FileFormat {
        id: 71_274_998,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 8",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
