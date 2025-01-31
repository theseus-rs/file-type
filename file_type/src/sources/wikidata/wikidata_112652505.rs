use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112652505: FileFormat = FileFormat {
    id: 112_652_505,
    puid: "wikidata/112652505",
    name: "Astound Media Library format",
    extensions: &["mml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
