use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_21462816: FileType = FileType {
    file_format: &FileFormat {
        id: 21_462_816,
        source_type: SourceType::Wikidata,
        name: "Android Secure encrypted file",
        extensions: &["asec"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
