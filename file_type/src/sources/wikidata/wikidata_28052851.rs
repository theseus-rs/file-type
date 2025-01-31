use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28052851: FileFormat = FileFormat {
    id: 28_052_851,
    puid: "wikidata/28052851",
    name: "RePub",
    extensions: &["epub"],
    media_types: &["application/epub+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
