use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111341736: FileType = FileType {
    file_format: &FileFormat {
        id: 111_341_736,
        source_type: SourceType::Wikidata,
        name: "Sound Designer II flattened file",
        extensions: &["sd2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
