use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3671283767: FileFormat = FileFormat {
    id: 3_671_283_767,
    source_type: SourceType::Iana,
    name: "form-data",
    extensions: &[],
    media_types: &["multipart/form-data"],
    signatures: &[],
    related_formats: &[],
};
