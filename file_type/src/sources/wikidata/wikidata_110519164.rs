use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110519164: FileFormat = FileFormat {
    id: 110_519_164,
    source_type: SourceType::Wikidata,
    name: "ISDOCX Information System Document, version 6.0.1",
    extensions: &["isdocx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
