use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3483875492: FileType = FileType {
    file_format: &FileFormat {
        id: 3_483_875_492,
        source_type: SourceType::Iana,
        name: "vnd.fujitsu.oasys2",
        extensions: &[],
        media_types: &["application/vnd.fujitsu.oasys2"],
        signatures: &[],
        related_formats: &[],
    },
};
