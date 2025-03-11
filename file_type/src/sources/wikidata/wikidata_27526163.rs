use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27526163: FileType = FileType {
    file_format: &FileFormat {
        id: 27_526_163,
        source_type: SourceType::Wikidata,
        name: "Microsoft Word for Macintosh Document, version 6.0",
        extensions: &[],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[],
    },
};
