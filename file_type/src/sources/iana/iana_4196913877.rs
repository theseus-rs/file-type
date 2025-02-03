use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4196913877: FileFormat = FileFormat {
    id: 4_196_913_877,
    source_type: SourceType::Iana,
    name: "vnd.osgi.bundle",
    extensions: &[],
    media_types: &["application/vnd.osgi.bundle"],
    internal_signatures: &[],
    related_formats: &[],
};
