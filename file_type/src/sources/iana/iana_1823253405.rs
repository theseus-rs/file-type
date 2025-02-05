use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1823253405: FileFormat = FileFormat {
    id: 1_823_253_405,
    source_type: SourceType::Iana,
    name: "vnd.orange.indata",
    extensions: &[],
    media_types: &["application/vnd.orange.indata"],
    signatures: &[],
    related_formats: &[],
};
