use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122170514: FileFormat = FileFormat {
    id: 122_170_514,
    source_type: SourceType::Wikidata,
    name: "WireGuard profile",
    extensions: &["conf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
