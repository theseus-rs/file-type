use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_664677117: FileFormat = FileFormat {
    id: 664_677_117,
    source_type: SourceType::Iana,
    name: "vnd.nokia.radio-preset",
    extensions: &[],
    media_types: &["application/vnd.nokia.radio-preset"],
    internal_signatures: &[],
    related_formats: &[],
};
