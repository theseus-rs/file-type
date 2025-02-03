use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1690715: FileFormat = FileFormat {
    id: 1_690_715,
    source_type: SourceType::Iana,
    name: "vnd.iccprofile",
    extensions: &[],
    media_types: &["application/vnd.iccprofile"],
    internal_signatures: &[],
    related_formats: &[],
};
