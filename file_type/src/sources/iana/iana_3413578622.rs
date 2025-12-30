use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3413578622: FileType = FileType {
    file_format: &FileFormat {
        id: 3_413_578_622,
        source_type: SourceType::Iana,
        name: "vnd.uic.tlb-fcb",
        extensions: &[],
        media_types: &["application/vnd.uic.tlb-fcb"],
        signatures: &[],
        related_formats: &[],
    },
};
