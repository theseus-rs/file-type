use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206443: FileFormat = FileFormat {
    id: 28_206_443,
    puid: "wikidata/28206443",
    name: "Kt Interchange File Format",
    extensions: &["kif", "kiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
