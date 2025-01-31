use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119519641: FileFormat = FileFormat {
    id: 119_519_641,
    puid: "wikidata/119519641",
    name: "Windows Spelling Dictionary Identifier",
    extensions: &["dub"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
