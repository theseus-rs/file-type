use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_668848572: FileType = FileType {
    file_format: &FileFormat {
        id: 668_848_572,
        source_type: SourceType::Iana,
        name: "vnd.fujitsu.oasys3",
        extensions: &[],
        media_types: &["application/vnd.fujitsu.oasys3"],
        signatures: &[],
        related_formats: &[],
    },
};
