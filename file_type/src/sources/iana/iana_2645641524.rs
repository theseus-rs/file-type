use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2645641524: FileFormat = FileFormat {
    id: 2_645_641_524,
    source_type: SourceType::Iana,
    name: "mxf",
    extensions: &[],
    media_types: &["application/mxf"],
    internal_signatures: &[],
    related_formats: &[],
};
