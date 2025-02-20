use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61639409: FileType = FileType {
    file_format: &FileFormat {
        id: 61_639_409,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word for Windows Document, version 1",
        extensions: &["doc"],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[],
    },
};
