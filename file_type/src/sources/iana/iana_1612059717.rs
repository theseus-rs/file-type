use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1612059717: FileFormat = FileFormat {
    id: 1_612_059_717,
    source_type: SourceType::Iana,
    name: "vnd.dts.uhd",
    extensions: &[],
    media_types: &["audio/vnd.dts.uhd"],
    internal_signatures: &[],
    related_formats: &[],
};
