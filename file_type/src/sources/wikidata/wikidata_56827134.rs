use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_56827134: FileFormat = FileFormat {
    id: 56_827_134,
    puid: "wikidata/56827134",
    name: "PicoTech Picologger PLW",
    extensions: &["plw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
