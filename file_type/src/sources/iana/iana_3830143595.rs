use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3830143595: FileFormat = FileFormat {
    id: 3_830_143_595,
    source_type: SourceType::Iana,
    name: "yaml",
    extensions: &[],
    media_types: &["application/yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
