use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_446836469: FileFormat = FileFormat {
    id: 446_836_469,
    source_type: SourceType::Iana,
    name: "vnd.wap.wbxml",
    extensions: &[],
    media_types: &["application/vnd.wap.wbxml"],
    signatures: &[],
    related_formats: &[],
};
