use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3818381341: FileFormat = FileFormat {
    id: 3_818_381_341,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-package.core-properties+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-package.core-properties+xml"],
    signatures: &[],
    related_formats: &[],
};
