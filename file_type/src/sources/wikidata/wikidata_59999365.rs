use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59999365: FileType = FileType {
    file_format: &FileFormat {
        id: 59_999_365,
        source_type: SourceType::Wikidata,
        name: "Secure DjVU",
        extensions: &["djv", "djvu"],
        media_types: &["image/vnd.djvu", "image/x-djvu"],
        signatures: &[],
        related_formats: &[],
    },
};
