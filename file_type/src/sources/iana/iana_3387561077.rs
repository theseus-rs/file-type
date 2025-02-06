use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3387561077: FileFormat = FileFormat {
    id: 3_387_561_077,
    source_type: SourceType::Iana,
    name: "vnd.wqd",
    extensions: &[],
    media_types: &["application/vnd.wqd"],
    signatures: &[],
    related_formats: &[],
};
