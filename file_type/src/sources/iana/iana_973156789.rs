use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_973156789: FileType = FileType {
    file_format: &FileFormat {
        id: 973_156_789,
        source_type: SourceType::Iana,
        name: "vnd.Mobius.MSL",
        extensions: &[],
        media_types: &["application/vnd.Mobius.MSL"],
        signatures: &[],
        related_formats: &[],
    },
};
