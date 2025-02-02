use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110502531: FileFormat = FileFormat {
    id: 110_502_531,
    source_type: SourceType::Wikidata,
    name: "ISDOCX Information System Document (Generic)",
    extensions: &["isdocx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
