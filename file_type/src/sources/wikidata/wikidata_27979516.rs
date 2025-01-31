use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979516: FileFormat = FileFormat {
    id: 27_979_516,
    puid: "wikidata/27979516",
    name: "Manga Studio Page",
    extensions: &["cpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
