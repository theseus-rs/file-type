use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110443436: FileFormat = FileFormat {
    id: 110_443_436,
    puid: "wikidata/110443436",
    name: "Bentley Microstation Hidden Line File",
    extensions: &["hln"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
