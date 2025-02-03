use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_680212271: FileFormat = FileFormat {
    id: 680_212_271,
    source_type: SourceType::Iana,
    name: "vnd.fujixerox.edmics-rlc",
    extensions: &[],
    media_types: &["image/vnd.fujixerox.edmics-rlc"],
    internal_signatures: &[],
    related_formats: &[],
};
