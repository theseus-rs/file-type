use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3530494109: FileFormat = FileFormat {
    id: 3_530_494_109,
    source_type: SourceType::Iana,
    name: "vnd.uplanet.cacheop",
    extensions: &[],
    media_types: &["application/vnd.uplanet.cacheop"],
    signatures: &[],
    related_formats: &[],
};
