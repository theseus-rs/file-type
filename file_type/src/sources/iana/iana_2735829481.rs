use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2735829481: FileFormat = FileFormat {
    id: 2_735_829_481,
    source_type: SourceType::Iana,
    name: "quicktime",
    extensions: &[],
    media_types: &["video/quicktime"],
    internal_signatures: &[],
    related_formats: &[],
};
