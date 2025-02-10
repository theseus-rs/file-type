use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116859776: FileType = FileType {
    file_format: &FileFormat {
        id: 116_859_776,
        source_type: SourceType::Wikidata,
        name: "Quicken Payee List",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
