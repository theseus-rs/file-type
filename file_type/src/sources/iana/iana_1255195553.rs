use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1255195553: FileFormat = FileFormat {
    id: 1_255_195_553,
    source_type: SourceType::Iana,
    name: "3gpp",
    extensions: &[],
    media_types: &["video/3gpp"],
    internal_signatures: &[],
    related_formats: &[],
};
