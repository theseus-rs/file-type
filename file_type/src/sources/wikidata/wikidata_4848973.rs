use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4848973: FileFormat = FileFormat {
    id: 4_848_973,
    puid: "wikidata/4848973",
    name: "Bak file",
    extensions: &["bak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
