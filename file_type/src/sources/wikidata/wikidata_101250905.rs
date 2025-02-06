use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_101250905: FileFormat = FileFormat {
    id: 101_250_905,
    source_type: SourceType::Wikidata,
    name: ".piskel",
    extensions: &["piskel"],
    media_types: &["image/png+json"],
    signatures: &[],
    related_formats: &[],
};
