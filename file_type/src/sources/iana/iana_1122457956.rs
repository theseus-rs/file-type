use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1122457956: FileFormat = FileFormat {
    id: 1_122_457_956,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.seal-mbms-usage-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.seal-mbms-usage-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
