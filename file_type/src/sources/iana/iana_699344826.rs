use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_699344826: FileFormat = FileFormat {
    id: 699_344_826,
    source_type: SourceType::Iana,
    name: "vnd.Quark.QuarkXPress",
    extensions: &[],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    signatures: &[],
    related_formats: &[],
};
