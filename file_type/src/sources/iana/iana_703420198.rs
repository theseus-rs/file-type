use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_703420198: FileType = FileType {
    file_format: &FileFormat {
        id: 703_420_198,
        source_type: SourceType::Iana,
        name: "vnd.autopackage",
        extensions: &[],
        media_types: &["application/vnd.autopackage"],
        signatures: &[],
        related_formats: &[],
    },
};
