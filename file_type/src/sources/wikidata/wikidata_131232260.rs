use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131232260: FileFormat = FileFormat {
    id: 131_232_260,
    puid: "wikidata/131232260",
    name: "Allotrope Simple Model",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
