use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61766916: FileType = FileType {
    file_format: &FileFormat {
        id: 61_766_916,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Word Processor for Windows",
        extensions: &[],
        media_types: &["application/vnd.ms-works"],
        signatures: &[],
        related_formats: &[],
    },
};
