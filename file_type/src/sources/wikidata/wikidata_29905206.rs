use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905206: FileFormat = FileFormat {
    id: 29_905_206,
    source_type: SourceType::Wikidata,
    name: "Self-Dissolving Archive",
    extensions: &["sda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
