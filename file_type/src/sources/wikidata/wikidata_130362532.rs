use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130362532: FileFormat = FileFormat {
    id: 130_362_532,
    source_type: SourceType::Wikidata,
    name: "MuPAD file format",
    extensions: &["mu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
