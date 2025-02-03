use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2581876382: FileFormat = FileFormat {
    id: 2_581_876_382,
    source_type: SourceType::Iana,
    name: "cms",
    extensions: &[],
    media_types: &["application/cms"],
    internal_signatures: &[],
    related_formats: &[],
};
