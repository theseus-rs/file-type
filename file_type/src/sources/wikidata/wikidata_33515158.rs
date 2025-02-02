use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_33515158: FileFormat = FileFormat {
    id: 33_515_158,
    source_type: SourceType::Wikidata,
    name: "LAS 1.1",
    extensions: &["las", "laz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
