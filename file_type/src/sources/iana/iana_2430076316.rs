use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2430076316: FileType = FileType {
    file_format: &FileFormat {
        id: 2_430_076_316,
        source_type: SourceType::Iana,
        name: "vnd.fujitsu.oasys",
        extensions: &[],
        media_types: &["application/vnd.fujitsu.oasys"],
        signatures: &[],
        related_formats: &[],
    },
};
