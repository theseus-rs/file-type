use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3270654971: FileType = FileType {
    file_format: &FileFormat {
        id: 3_270_654_971,
        source_type: SourceType::Iana,
        name: "vnd.bpf3",
        extensions: &[],
        media_types: &["application/vnd.bpf3"],
        signatures: &[],
        related_formats: &[],
    },
};
