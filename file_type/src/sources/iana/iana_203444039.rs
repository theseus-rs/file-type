use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_203444039: FileFormat = FileFormat {
    id: 203_444_039,
    source_type: SourceType::Iana,
    name: "vnd.panoply",
    extensions: &[],
    media_types: &["application/vnd.panoply"],
    signatures: &[],
    related_formats: &[],
};
