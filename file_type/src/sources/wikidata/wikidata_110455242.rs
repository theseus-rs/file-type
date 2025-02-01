use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110455242: FileFormat = FileFormat {
    id: 110_455_242,
    puid: "wikidata/110455242",
    name: "Septentrio Binary Format",
    extensions: &["sbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
