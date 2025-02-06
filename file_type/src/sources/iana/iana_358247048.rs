use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_358247048: FileFormat = FileFormat {
    id: 358_247_048,
    source_type: SourceType::Iana,
    name: "vnd.canon-cpdl",
    extensions: &[],
    media_types: &["application/vnd.canon-cpdl"],
    signatures: &[],
    related_formats: &[],
};
