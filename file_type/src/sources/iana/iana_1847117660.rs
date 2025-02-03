use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1847117660: FileFormat = FileFormat {
    id: 1_847_117_660,
    source_type: SourceType::Iana,
    name: "vp",
    extensions: &[],
    media_types: &["application/vp"],
    internal_signatures: &[],
    related_formats: &[],
};
