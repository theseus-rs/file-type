use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112820809: FileFormat = FileFormat {
    id: 112_820_809,
    source_type: SourceType::Wikidata,
    name: "LightWave binary object file",
    extensions: &["lw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
