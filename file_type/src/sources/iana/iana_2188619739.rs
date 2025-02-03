use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2188619739: FileFormat = FileFormat {
    id: 2_188_619_739,
    source_type: SourceType::Iana,
    name: "vnd.noblenet-web",
    extensions: &[],
    media_types: &["application/vnd.noblenet-web"],
    internal_signatures: &[],
    related_formats: &[],
};
