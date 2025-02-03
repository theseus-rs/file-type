use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_84177950: FileFormat = FileFormat {
    id: 84_177_950,
    source_type: SourceType::Iana,
    name: "vnd.uplanet.cacheop-wbxml",
    extensions: &[],
    media_types: &["application/vnd.uplanet.cacheop-wbxml"],
    internal_signatures: &[],
    related_formats: &[],
};
