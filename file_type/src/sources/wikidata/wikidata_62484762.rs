use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62484762: FileType = FileType {
    file_format: &FileFormat {
        id: 62_484_762,
        source_type: SourceType::Wikidata,
        name: "AccessData Custom Content Image, Encrypted version",
        extensions: &["ad1", "ad2", "ad3", "ad4", "ad5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
