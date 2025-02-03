use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28212272: FileFormat = FileFormat {
    id: 28_212_272,
    source_type: SourceType::Wikidata,
    name: "Noweb",
    extensions: &["nw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
