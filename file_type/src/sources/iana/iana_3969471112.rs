use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3969471112: FileFormat = FileFormat {
    id: 3_969_471_112,
    source_type: SourceType::Iana,
    name: "vnd.oma.cab-user-prefs+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.cab-user-prefs+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
