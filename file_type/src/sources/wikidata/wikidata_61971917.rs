use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61971917: FileFormat = FileFormat {
    id: 61_971_917,
    puid: "wikidata/61971917",
    name: "FoxPro Database, version 2x",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
