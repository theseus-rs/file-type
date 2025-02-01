use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206171: FileFormat = FileFormat {
    id: 28_206_171,
    puid: "wikidata/28206171",
    name: "GIMP Animated Brush",
    extensions: &["gih"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
