use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1863426116: FileType = FileType {
    file_format: &FileFormat {
        id: 1_863_426_116,
        source_type: SourceType::Iana,
        name: "vnd.oma.dcd",
        extensions: &[],
        media_types: &["application/vnd.oma.dcd"],
        signatures: &[],
        related_formats: &[],
    },
};
