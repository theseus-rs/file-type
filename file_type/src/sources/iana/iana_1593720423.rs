use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1593720423: FileFormat = FileFormat {
    id: 1_593_720_423,
    source_type: SourceType::Iana,
    name: "vnd.koan",
    extensions: &[],
    media_types: &["application/vnd.koan"],
    internal_signatures: &[],
    related_formats: &[],
};
