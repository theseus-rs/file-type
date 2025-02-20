use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_592837311: FileType = FileType {
    file_format: &FileFormat {
        id: 592_837_311,
        source_type: SourceType::Iana,
        name: "simple-filter+xml",
        extensions: &[],
        media_types: &["application/simple-filter+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
