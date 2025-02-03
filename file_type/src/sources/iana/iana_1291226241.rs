use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1291226241: FileFormat = FileFormat {
    id: 1_291_226_241,
    source_type: SourceType::Iana,
    name: "vnd.onepager",
    extensions: &[],
    media_types: &["application/vnd.onepager"],
    internal_signatures: &[],
    related_formats: &[],
};
