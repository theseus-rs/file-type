use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757910: FileFormat = FileFormat {
    id: 28_757_910,
    puid: "wikidata/28757910",
    name: "Google Document",
    extensions: &["gdoc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
