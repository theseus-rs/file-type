use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206284: FileFormat = FileFormat {
    id: 28_206_284,
    source_type: SourceType::Wikidata,
    name: "IBM KIPS palette",
    extensions: &["kpl", "pal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
