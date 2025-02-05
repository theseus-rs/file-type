use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3129244361: FileFormat = FileFormat {
    id: 3_129_244_361,
    source_type: SourceType::Iana,
    name: "vnd.etsi.mheg5",
    extensions: &[],
    media_types: &["application/vnd.etsi.mheg5"],
    signatures: &[],
    related_formats: &[],
};
