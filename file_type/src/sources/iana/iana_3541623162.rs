use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3541623162: FileFormat = FileFormat {
    id: 3_541_623_162,
    source_type: SourceType::Iana,
    name: "mathematica",
    extensions: &[],
    media_types: &["application/mathematica"],
    signatures: &[],
    related_formats: &[],
};
