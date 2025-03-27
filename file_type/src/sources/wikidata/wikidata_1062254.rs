use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1062254: FileType = FileType {
    file_format: &FileFormat {
        id: 1_062_254,
        source_type: SourceType::Wikidata,
        name: "Channel Definition Format",
        extensions: &["cdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
