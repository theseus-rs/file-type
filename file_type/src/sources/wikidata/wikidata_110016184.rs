use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110016184: FileFormat = FileFormat {
    id: 110_016_184,
    source_type: SourceType::Wikidata,
    name: "Archimedes Tracker Module",
    extensions: &["musx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
