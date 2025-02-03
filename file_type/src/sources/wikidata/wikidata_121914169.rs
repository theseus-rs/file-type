use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121914169: FileFormat = FileFormat {
    id: 121_914_169,
    source_type: SourceType::Wikidata,
    name: "Memory Stick Voice File /Digital Voice File LPEC Codec",
    extensions: &["dvf", "msv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
