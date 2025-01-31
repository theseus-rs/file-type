use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128775109: FileFormat = FileFormat {
    id: 128_775_109,
    puid: "wikidata/128775109",
    name: "Component Pascal source code file",
    extensions: &["cp"],
    media_types: &["text/x-component-pascal"],
    internal_signatures: &[],
    related_formats: &[],
};
