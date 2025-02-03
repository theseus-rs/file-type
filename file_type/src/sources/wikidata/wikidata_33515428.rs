use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_33515428: FileFormat = FileFormat {
    id: 33_515_428,
    source_type: SourceType::Wikidata,
    name: "LAS 1.2 file format",
    extensions: &["las", "laz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
