use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2696839422: FileFormat = FileFormat {
    id: 2_696_839_422,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"],
    internal_signatures: &[],
    related_formats: &[],
};
