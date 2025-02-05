use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_99973606: FileFormat = FileFormat {
    id: 99_973_606,
    source_type: SourceType::Wikidata,
    name: "XDOMEA 2.0.0",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
