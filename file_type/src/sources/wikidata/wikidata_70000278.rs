use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_70000278: FileFormat = FileFormat {
    id: 70_000_278,
    puid: "wikidata/70000278",
    name: "FAMILYFILE",
    extensions: &["familyfile"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
