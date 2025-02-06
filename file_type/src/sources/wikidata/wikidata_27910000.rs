use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27910000: FileFormat = FileFormat {
    id: 27_910_000,
    source_type: SourceType::Wikidata,
    name: "RADARSAT-2 Product Information File",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
