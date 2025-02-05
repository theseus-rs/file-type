use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118644981: FileFormat = FileFormat {
    id: 118_644_981,
    source_type: SourceType::Wikidata,
    name: "ISOBMFF Segment",
    extensions: &["m4s"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
