use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100323905: FileFormat = FileFormat {
    id: 100_323_905,
    source_type: SourceType::Wikidata,
    name: "PFS:Write Document",
    extensions: &["pfs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
