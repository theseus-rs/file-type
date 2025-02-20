use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28445591: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_591,
        source_type: SourceType::Wikidata,
        name: "AMOS BASIC tokenized file",
        extensions: &["amos"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
