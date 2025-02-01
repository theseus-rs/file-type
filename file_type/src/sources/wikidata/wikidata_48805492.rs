use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48805492: FileFormat = FileFormat {
    id: 48_805_492,
    puid: "wikidata/48805492",
    name: "ChiWriter Document",
    extensions: &["chi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
