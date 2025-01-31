use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966948: FileFormat = FileFormat {
    id: 27_966_948,
    puid: "wikidata/27966948",
    name: "SPC",
    extensions: &["rsn", "spc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
