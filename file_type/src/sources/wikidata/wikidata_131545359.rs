use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131545359: FileType = FileType {
    file_format: &FileFormat {
        id: 131_545_359,
        source_type: SourceType::Wikidata,
        name: "Cloud Optimized Point Cloud file",
        extensions: &["copc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
