use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110086833: FileFormat = FileFormat {
    id: 110_086_833,
    source_type: SourceType::Wikidata,
    name: "Agisoft Tiled Model",
    extensions: &["tls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
