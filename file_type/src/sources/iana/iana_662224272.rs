use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_662224272: FileFormat = FileFormat {
    id: 662_224_272,
    source_type: SourceType::Iana,
    name: "step",
    extensions: &[],
    media_types: &["model/step"],
    internal_signatures: &[],
    related_formats: &[],
};
