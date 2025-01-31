use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118644981: FileFormat = FileFormat {
    id: 118_644_981,
    puid: "wikidata/118644981",
    name: "ISOBMFF Segment",
    extensions: &["m4s"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
