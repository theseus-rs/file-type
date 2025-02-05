use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110016184: FileFormat = FileFormat {
    id: 110_016_184,
    source_type: SourceType::Wikidata,
    name: "Archimedes Tracker Module",
    extensions: &["musx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
