use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111430980: FileType = FileType {
    file_format: &FileFormat {
        id: 111_430_980,
        source_type: SourceType::Wikidata,
        name: "ExtendScript Script File",
        extensions: &["jxs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
