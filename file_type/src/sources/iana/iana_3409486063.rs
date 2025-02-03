use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3409486063: FileFormat = FileFormat {
    id: 3_409_486_063,
    source_type: SourceType::Iana,
    name: "vnd.dcmp+xml",
    extensions: &[],
    media_types: &["application/vnd.dcmp+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
