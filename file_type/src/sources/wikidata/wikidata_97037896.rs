use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_97037896: FileFormat = FileFormat {
    id: 97_037_896,
    puid: "wikidata/97037896",
    name: "Personal Icon",
    extensions: &["picon"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
