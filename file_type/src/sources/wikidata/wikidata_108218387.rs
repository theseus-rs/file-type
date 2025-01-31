use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108218387: FileFormat = FileFormat {
    id: 108_218_387,
    puid: "wikidata/108218387",
    name: "Citation File Format",
    extensions: &["cff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
