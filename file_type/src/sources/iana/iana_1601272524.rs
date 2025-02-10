use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1601272524: FileType = FileType {
    file_format: &FileFormat {
        id: 1_601_272_524,
        source_type: SourceType::Iana,
        name: "vnd.sbm.mid2",
        extensions: &[],
        media_types: &["application/vnd.sbm.mid2"],
        signatures: &[],
        related_formats: &[],
    },
};
