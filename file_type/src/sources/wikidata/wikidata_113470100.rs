use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113470100: FileType = FileType {
    file_format: &FileFormat {
        id: 113_470_100,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word for MS-DOS Document, version 6.0",
        extensions: &["doc"],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[],
    },
};
