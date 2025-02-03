use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5533911: FileFormat = FileFormat {
    id: 5_533_911,
    source_type: SourceType::Wikidata,
    name: "GeoPDF",
    extensions: &["pdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
