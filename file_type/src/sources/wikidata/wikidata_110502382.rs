use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110502382: FileFormat = FileFormat {
    id: 110_502_382,
    source_type: SourceType::Wikidata,
    name: "ISDOC Information System Document (Generic)",
    extensions: &["isdoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
