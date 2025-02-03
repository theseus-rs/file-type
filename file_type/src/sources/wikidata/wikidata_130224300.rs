use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130224300: FileFormat = FileFormat {
    id: 130_224_300,
    source_type: SourceType::Wikidata,
    name: "Lean 4 file format",
    extensions: &["lean"],
    media_types: &["text/x-lean4"],
    internal_signatures: &[],
    related_formats: &[],
};
