use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206851: FileFormat = FileFormat {
    id: 28_206_851,
    puid: "wikidata/28206851",
    name: "Secure Pick Ax file",
    extensions: &["pax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
