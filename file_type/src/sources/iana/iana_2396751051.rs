use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2396751051: FileType = FileType {
    file_format: &FileFormat {
        id: 2_396_751_051,
        source_type: SourceType::Iana,
        name: "smpte336m",
        extensions: &[],
        media_types: &["application/smpte336m"],
        signatures: &[],
        related_formats: &[],
    },
};
