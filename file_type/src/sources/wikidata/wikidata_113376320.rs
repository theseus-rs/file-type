use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113376320: FileType = FileType {
    file_format: &FileFormat {
        id: 113_376_320,
        source_type: SourceType::Wikidata,
        name: "XL-Paint format",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
