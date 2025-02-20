use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28771300: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_300,
        source_type: SourceType::Wikidata,
        name: "Markdeep",
        extensions: &["md.html"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
