use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3083798485: FileFormat = FileFormat {
    id: 3_083_798_485,
    source_type: SourceType::Iana,
    name: "vnd.ms-excel.template.macroEnabled.12",
    extensions: &[],
    media_types: &["application/vnd.ms-excel.template.macroEnabled.12"],
    internal_signatures: &[],
    related_formats: &[],
};
