use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113579493: FileFormat = FileFormat {
    id: 113_579_493,
    source_type: SourceType::Wikidata,
    name: "Justfile",
    extensions: &["just", "justfile"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
