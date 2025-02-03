use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206218: FileFormat = FileFormat {
    id: 28_206_218,
    source_type: SourceType::Wikidata,
    name: "GRF",
    extensions: &["grf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
