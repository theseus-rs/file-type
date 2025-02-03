use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000593: FileFormat = FileFormat {
    id: 29_000_593,
    source_type: SourceType::Wikidata,
    name: "HFS Plus Journal",
    extensions: &["journal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
