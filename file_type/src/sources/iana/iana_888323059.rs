use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_888323059: FileFormat = FileFormat {
    id: 888_323_059,
    source_type: SourceType::Iana,
    name: "vnd.seis+json",
    extensions: &[],
    media_types: &["application/vnd.seis+json"],
    internal_signatures: &[],
    related_formats: &[],
};
