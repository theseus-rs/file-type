use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2750313342: FileFormat = FileFormat {
    id: 2_750_313_342,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.imd+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.imd+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
