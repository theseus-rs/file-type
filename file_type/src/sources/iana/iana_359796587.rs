use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_359796587: FileFormat = FileFormat {
    id: 359_796_587,
    source_type: SourceType::Iana,
    name: "vnd.noblenet-sealer",
    extensions: &[],
    media_types: &["application/vnd.noblenet-sealer"],
    internal_signatures: &[],
    related_formats: &[],
};
