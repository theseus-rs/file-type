use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263309: FileFormat = FileFormat {
    id: 111_263_309,
    puid: "wikidata/111263309",
    name: "Sound Designer I file",
    extensions: &["dig", "sd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
