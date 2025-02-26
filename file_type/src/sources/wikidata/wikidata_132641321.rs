use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132641321: FileType = FileType {
    file_format: &FileFormat {
        id: 132_641_321,
        source_type: SourceType::Wikidata,
        name: "Package source file",
        extensions: &["pks"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
