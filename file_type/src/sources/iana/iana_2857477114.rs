use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2857477114: FileFormat = FileFormat {
    id: 2_857_477_114,
    source_type: SourceType::Iana,
    name: "vnd.kahootz",
    extensions: &[],
    media_types: &["application/vnd.kahootz"],
    internal_signatures: &[],
    related_formats: &[],
};
