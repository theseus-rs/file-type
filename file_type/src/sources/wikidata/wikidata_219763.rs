use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_219763: FileFormat = FileFormat {
    id: 219_763,
    source_type: SourceType::Wikidata,
    name: "MPEG-4",
    extensions: &["mp4"],
    media_types: &["video/mp4"],
    signatures: &[],
    related_formats: &[],
};
