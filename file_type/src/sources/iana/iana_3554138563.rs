use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3554138563: FileFormat = FileFormat {
    id: 3_554_138_563,
    source_type: SourceType::Iana,
    name: "vnd.meridian-slingshot",
    extensions: &[],
    media_types: &["application/vnd.meridian-slingshot"],
    internal_signatures: &[],
    related_formats: &[],
};
