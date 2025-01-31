use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112653466: FileFormat = FileFormat {
    id: 112_653_466,
    puid: "wikidata/112653466",
    name: "Professional Draw 1.0 file",
    extensions: &["pdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
