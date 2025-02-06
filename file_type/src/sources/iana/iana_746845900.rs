use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_746845900: FileFormat = FileFormat {
    id: 746_845_900,
    source_type: SourceType::Iana,
    name: "vnd.canon-lips",
    extensions: &[],
    media_types: &["application/vnd.canon-lips"],
    signatures: &[],
    related_formats: &[],
};
