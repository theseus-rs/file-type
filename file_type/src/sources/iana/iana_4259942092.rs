use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4259942092: FileType = FileType {
    file_format: &FileFormat {
        id: 4_259_942_092,
        source_type: SourceType::Iana,
        name: "vnd.gp3",
        extensions: &[],
        media_types: &["application/vnd.gp3"],
        signatures: &[],
        related_formats: &[],
    },
};
