use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_703420198: FileFormat = FileFormat {
    id: 703_420_198,
    source_type: SourceType::Iana,
    name: "vnd.autopackage",
    extensions: &[],
    media_types: &["application/vnd.autopackage"],
    internal_signatures: &[],
    related_formats: &[],
};
