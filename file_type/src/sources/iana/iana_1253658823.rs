use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1253658823: FileFormat = FileFormat {
    id: 1_253_658_823,
    source_type: SourceType::Iana,
    name: "vnd.evolv.ecig.theme",
    extensions: &[],
    media_types: &["application/vnd.evolv.ecig.theme"],
    signatures: &[],
    related_formats: &[],
};
