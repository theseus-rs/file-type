use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1607493878: FileFormat = FileFormat {
    id: 1_607_493_878,
    source_type: SourceType::Iana,
    name: "MPV",
    extensions: &[],
    media_types: &["video/MPV"],
    internal_signatures: &[],
    related_formats: &[],
};
