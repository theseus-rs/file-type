use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121914169: FileFormat = FileFormat {
    id: 121_914_169,
    source_type: SourceType::Wikidata,
    name: "Memory Stick Voice File /Digital Voice File LPEC Codec",
    extensions: &["dvf", "msv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
