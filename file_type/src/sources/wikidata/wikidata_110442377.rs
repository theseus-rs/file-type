use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110442377: FileType = FileType {
    file_format: &FileFormat {
        id: 110_442_377,
        source_type: SourceType::Wikidata,
        name: "Daisy-Dot Font File, version III",
        extensions: &["nlq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
