use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206284: FileFormat = FileFormat {
    id: 28_206_284,
    puid: "wikidata/28206284",
    name: "IBM KIPS palette",
    extensions: &["kpl", "pal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
