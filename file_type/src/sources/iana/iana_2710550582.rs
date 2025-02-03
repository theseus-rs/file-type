use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2710550582: FileFormat = FileFormat {
    id: 2_710_550_582,
    source_type: SourceType::Iana,
    name: "vnd.nokia.interleaved-multimedia",
    extensions: &[],
    media_types: &["video/vnd.nokia.interleaved-multimedia"],
    internal_signatures: &[],
    related_formats: &[],
};
