use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206073: FileFormat = FileFormat {
    id: 28_206_073,
    source_type: SourceType::Wikidata,
    name: "Fuckpaint PI4",
    extensions: &["PI4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
