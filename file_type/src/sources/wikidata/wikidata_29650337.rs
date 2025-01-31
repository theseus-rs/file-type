use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650337: FileFormat = FileFormat {
    id: 29_650_337,
    puid: "wikidata/29650337",
    name: "PUPA Font Format, version 2",
    extensions: &["pf2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
