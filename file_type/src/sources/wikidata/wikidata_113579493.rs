use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113579493: FileFormat = FileFormat {
    id: 113_579_493,
    puid: "wikidata/113579493",
    name: "Justfile",
    extensions: &["just", "justfile"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
