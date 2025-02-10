use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27824032: FileType = FileType {
    file_format: &FileFormat {
        id: 27_824_032,
        source_type: SourceType::Wikidata,
        name: "ar, Sixth Edition Unix variant",
        extensions: &["a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
