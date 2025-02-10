use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67384373: FileType = FileType {
    file_format: &FileFormat {
        id: 67_384_373,
        source_type: SourceType::Wikidata,
        name: "Crayola Art Studio graphic Art",
        extensions: &["art"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
