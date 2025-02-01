use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113577541: FileFormat = FileFormat {
    id: 113_577_541,
    puid: "wikidata/113577541",
    name: "DiscJuggler Image",
    extensions: &["cdi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
