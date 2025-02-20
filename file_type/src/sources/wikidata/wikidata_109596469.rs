use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109596469: FileType = FileType {
    file_format: &FileFormat {
        id: 109_596_469,
        source_type: SourceType::Wikidata,
        name: "DrawPlus Template",
        extensions: &["dpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
