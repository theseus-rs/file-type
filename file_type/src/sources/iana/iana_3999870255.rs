use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3999870255: FileFormat = FileFormat {
    id: 3_999_870_255,
    source_type: SourceType::Iana,
    name: "vnd.mophun.application",
    extensions: &[],
    media_types: &["application/vnd.mophun.application"],
    internal_signatures: &[],
    related_formats: &[],
};
