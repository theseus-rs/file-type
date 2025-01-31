use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4652973: FileFormat = FileFormat {
    id: 4_652_973,
    puid: "wikidata/4652973",
    name: "ANIM",
    extensions: &["anim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
