use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110519164: FileFormat = FileFormat {
    id: 110_519_164,
    source_type: SourceType::Wikidata,
    name: "ISDOCX Information System Document, version 6.0.1",
    extensions: &["isdocx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
