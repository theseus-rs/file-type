use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2627890512: FileFormat = FileFormat {
    id: 2_627_890_512,
    source_type: SourceType::Iana,
    name: "vnd.smart.notebook",
    extensions: &[],
    media_types: &["application/vnd.smart.notebook"],
    signatures: &[],
    related_formats: &[],
};
