use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1250383: FileType = FileType {
    file_format: &FileFormat {
        id: 1_250_383,
        source_type: SourceType::Wikidata,
        name: "XYZ file format",
        extensions: &["xyz"],
        media_types: &["chemical/x-xyz"],
        signatures: &[],
        related_formats: &[],
    },
};
