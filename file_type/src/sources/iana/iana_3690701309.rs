use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3690701309: FileFormat = FileFormat {
    id: 3_690_701_309,
    source_type: SourceType::Iana,
    name: "vnd.ms-mediapackage",
    extensions: &[],
    media_types: &["text/vnd.ms-mediapackage"],
    signatures: &[],
    related_formats: &[],
};
