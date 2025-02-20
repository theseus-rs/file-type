use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_351975492: FileType = FileType {
    file_format: &FileFormat {
        id: 351_975_492,
        source_type: SourceType::Iana,
        name: "vnd.hdt",
        extensions: &[],
        media_types: &["application/vnd.hdt"],
        signatures: &[],
        related_formats: &[],
    },
};
