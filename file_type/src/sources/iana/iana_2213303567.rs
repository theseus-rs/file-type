use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2213303567: FileFormat = FileFormat {
    id: 2_213_303_567,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.template",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.template"],
    internal_signatures: &[],
    related_formats: &[],
};
