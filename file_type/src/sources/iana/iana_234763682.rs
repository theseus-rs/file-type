use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_234763682: FileFormat = FileFormat {
    id: 234_763_682,
    source_type: SourceType::Iana,
    name: "vnd.balsamiq.bmpr",
    extensions: &[],
    media_types: &["application/vnd.balsamiq.bmpr"],
    signatures: &[],
    related_formats: &[],
};
