use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3841211511: FileFormat = FileFormat {
    id: 3_841_211_511,
    source_type: SourceType::Iana,
    name: "dvcs",
    extensions: &[],
    media_types: &["application/dvcs"],
    internal_signatures: &[],
    related_formats: &[],
};
