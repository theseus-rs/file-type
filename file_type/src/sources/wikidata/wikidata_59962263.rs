use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59962263: FileFormat = FileFormat {
    id: 59_962_263,
    puid: "wikidata/59962263",
    name: "ChiWriter Document",
    extensions: &["chi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
