use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3477608633: FileFormat = FileFormat {
    id: 3_477_608_633,
    source_type: SourceType::Iana,
    name: "vnd.radgamettools.smacker",
    extensions: &[],
    media_types: &["video/vnd.radgamettools.smacker"],
    internal_signatures: &[],
    related_formats: &[],
};
