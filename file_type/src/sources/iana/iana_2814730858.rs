use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2814730858: FileFormat = FileFormat {
    id: 2_814_730_858,
    source_type: SourceType::Iana,
    name: "jxrA",
    extensions: &[],
    media_types: &["image/jxrA"],
    signatures: &[],
    related_formats: &[],
};
