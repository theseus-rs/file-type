use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4072104703: FileType = FileType {
    file_format: &FileFormat {
        id: 4_072_104_703,
        source_type: SourceType::Iana,
        name: "vnd.cmmf-efd+xml",
        extensions: &[],
        media_types: &["application/vnd.cmmf-efd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
