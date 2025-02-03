use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2043506452: FileFormat = FileFormat {
    id: 2_043_506_452,
    source_type: SourceType::Iana,
    name: "vnd.wfa.dpp",
    extensions: &[],
    media_types: &["application/vnd.wfa.dpp"],
    internal_signatures: &[],
    related_formats: &[],
};
