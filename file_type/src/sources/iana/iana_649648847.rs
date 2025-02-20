use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_649648847: FileType = FileType {
    file_format: &FileFormat {
        id: 649_648_847,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-package.digital-signature-xmlsignature+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
