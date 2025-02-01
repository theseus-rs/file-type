use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131413865: FileFormat = FileFormat {
    id: 131_413_865,
    puid: "wikidata/131413865",
    name: "Vyper file format",
    extensions: &["vy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
