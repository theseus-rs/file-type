use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2775293850: FileFormat = FileFormat {
    id: 2_775_293_850,
    source_type: SourceType::Iana,
    name: "troff",
    extensions: &[],
    media_types: &["text/troff"],
    internal_signatures: &[],
    related_formats: &[],
};
