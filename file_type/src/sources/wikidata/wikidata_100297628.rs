use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100297628: FileFormat = FileFormat {
    id: 100_297_628,
    source_type: SourceType::Wikidata,
    name: "Flow Charting file format, version 3",
    extensions: &["fcd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
