use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111166091: FileType = FileType {
    file_format: &FileFormat {
        id: 111_166_091,
        source_type: SourceType::Wikidata,
        name: "Ludwig song file",
        extensions: &["ludwig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
