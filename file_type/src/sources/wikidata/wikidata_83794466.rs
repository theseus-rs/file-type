use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83794466: FileFormat = FileFormat {
    id: 83_794_466,
    puid: "wikidata/83794466",
    name: "FileMaker Pro Database, version 12",
    extensions: &["fmp12"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
