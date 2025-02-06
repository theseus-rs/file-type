use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_311: FileFormat = FileFormat {
    id: 311,
    source_type: SourceType::Linguist,
    name: "REXX",
    extensions: &["pprx", "rex", "rexx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
