use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2661480: FileType = FileType {
    file_format: &FileFormat {
        id: 2_661_480,
        source_type: SourceType::Wikidata,
        name: "BSON",
        extensions: &["bson"],
        media_types: &["application/bson"],
        signatures: &[],
        related_formats: &[],
    },
};
