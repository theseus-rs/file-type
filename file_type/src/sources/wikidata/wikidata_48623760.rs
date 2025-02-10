use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48623760: FileType = FileType {
    file_format: &FileFormat {
        id: 48_623_760,
        source_type: SourceType::Wikidata,
        name: "Paint Shop Pro Image, version 5",
        extensions: &["psp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
