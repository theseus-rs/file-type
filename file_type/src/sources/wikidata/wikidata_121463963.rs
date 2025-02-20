use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121463963: FileType = FileType {
    file_format: &FileFormat {
        id: 121_463_963,
        source_type: SourceType::Wikidata,
        name: "Adobe Lightroom Library",
        extensions: &["aglib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
