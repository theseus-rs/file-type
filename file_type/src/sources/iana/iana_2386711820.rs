use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2386711820: FileFormat = FileFormat {
    id: 2_386_711_820,
    source_type: SourceType::Iana,
    name: "vnd.citationstyles.style+xml",
    extensions: &[],
    media_types: &["application/vnd.citationstyles.style+xml"],
    signatures: &[],
    related_formats: &[],
};
