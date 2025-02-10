use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2022828087: FileType = FileType {
    file_format: &FileFormat {
        id: 2_022_828_087,
        source_type: SourceType::Iana,
        name: "vnd.framemaker",
        extensions: &[],
        media_types: &["application/vnd.framemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
