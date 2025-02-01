use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975668: FileFormat = FileFormat {
    id: 28_975_668,
    puid: "wikidata/28975668",
    name: "Alchemy 2000 Molecule format",
    extensions: &["al2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
