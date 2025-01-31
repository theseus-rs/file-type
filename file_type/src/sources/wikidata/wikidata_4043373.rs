use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4043373: FileFormat = FileFormat {
    id: 4_043_373,
    puid: "wikidata/4043373",
    name: "MAGMA",
    extensions: &["magma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
