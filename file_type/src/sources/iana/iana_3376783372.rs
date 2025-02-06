use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3376783372: FileFormat = FileFormat {
    id: 3_376_783_372,
    source_type: SourceType::Iana,
    name: "vnd.digital-winds",
    extensions: &[],
    media_types: &["audio/vnd.digital-winds"],
    signatures: &[],
    related_formats: &[],
};
