use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206300: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_300,
        source_type: SourceType::Wikidata,
        name: "Image Exchange Format",
        extensions: &[],
        media_types: &["image/ief"],
        signatures: &[],
        related_formats: &[],
    },
};
