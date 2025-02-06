use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2489989862: FileFormat = FileFormat {
    id: 2_489_989_862,
    source_type: SourceType::Iana,
    name: "vnd.ms-excel.sheet.macroEnabled.12",
    extensions: &[],
    media_types: &["application/vnd.ms-excel.sheet.macroEnabled.12"],
    signatures: &[],
    related_formats: &[],
};
