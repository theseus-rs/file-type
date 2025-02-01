use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34303668: FileFormat = FileFormat {
    id: 34_303_668,
    puid: "wikidata/34303668",
    name: "Syntactically Awesome StyleSheet",
    extensions: &["sass"],
    media_types: &["text/x-sass"],
    internal_signatures: &[],
    related_formats: &[],
};
