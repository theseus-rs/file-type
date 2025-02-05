use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1253575532: FileFormat = FileFormat {
    id: 1_253_575_532,
    source_type: SourceType::Iana,
    name: "vnd.fujixerox.HBPL",
    extensions: &[],
    media_types: &["application/vnd.fujixerox.HBPL"],
    signatures: &[],
    related_formats: &[],
};
