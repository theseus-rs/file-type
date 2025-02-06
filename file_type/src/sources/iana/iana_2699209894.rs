use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2699209894: FileFormat = FileFormat {
    id: 2_699_209_894,
    source_type: SourceType::Iana,
    name: "vnd.powerbuilder7",
    extensions: &[],
    media_types: &["application/vnd.powerbuilder7"],
    signatures: &[],
    related_formats: &[],
};
