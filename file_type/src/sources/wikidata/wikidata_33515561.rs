use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_33515561: FileFormat = FileFormat {
    id: 33_515_561,
    source_type: SourceType::Wikidata,
    name: "LAS 1.3 file format",
    extensions: &["las", "laz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
