use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206198: FileFormat = FileFormat {
    id: 28_206_198,
    puid: "wikidata/28206198",
    name: "GodPaint",
    extensions: &["god"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
