use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112820809: FileFormat = FileFormat {
    id: 112_820_809,
    source_type: SourceType::Wikidata,
    name: "LightWave binary object file",
    extensions: &["lw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
