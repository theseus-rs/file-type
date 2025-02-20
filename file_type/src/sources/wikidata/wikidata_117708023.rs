use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117708023: FileType = FileType {
    file_format: &FileFormat {
        id: 117_708_023,
        source_type: SourceType::Wikidata,
        name: "3DHome5 Document",
        extensions: &["bld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
