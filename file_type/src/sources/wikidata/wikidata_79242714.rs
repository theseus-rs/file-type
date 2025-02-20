use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_79242714: FileType = FileType {
    file_format: &FileFormat {
        id: 79_242_714,
        source_type: SourceType::Wikidata,
        name: "Lotus Approach Database index",
        extensions: &["adx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
