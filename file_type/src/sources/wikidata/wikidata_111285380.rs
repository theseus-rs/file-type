use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111285380: FileFormat = FileFormat {
    id: 111_285_380,
    puid: "wikidata/111285380",
    name: "Ensoniq EPS family disk image",
    extensions: &["gkh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
