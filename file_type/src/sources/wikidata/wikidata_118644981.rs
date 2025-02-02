use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118644981: FileFormat = FileFormat {
    id: 118_644_981,
    source_type: SourceType::Wikidata,
    name: "ISOBMFF Segment",
    extensions: &["m4s"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
