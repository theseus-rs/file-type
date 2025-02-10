use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_776095572: FileType = FileType {
    file_format: &FileFormat {
        id: 776_095_572,
        source_type: SourceType::Iana,
        name: "vc2",
        extensions: &[],
        media_types: &["video/vc2"],
        signatures: &[],
        related_formats: &[],
    },
};
