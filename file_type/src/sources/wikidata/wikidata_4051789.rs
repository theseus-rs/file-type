use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4051789: FileFormat = FileFormat {
    id: 4_051_789,
    puid: "wikidata/4051789",
    name: "Trivial Graph Format",
    extensions: &["tgf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
