use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850401: FileFormat = FileFormat {
    id: 105_850_401,
    puid: "wikidata/105850401",
    name: "Poser character rigging",
    extensions: &["cr2"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
