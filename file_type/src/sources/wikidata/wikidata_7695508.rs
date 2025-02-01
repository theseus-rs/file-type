use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7695508: FileFormat = FileFormat {
    id: 7_695_508,
    puid: "wikidata/7695508",
    name: "Tektronix extended HEX",
    extensions: &["tek"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
