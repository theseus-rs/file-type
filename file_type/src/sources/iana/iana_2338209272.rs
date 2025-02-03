use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2338209272: FileFormat = FileFormat {
    id: 2_338_209_272,
    source_type: SourceType::Iana,
    name: "G722",
    extensions: &[],
    media_types: &["audio/G722"],
    internal_signatures: &[],
    related_formats: &[],
};
