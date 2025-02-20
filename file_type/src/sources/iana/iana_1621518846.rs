use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1621518846: FileType = FileType {
    file_format: &FileFormat {
        id: 1_621_518_846,
        source_type: SourceType::Iana,
        name: "ulpfec",
        extensions: &[],
        media_types: &["application/ulpfec"],
        signatures: &[],
        related_formats: &[],
    },
};
