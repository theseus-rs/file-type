use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1888920778: FileFormat = FileFormat {
    id: 1_888_920_778,
    source_type: SourceType::Iana,
    name: "evc",
    extensions: &[],
    media_types: &["video/evc"],
    internal_signatures: &[],
    related_formats: &[],
};
