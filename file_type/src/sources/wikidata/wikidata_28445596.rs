use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445596: FileFormat = FileFormat {
    id: 28_445_596,
    puid: "wikidata/28445596",
    name: "APD",
    extensions: &["apd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
