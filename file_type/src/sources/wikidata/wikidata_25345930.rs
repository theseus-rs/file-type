use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_25345930: FileType = FileType {
    file_format: &FileFormat {
        id: 25_345_930,
        source_type: SourceType::Wikidata,
        name: "Citrine",
        extensions: &["ctr"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
