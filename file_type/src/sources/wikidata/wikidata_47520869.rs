use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47520869: FileFormat = FileFormat {
    id: 47_520_869,
    puid: "wikidata/47520869",
    name: "Serif PagePlus Publication file format, version 12",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
