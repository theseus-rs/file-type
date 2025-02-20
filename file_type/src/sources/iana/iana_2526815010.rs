use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2526815010: FileType = FileType {
    file_format: &FileFormat {
        id: 2_526_815_010,
        source_type: SourceType::Iana,
        name: "vnd.Mobius.PLC",
        extensions: &[],
        media_types: &["application/vnd.Mobius.PLC"],
        signatures: &[],
        related_formats: &[],
    },
};
