use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206284: FileFormat = FileFormat {
    id: 28_206_284,
    source_type: SourceType::Wikidata,
    name: "IBM KIPS palette",
    extensions: &["kpl", "pal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
