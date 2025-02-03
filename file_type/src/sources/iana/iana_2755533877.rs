use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2755533877: FileFormat = FileFormat {
    id: 2_755_533_877,
    source_type: SourceType::Iana,
    name: "vnd.oipf.userprofile+xml",
    extensions: &[],
    media_types: &["application/vnd.oipf.userprofile+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
