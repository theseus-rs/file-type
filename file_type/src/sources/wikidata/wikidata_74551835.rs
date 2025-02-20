use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_74551835: FileType = FileType {
    file_format: &FileFormat {
        id: 74_551_835,
        source_type: SourceType::Wikidata,
        name: "ChiWriter Screen Font",
        extensions: &["sft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
