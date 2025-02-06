use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3205597145: FileFormat = FileFormat {
    id: 3_205_597_145,
    source_type: SourceType::Iana,
    name: "vnd.sosi",
    extensions: &[],
    media_types: &["text/vnd.sosi"],
    signatures: &[],
    related_formats: &[],
};
