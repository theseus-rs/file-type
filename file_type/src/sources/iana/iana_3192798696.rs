use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3192798696: FileFormat = FileFormat {
    id: 3_192_798_696,
    source_type: SourceType::Iana,
    name: "vnd.msign",
    extensions: &[],
    media_types: &["application/vnd.msign"],
    internal_signatures: &[],
    related_formats: &[],
};
