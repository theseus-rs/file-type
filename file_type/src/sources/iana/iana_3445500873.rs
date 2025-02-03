use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3445500873: FileFormat = FileFormat {
    id: 3_445_500_873,
    source_type: SourceType::Iana,
    name: "vnd.ms-tnef",
    extensions: &[],
    media_types: &["application/vnd.ms-tnef"],
    internal_signatures: &[],
    related_formats: &[],
};
