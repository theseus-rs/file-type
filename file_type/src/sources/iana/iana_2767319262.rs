use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2767319262: FileFormat = FileFormat {
    id: 2_767_319_262,
    source_type: SourceType::Iana,
    name: "vnd.ms-asf",
    extensions: &[],
    media_types: &["application/vnd.ms-asf"],
    signatures: &[],
    related_formats: &[],
};
