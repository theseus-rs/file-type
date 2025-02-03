use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4126781220: FileFormat = FileFormat {
    id: 4_126_781_220,
    source_type: SourceType::Iana,
    name: "vnd.nokia.catalogs",
    extensions: &[],
    media_types: &["application/vnd.nokia.catalogs"],
    internal_signatures: &[],
    related_formats: &[],
};
