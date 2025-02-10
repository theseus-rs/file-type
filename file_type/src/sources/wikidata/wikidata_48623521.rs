use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48623521: FileType = FileType {
    file_format: &FileFormat {
        id: 48_623_521,
        source_type: SourceType::Wikidata,
        name: "Paint Shop Pro Image, version 3",
        extensions: &["psp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
