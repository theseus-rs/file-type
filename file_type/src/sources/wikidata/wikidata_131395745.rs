use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131395745: FileFormat = FileFormat {
    id: 131_395_745,
    puid: "wikidata/131395745",
    name: "VGL source code file",
    extensions: &["rpf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
