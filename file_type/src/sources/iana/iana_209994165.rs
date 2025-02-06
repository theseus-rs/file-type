use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_209994165: FileFormat = FileFormat {
    id: 209_994_165,
    source_type: SourceType::Iana,
    name: "set-registration-initiation",
    extensions: &[],
    media_types: &["application/set-registration-initiation"],
    signatures: &[],
    related_formats: &[],
};
