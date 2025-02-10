use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52834540: FileType = FileType {
    file_format: &FileFormat {
        id: 52_834_540,
        source_type: SourceType::Wikidata,
        name: "Paint Shop Pro Image, version 4",
        extensions: &["psp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
