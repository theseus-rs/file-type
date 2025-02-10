use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_209994165: FileType = FileType {
    file_format: &FileFormat {
        id: 209_994_165,
        source_type: SourceType::Iana,
        name: "set-registration-initiation",
        extensions: &[],
        media_types: &["application/set-registration-initiation"],
        signatures: &[],
        related_formats: &[],
    },
};
