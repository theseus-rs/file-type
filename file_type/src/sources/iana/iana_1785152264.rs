use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1785152264: FileFormat = FileFormat {
    id: 1_785_152_264,
    source_type: SourceType::Iana,
    name: "vnd.ctc-posml",
    extensions: &[],
    media_types: &["application/vnd.ctc-posml"],
    internal_signatures: &[],
    related_formats: &[],
};
