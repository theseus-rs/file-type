use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967214: FileFormat = FileFormat {
    id: 27_967_214,
    puid: "wikidata/27967214",
    name: "SBStudio module",
    extensions: &["pac", "son", "sou"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
