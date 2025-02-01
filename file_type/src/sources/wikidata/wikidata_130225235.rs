use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130225235: FileFormat = FileFormat {
    id: 130_225_235,
    puid: "wikidata/130225235",
    name: "Limbo file format",
    extensions: &["b"],
    media_types: &["text/limbo"],
    internal_signatures: &[],
    related_formats: &[],
};
