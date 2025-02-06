use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_121497109: FileFormat = FileFormat {
    id: 121_497_109,
    source_type: SourceType::Iana,
    name: "vnd.piaccess.application-licence",
    extensions: &[],
    media_types: &["application/vnd.piaccess.application-licence"],
    signatures: &[],
    related_formats: &[],
};
