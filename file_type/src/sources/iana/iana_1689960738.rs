use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1689960738: FileFormat = FileFormat {
    id: 1_689_960_738,
    source_type: SourceType::Iana,
    name: "vnd.bbf.usp.msg+json",
    extensions: &[],
    media_types: &["application/vnd.bbf.usp.msg+json"],
    signatures: &[],
    related_formats: &[],
};
