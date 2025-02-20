use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29943965: FileType = FileType {
    file_format: &FileFormat {
        id: 29_943_965,
        source_type: SourceType::Wikidata,
        name: "Statistical Package for the Social Sciences portable data format",
        extensions: &["por"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
