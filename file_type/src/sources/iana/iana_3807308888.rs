use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3807308888: FileType = FileType {
    file_format: &FileFormat {
        id: 3_807_308_888,
        source_type: SourceType::Iana,
        name: "vnd.aumtrix.aum",
        extensions: &[],
        media_types: &["application/vnd.aumtrix.aum"],
        signatures: &[],
        related_formats: &[],
    },
};
