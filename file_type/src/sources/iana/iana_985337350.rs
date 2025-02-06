use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_985337350: FileFormat = FileFormat {
    id: 985_337_350,
    source_type: SourceType::Iana,
    name: "vnd.loom",
    extensions: &[],
    media_types: &["application/vnd.loom"],
    signatures: &[],
    related_formats: &[],
};
