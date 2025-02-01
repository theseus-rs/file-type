use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131519262: FileFormat = FileFormat {
    id: 131_519_262,
    puid: "wikidata/131519262",
    name: "Stimulate Signal Parameters",
    extensions: &["spr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
