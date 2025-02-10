use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_108836986: FileType = FileType {
    file_format: &FileFormat {
        id: 108_836_986,
        source_type: SourceType::Wikidata,
        name: "Nero Audio Compilation",
        extensions: &["nra"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
