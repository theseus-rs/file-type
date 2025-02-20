use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71275233: FileType = FileType {
    file_format: &FileFormat {
        id: 71_275_233,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 7",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
