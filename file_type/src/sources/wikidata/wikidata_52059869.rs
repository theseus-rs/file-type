use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52059869: FileFormat = FileFormat {
    id: 52_059_869,
    puid: "wikidata/52059869",
    name: "Micrografx Designer format, version 3.1",
    extensions: &["drw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
