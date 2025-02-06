use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3281459987: FileFormat = FileFormat {
    id: 3_281_459_987,
    source_type: SourceType::Iana,
    name: "ief",
    extensions: &[],
    media_types: &["image/ief"],
    signatures: &[],
    related_formats: &[],
};
