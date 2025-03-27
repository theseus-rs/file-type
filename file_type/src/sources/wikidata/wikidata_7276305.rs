use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7276305: FileType = FileType {
    file_format: &FileFormat {
        id: 7_276_305,
        source_type: SourceType::Wikidata,
        name: "RED digital pictures",
        extensions: &["r3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
