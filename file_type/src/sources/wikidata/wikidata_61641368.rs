use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61641368: FileType = FileType {
    file_format: &FileFormat {
        id: 61_641_368,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word for Windows Document, version 2",
        extensions: &["doc"],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[],
    },
};
