use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67206683: FileType = FileType {
    file_format: &FileFormat {
        id: 67_206_683,
        source_type: SourceType::Wikidata,
        name: "VRML Worlds",
        extensions: &["3dv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
